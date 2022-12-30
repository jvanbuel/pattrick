const completion: Fig.Spec = {
  name: "pattrick",
  description: "CLI to manage Azure DevOps Personal Access Tokens (PAT)",
  subcommands: [
    {
      name: "create",
      description: "Create a new PAT token",
      options: [
        {
          name: ["-l", "--lifetime"],
          description: "Number of seconds the token should be valid for",
          isRepeatable: true,
          args: {
            name: "lifetime",
            isOptional: true,
          },
        },
        {
          name: ["-s", "--scope"],
          description: "Scope of the token",
          isRepeatable: true,
          args: {
            name: "scope",
            isOptional: true,
            suggestions: [
              "full-access",
              "agent-pools",
              "agent-pools-manage",
              "build",
              "build-execute",
              "code",
              "code-write",
              "code-manage",
              "code-full",
              "dashboards",
              "dashboards-manage",
              "drop-write",
              "extension",
              "extension-manage",
              "governance",
              "graph",
              "graph-manage",
              "notification",
              "notification-diagnostics",
              "packaging",
              "packaging-manage",
              "packaging-write",
              "profile",
              "project",
              "project-manage",
              "release",
              "release-execute",
              "security",
              "security-manage",
              "test",
              "test-write",
              "work",
              "work-write",
              "wiki",
              "wiki-write",
            ],
          },
        },
        {
          name: ["-o", "--out"],
          description: "Output format of the token: print to stdout, write to dotenv or netrc",
          isRepeatable: true,
          args: {
            name: "out",
            isOptional: true,
            suggestions: [
              "std-out",
              "dot-netrc",
              "dot-env",
            ],
          },
        },
        {
          name: ["-v", "--verbose"],
          description: "More output per occurrence",
          isRepeatable: true,
        },
        {
          name: ["-q", "--quiet"],
          description: "Less output per occurrence",
          exclusiveOn: [
            "-v",
            "--verbose",
          ],
          isRepeatable: true,
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
      args: {
        name: "name",
        isOptional: true,
      },
    },
    {
      name: "list",
      description: "List all PAT tokens",
      options: [
        {
          name: ["-a", "--all"],
          description: "Show all tokens, including expired ones",
        },
        {
          name: ["-v", "--verbose"],
          description: "More output per occurrence",
          isRepeatable: true,
        },
        {
          name: ["-q", "--quiet"],
          description: "Less output per occurrence",
          exclusiveOn: [
            "-v",
            "--verbose",
          ],
          isRepeatable: true,
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "get",
      description: "Show contents of a PAT token",
      options: [
        {
          name: ["-o", "--out"],
          isRepeatable: true,
          args: {
            name: "out",
            isOptional: true,
            suggestions: [
              "std-out",
              "dot-netrc",
              "dot-env",
            ],
          },
        },
        {
          name: ["-v", "--verbose"],
          description: "More output per occurrence",
          isRepeatable: true,
        },
        {
          name: ["-q", "--quiet"],
          description: "Less output per occurrence",
          exclusiveOn: [
            "-v",
            "--verbose",
          ],
          isRepeatable: true,
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
      args: {
        name: "id",
      },
    },
    {
      name: "delete",
      description: "Delete a PAT token",
      options: [
        {
          name: ["-n", "--name"],
          description: "Name of the token to delete",
          isRepeatable: true,
          args: {
            name: "name",
            isOptional: true,
          },
        },
        {
          name: ["-i", "--id"],
          description: "Id of the token to delete",
          isRepeatable: true,
          args: {
            name: "id",
            isOptional: true,
          },
        },
        {
          name: ["-a", "--all"],
          description: "Delete all tokens, including expired ones",
        },
        {
          name: ["-v", "--verbose"],
          description: "More output per occurrence",
          isRepeatable: true,
        },
        {
          name: ["-q", "--quiet"],
          description: "Less output per occurrence",
          exclusiveOn: [
            "-v",
            "--verbose",
          ],
          isRepeatable: true,
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "update",
      description: "Update Pattrick",
      options: [
        {
          name: ["-v", "--verbose"],
          description: "More output per occurrence",
          isRepeatable: true,
        },
        {
          name: ["-q", "--quiet"],
          description: "Less output per occurrence",
          exclusiveOn: [
            "-v",
            "--verbose",
          ],
          isRepeatable: true,
        },
        {
          name: ["-h", "--help"],
          description: "Print help information",
        },
        {
          name: ["-V", "--version"],
          description: "Print version information",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "create",
          description: "Create a new PAT token",
        },
        {
          name: "list",
          description: "List all PAT tokens",
        },
        {
          name: "get",
          description: "Show contents of a PAT token",
        },
        {
          name: "delete",
          description: "Delete a PAT token",
        },
        {
          name: "update",
          description: "Update Pattrick",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: ["-v", "--verbose"],
      description: "More output per occurrence",
      isRepeatable: true,
    },
    {
      name: ["-q", "--quiet"],
      description: "Less output per occurrence",
      exclusiveOn: [
        "-v",
        "--verbose",
      ],
      isRepeatable: true,
    },
    {
      name: ["-h", "--help"],
      description: "Print help information",
    },
    {
      name: ["-V", "--version"],
      description: "Print version information",
    },
  ],
};

export default completion;
