#[inline]
fn handle_quoted(arg: &str) -> std::borrow::Cow<'_, str> {
    if arg.contains(' ') || arg.contains('"') {
        let arg = format!("\"{arg}\"");
        std::borrow::Cow::Owned(arg)
    } else {
        std::borrow::Cow::Borrowed(arg)
    }
}

pub struct AutomateCCommander {
    pub args_vec: Vec<String>,
}

impl From<AutomateCCommander> for Vec<String> {
    #[inline]
    fn from(cmd: AutomateCCommander) -> Self {
        cmd.args_vec
    }
}

impl From<AutomateBuilderBase> for AutomateCCommander {
    #[inline]
    fn from(cmd: AutomateBuilderBase) -> Self {
        AutomateCCommander { args_vec: cmd.args }
    }
}

#[derive(Debug, Default)]
pub struct AutomateBuilderBase {
    args: Vec<String>,
}

impl AutomateBuilderBase {
    #[inline]
    pub fn new() -> Self {
        AutomateBuilderBase::default()
    }

    /// Unless you have a Command struct, you probably want to use the default method
    #[inline]
    pub fn with_sso(&mut self) -> &mut Self {
        self.sso()
    }

    #[inline]
    pub fn with_process(&mut self, process_name: impl AsRef<str>) -> &mut Self {
        self.run();
        self.args.push(process_name.as_ref().to_string());
        self
    }

    #[inline]
    pub fn with_resource(&mut self, resource: impl AsRef<str>) -> &mut Self {
        self.resource();
        self.args.push(resource.as_ref().to_string());
        self
    }

    #[inline]
    pub fn with_user(&mut self, user: impl AsRef<str>) -> &mut Self {
        self.user();
        let user = handle_quoted(user.as_ref());
        self.args.push(user.as_ref().to_string());
        self
    }

    #[inline]
    pub fn with_password(&mut self, password: impl AsRef<str>) -> &mut Self {
        self.password();
        let password = handle_quoted(password.as_ref());
        self.args.push(password.as_ref().to_string());
        self
    }

    #[inline]
    pub fn build(&self) -> AutomateCCommander {
        AutomateCCommander {
            args_vec: self.args.clone(),
        }
    }
}

impl From<AutomateBuilderBase> for String {
    #[inline]
    fn from(cmd: AutomateBuilderBase) -> Self {
        cmd.args.join(" ")
    }
}

impl AutomateBuilderBase {
    /// Internal function to add the /sso argument to the args.
    /// This is used for calling the `AutomateC` executable with the /sso flag, the public method is `with_sso`.
    #[inline]
    fn sso(&mut self) -> &mut Self {
        self.args.push("/sso".into());
        self
    }

    /// Internal function to add the /run argument to the args.
    /// This is used for calling the `AutomateC` executable with the /run flag, the public method is `with_process`.
    #[inline]
    fn run(&mut self) -> &mut Self {
        self.args.push("/run".into());
        self
    }

    /// Internal function to add the /resource argument to the args.
    /// This is used for calling the `AutomateC` executable with the /resource flag, the public method is `with_resource`.
    #[inline]
    fn resource(&mut self) -> &mut Self {
        self.args.push("/resource".into());
        self
    }

    /// Internal function to add the /user argument to the args.
    /// This is used for calling the `AutomateC` executable with the /user flag, the public method is `with_user`.
    #[inline]
    fn user(&mut self) -> &mut Self {
        self.args.push("/user".into());
        self
    }

    /// Internal function to add the /password argument to the args.
    /// This is used for calling the `AutomateC` executable with the /password flag, the public method is `with_password`.
    #[inline]
    fn password(&mut self) -> &mut Self {
        self.args.push("/password".into());
        self
    }
}
