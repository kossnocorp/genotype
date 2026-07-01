use crate::prelude::internal::*;

impl GtpFormatter {
    pub fn cmd(&self) -> GtpFormatterCmd {
        match self {
            GtpFormatter::Shell(GtpFormatterShell { cmd, args }) => GtpFormatterCmd {
                cmd: cmd.clone(),
                args: args.clone().unwrap_or_default(),
            },

            GtpFormatter::Executor(exec) => self.apply_executor_args(
                self.executor_cmd_base(&exec.kind),
                exec.cmd.clone(),
                exec.args.clone(),
            ),

            GtpFormatter::PresetOxfmt(GtpFormatterPresetOxfmt { via, args }) => {
                let mut cmd = GtpFormatterCmd {
                    cmd: "oxfmt".to_owned(),
                    args: vec!["--no-error-on-unmatched-pattern".to_owned()],
                };
                if let Some(args) = args {
                    cmd.args.extend(args.clone());
                }

                match via {
                    Some(via) => self.apply_executor_cmd(self.executor_cmd_base_node(via), cmd),

                    None => cmd,
                }
            }

            GtpFormatter::PresetPrettier(GtpFormatterPresetPrettier { via, args }) => {
                let mut cmd = GtpFormatterCmd {
                    cmd: "prettier".to_owned(),
                    args: vec!["--write".to_owned(), ".".to_owned()],
                };
                if let Some(args) = args {
                    cmd.args.extend(args.clone());
                }

                match via {
                    Some(via) => self.apply_executor_cmd(self.executor_cmd_base_node(via), cmd),

                    None => cmd,
                }
            }

            GtpFormatter::PresetRuff(GtpFormatterPresetRuff { via, args }) => {
                let mut cmd = GtpFormatterCmd {
                    cmd: "ruff".to_owned(),
                    args: vec!["format".to_owned(), ".".to_owned()],
                };
                if let Some(args) = args {
                    cmd.args.extend(args.clone());
                }

                match via {
                    Some(via) => self.apply_executor_cmd(self.executor_cmd_base_python(via), cmd),

                    None => cmd,
                }
            }
        }
    }

    fn apply_executor_cmd(
        &self,
        base_cmd: GtpFormatterCmd,
        cmd: GtpFormatterCmd,
    ) -> GtpFormatterCmd {
        let GtpFormatterCmd { cmd, args } = cmd;
        self.apply_executor_args(base_cmd, cmd, Some(args))
    }

    fn apply_executor_args(
        &self,
        base_cmd: GtpFormatterCmd,
        cmd: String,
        args: Option<Vec<String>>,
    ) -> GtpFormatterCmd {
        let mut exec_cmd = base_cmd;
        exec_cmd.args.push(cmd);
        if let Some(exec_args) = args {
            exec_cmd.args.extend(exec_args);
        }
        exec_cmd
    }

    fn executor_cmd_base(&self, kind: &GtpFormatterExecutorKind) -> GtpFormatterCmd {
        match kind {
            GtpFormatterExecutorKind::Cargo => GtpFormatterCmd {
                cmd: "cargo".to_owned(),
                args: vec![],
            },

            GtpFormatterExecutorKind::Node(kind) => self.executor_cmd_base_node(kind),

            GtpFormatterExecutorKind::Python(kind) => self.executor_cmd_base_python(kind),
        }
    }

    fn executor_cmd_base_node(&self, kind: &GtpFormatterExecutorKindNode) -> GtpFormatterCmd {
        let (cmd, args) = match kind {
            GtpFormatterExecutorKindNode::Npm => ("npm", vec!["exec".to_owned()]),

            GtpFormatterExecutorKindNode::Npx => ("npx", vec![]),

            GtpFormatterExecutorKindNode::Pnpm => ("pnpm", vec!["exec".to_owned()]),

            GtpFormatterExecutorKindNode::Pnx => ("pnx", vec![]),

            GtpFormatterExecutorKindNode::Bun => ("bun", vec!["exec".to_owned()]),

            GtpFormatterExecutorKindNode::Bunx => ("bunx", vec![]),
        };

        GtpFormatterCmd {
            cmd: cmd.into(),
            args,
        }
    }

    fn executor_cmd_base_python(&self, kind: &GtpFormatterExecutorKindPython) -> GtpFormatterCmd {
        match kind {
            GtpFormatterExecutorKindPython::Uv => GtpFormatterCmd {
                cmd: "uv".to_owned(),
                args: vec!["run".to_owned()],
            },

            GtpFormatterExecutorKindPython::Poetry => GtpFormatterCmd {
                cmd: "poetry".to_owned(),
                args: vec!["run".to_owned()],
            },

            GtpFormatterExecutorKindPython::Pipx => GtpFormatterCmd {
                cmd: "pipx".to_owned(),
                args: vec!["run".to_owned()],
            },
        }
    }
}

impl Display for GtpFormatterCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chunks = vec![self.cmd.clone()];
        chunks.extend(self.args.clone());
        write!(f, "{}", chunks.join(" "))
    }
}
