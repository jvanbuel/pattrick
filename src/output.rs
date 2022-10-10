use std::{
    error::Error,
    fs::File,
    io::{LineWriter, Write},
};

use netrc::Netrc;
use tabled::{Style, Table};

use pattrick::PatToken;

pub fn print_as_table(pat_tokens: Vec<PatToken>) {
    let mut table = Table::new(pat_tokens);
    table.with(Style::modern());
    println!("{:#^10}", table.to_string());
}

pub fn update_netrc(netrc: &mut Netrc, host: String, machine: netrc::Machine) -> Option<()> {
    netrc
        .hosts
        .iter_mut()
        .find(|h| h.0 == host)
        .map(|h| h.1 = machine)
}

pub fn write_to_netrc(pat_token: PatToken) -> Result<(), Box<dyn Error>> {
    let netrc_path = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".netrc");
    let netrc_contents = std::fs::read_to_string(&netrc_path)?;
    let mut netrc = netrc::Netrc::parse(netrc_contents.as_bytes()).unwrap();

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

    let file = File::create(&netrc_path)?;
    let mut file = LineWriter::new(file);
    for hosts in netrc.hosts {
        let host = hosts.0;
        let machine = hosts.1;
        file.write_all(
            format!(
                "machine {machine}\nlogin {login}\npassword {password}\n
                        ",
                machine = host,
                login = machine.login,
                password = machine.password.unwrap_or_default(),
            )
            .as_bytes(),
        )?;
    }
    Ok(())
}
