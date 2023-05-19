use std::collections::HashSet;
use std::fmt::Display;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// IDs of supported linux distros
pub enum DistroId {
    /// Arch Linux
    /// 
    /// ```bash
    /// ID=arch
    /// ```
    Arch,

    /// Debian
    /// 
    /// ```bash
    /// ID=debian
    /// ```
    Debian,

    /// Ubuntu
    /// 
    /// ```bash
    /// ID=ubuntu
    /// ```
    Ubuntu,

    /// Linux Mint
    /// 
    /// ```bash
    /// ID=linuxmint
    /// ```
    Mint,

    /// Red Hat Enterprise Linux (RHEL)
    /// 
    /// ```bash
    /// ID=rhel
    /// ```
    RHEL,

    /// Fedora (workstation, silverblue)
    /// 
    /// ```bash
    /// ID=fedora
    /// ```
    Fedora,

    /// OpenSUSE (leap, tumbleweed)
    /// 
    /// ```bash
    /// ID=suse
    /// ID=opensuse
    /// ID=opensuse-tumbleweed
    /// ```
    OpenSUSE,

    /// Gentoo
    /// 
    /// ```bash
    /// ID=gentoo
    /// ```
    Gentoo,

    /// ```bash
    /// ID=nixos
    /// ```
    NixOS,

    /// Nothing from above
    Other(String)
}

impl DistroId {
    /// List distro ids similar to the current one.
    /// Always include current distro itself
    pub fn list_similar(&self) -> Vec<Self> {
        match self {
            Self::Arch => vec![
                Self::Arch
            ],

            Self::Debian => vec![
                Self::Debian,
                Self::Ubuntu,
                Self::Mint
            ],

            Self::Ubuntu => vec![
                Self::Ubuntu,
                Self::Debian,
                Self::Mint
            ],

            Self::Mint => vec![
                Self::Mint,
                Self::Debian,
                Self::Ubuntu
            ],

            Self::RHEL => vec![
                Self::RHEL,
                Self::Fedora,
                Self::OpenSUSE
            ],

            Self::Fedora => vec![
                Self::Fedora,
                Self::RHEL,
                Self::OpenSUSE
            ],

            Self::OpenSUSE => vec![
                Self::OpenSUSE,
                Self::Fedora,
                Self::RHEL
            ],

            Self::Gentoo => vec![
                Self::Gentoo
            ],

            Self::NixOS => vec![
                Self::NixOS
            ],

            Self::Other(id) => vec![
                Self::Other(id.clone())
            ]
        }
    }

    #[inline]
    /// Compare given distro id with the current one
    pub fn is_similar<T: Into<Self>>(&self, other: T) -> bool {
        self.list_similar().contains(&other.into())
    }
}

impl<T> From<T> for DistroId where T: AsRef<str> {
    fn from(str: T) -> Self {
        match str.as_ref() {
            "arch"   => Self::Arch,
            "debian" => Self::Debian,
            "ubuntu" => Self::Ubuntu,

            "mint"      => Self::Mint,
            "linuxmint" => Self::Mint,

            "rhel"   => Self::RHEL,
            "fedora" => Self::Fedora,

            "suse"                => Self::OpenSUSE,
            "opensuse"            => Self::OpenSUSE,
            "opensuse_tumbleweed" => Self::OpenSUSE,

            "gentoo" => Self::Gentoo,
            "nixos"  => Self::NixOS,

            id => Self::Other(id.to_string())
        }
    }
}

impl Display for DistroId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arch     => write!(f, "arch"),
            Self::Debian   => write!(f, "debian"),
            Self::Ubuntu   => write!(f, "ubuntu"),
            Self::Mint     => write!(f, "linuxmint"),
            Self::RHEL     => write!(f, "rhel"),
            Self::Fedora   => write!(f, "fedora"),
            Self::OpenSUSE => write!(f, "opensuse"),
            Self::Gentoo   => write!(f, "gentoo"),
            Self::NixOS    => write!(f, "nixos"),

            Self::Other(id) => write!(f, "{id}")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Distro {
    name: String,
    id: DistroId,
    similar_ids: HashSet<DistroId>
}

impl Distro {
    #[inline]
    /// Identify current linux distro using `/etc/os-release` file
    pub fn current() -> Option<Self> {
        identify()
    }

    #[inline]
    /// Get current distro name (`NAME` entry)
    pub fn name(&self) -> &str {
        &self.name
    }

    #[inline]
    /// Get current distro id (`ID` entry)
    pub fn id(&self) -> &DistroId {
        &self.id
    }

    #[inline]
    /// Get list of similar distros (`ID_LIKE` entry)
    /// 
    /// ```
    /// if let Some(distro) = whatadistro::identify() {
    ///     println!("Your distro: {} ({})", distro.name(), distro.id());
    ///     println!("Similar distros: {:?}", distro.id().list_similar());
    /// }
    /// ```
    pub fn similar_ids(&self) -> &HashSet<DistroId> {
        &self.similar_ids
    }

    #[inline]
    /// Compare current distro with some another
    /// 
    /// ```
    /// let status = whatadistro::identify()
    ///     .map(|distro| distro.is_similar("arch")) // whatadistro::Distro::Arch can be used as well
    ///     .unwrap_or(false);
    /// 
    /// println!("Is current system arch-based: {:?}", status);
    /// ```
    pub fn is_similar<T: Into<DistroId>>(&self, other: T) -> bool {
        let other = other.into();

        self.similar_ids.contains(&other) || self.id.is_similar(other)
    }
}

/// Identify current linux distro using `/etc/os-release` file
/// 
/// ```
/// let distro = whatadistro::identify()
///     .expect("Failed to parse os-release file");
/// 
/// println!("Your distro name is {}", distro.name());
/// ```
pub fn identify() -> Option<Distro> {
    let mut id: Option<DistroId> = None;
    let mut name: Option<String> = None;
    let mut similar_ids: Option<HashSet<DistroId>> = None;

    if let Ok(release) = std::fs::read_to_string("/etc/os-release") {
        for line in release.lines() {
            if let Some(distro_id) = line.strip_prefix("ID=") {
                id = Some(distro_id.into());
            }

            else if let Some(distro_name) = line.strip_prefix("NAME=") {
                name = Some(distro_name.to_string());
            }

            else if let Some(ids) = line.strip_prefix("ID_LIKE=") {
                similar_ids = Some(ids.split_whitespace().map(|id| id.into()).collect());
            }
        }

        let Some(id) = id else {
            return None;
        };

        // TODO: maybe I can use here something like id.name() ?
        let Some(name) = name else {
            return None;
        };

        Some(Distro {
            id,
            name,
            similar_ids: similar_ids.unwrap_or_default()
        })
    }

    else {
        None
    }
}
