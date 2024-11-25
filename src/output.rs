use std::{
    error::Error,
    fs::OpenOptions,
    io::{ErrorKind, Read, Seek, SeekFrom, Write},
};

use netrc::Netrc;
use tabled::{
    settings::{location::ByColumnName, Remove, Style},
    Table,
};

use pattrick::PatToken;

pub fn print_as_table(pat_tokens: Vec<PatToken>, print_token: bool) {
    let mut table = Table::new(pat_tokens);
    table.with(Style::modern());
    table.with(Remove::column(ByColumnName::new("id")));
    if !print_token {
        table.with(Remove::column(ByColumnName::new("token")));
    }
    println!("{:#^10}", table.to_string());
}

pub fn write_to_dotenv(pat_token: PatToken) -> Result<(), Box<dyn Error>> {
    std::fs::write(
        ".env",
        format!("{}={}\n", pat_token.display_name, pat_token.token.unwrap()),
    )?;
    println!("✅ Successfully created .env file!");
    Ok(())
}

pub fn update_netrc(netrc: &mut Netrc, host: String, machine: netrc::Machine) {
    if !netrc.hosts.iter().any(|h| h.0 == host) {
        println!("✅ Adding host {host} to .netrc",);
        return netrc.hosts.push((host, machine));
    }
    if let Some(h) = netrc.hosts.iter_mut().find(|h| h.0 == host) {
        println!("✅ Updating host {host} in .netrc",);
        h.1 = machine
    };
}

pub fn write_to_netrc(pat_token: PatToken) -> Result<(), Box<dyn Error>> {
    let netrc_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".netrc");

    let mut netrc_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .create(true)
        .truncate(true)
        .open(netrc_path.as_path())?;

    let mut netrc_contents: String = Default::default();
    let _ = &netrc_file.read_to_string(&mut netrc_contents)?;

    let mut netrc = netrc::Netrc::parse(netrc_contents.as_bytes())
        .map_err(|_| std::io::Error::new(ErrorKind::InvalidData, "Could not parse .netrc "))?;

    update_netrc(
        &mut netrc,
        "pkgs.dev.azure.com".to_string(),
        netrc::Machine {
            login: pat_token.display_name,
            password: pat_token.token,
            account: None,
            port: None,
        },
    );

    netrc_file.set_len(0)?;
    netrc_file.seek(SeekFrom::Start(0))?;

    for hosts in netrc.hosts {
        let host = hosts.0;
        let machine = hosts.1;
        netrc_file.write_all(
            format!(
                "machine {machine}\nlogin {login}\npassword {password}\n\n",
                machine = host,
                login = machine.login,
                password = machine.password.unwrap_or_default(),
            )
            .as_bytes(),
        )?;
    }
    netrc_file.flush()?;
    Ok(())
}
