macro_rules! vendors {
    (
        $(
            {
                name: $name:literal,
                confirm: $confirm:literal,
                install: $install:literal,
                remove: $remove:literal,
                upgrade: $upgrade:literal,
                search: $search:literal,
                info: $show:literal,
                update_index: $update_index:literal,
                upgrade_all: $upgrade_all:literal,
                list_installed: $list_installed:literal,
            },
        )+
    ) => {
        pub fn init_vendor(name: &str) -> Result<$crate::Vendor, $crate::UptError> {
            use $crate::action::must_from_str;
            match name {
                $(
                    $name => {
                        let vendor = $crate::Vendor {
                            name: $name.to_string(),
                            confirm: $confirm.to_string(),
                            install: must_from_str($install, $name, "install"),
                            remove: must_from_str($remove, $name, "remove"),
                            upgrade: must_from_str($upgrade, $name, "upgrade"),
                            search: must_from_str($search, $name, "search"),
                            info: must_from_str($show, $name, "show"),
                            update_index: must_from_str($update_index, $name, "update_index"),
                            upgrade_all: must_from_str($upgrade_all, $name, "upgrade_all"),
                            list_installed: must_from_str($list_installed, $name, "list_installed"),
                        };
                        Ok(vendor)
                    },
                )+
                _ => Err(UptError::NoVendor(name.to_string()))
            }
        }

        pub(crate) fn which_cmd(name: &str) -> Option<&'static str> {
            match name {
                $(
                    $name => {
                        let (cmd, _) = $install.split_once(' ')?;
                        Some(cmd)
                    }
                )+
                _ => None
            }
        }

        #[allow(unused)]
        pub(crate) fn support_tools() -> Vec<&'static str> {
            vec![$( $name,)+]
        }
    }
}

macro_rules! os_vendors {
    ($($os:literal => $($tool:literal),+);+$(;)?) => {
        pub fn detect_vendor(os: &str) -> std::result::Result<$crate::Vendor, $crate::UptError> {
            let pairs: Vec<(&str, &str)> = match os {
                $(
                    $os => vec![$($tool),+].into_iter().filter_map(|tool| which_cmd(tool).map(|bin_name| (tool, bin_name))).collect(),
                )+
                "windows/msys2" => vec![("pacman","pacman")],
                _ => ["apt", "dnf", "pacman"].into_iter().map(|tool| (tool, tool)).collect(),
            };
            match $crate::utils::find_tool(&pairs) {
                Some(tool) => $crate::vendor::init_vendor(&tool),
                None => Err(UptError::NoDetectVendor),
            }
        }
    };
}
