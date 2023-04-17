use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    AbsoluteSystemPath, IntoSystem, PathError, PathValidationError, RelativeSystemPathBuf,
    RelativeUnixPathBuf,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct AnchoredSystemPathBuf(PathBuf);

impl TryFrom<&Path> for AnchoredSystemPathBuf {
    type Error = PathError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        if path.is_absolute() {
            let bad_path = path.display().to_string();
            return Err(PathValidationError::NotRelative(bad_path).into());
        }

        Ok(AnchoredSystemPathBuf(path.into_system()?))
    }
}

impl AnchoredSystemPathBuf {
    pub fn new(
        root: impl AsRef<AbsoluteSystemPath>,
        path: impl AsRef<AbsoluteSystemPath>,
    ) -> Result<Self, PathError> {
        let root = root.as_ref();
        let path = path.as_ref();
        let stripped_path = path
            .as_path()
            .strip_prefix(root.as_path())
            .map_err(|_| PathValidationError::NotParent(root.to_string(), path.to_string()))?
            .to_path_buf();

        Ok(AnchoredSystemPathBuf(stripped_path))
    }

    pub(crate) fn unchecked_new(path: impl Into<PathBuf>) -> Self {
        AnchoredSystemPathBuf(path.into())
    }

    pub fn as_path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn to_str(&self) -> Result<&str, PathError> {
        self.0
            .to_str()
            .ok_or_else(|| PathValidationError::InvalidUnicode(self.0.clone()).into())
    }

    pub fn to_unix(&self) -> Result<RelativeUnixPathBuf, PathError> {
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            let bytes = self.0.as_os_str().as_bytes();
            return RelativeUnixPathBuf::new(bytes);
        }
        #[cfg(not(unix))]
        {
            use crate::IntoUnix;
            let unix_buf = self.0.as_path().into_unix()?;
            let unix_str = unix_buf
                .to_str()
                .ok_or_else(|| PathValidationError::InvalidUnicode(unix_buf.clone()))?;
            return RelativeUnixPathBuf::new(unix_str.as_bytes());
        }
    }
}

impl From<AnchoredSystemPathBuf> for PathBuf {
    fn from(path: AnchoredSystemPathBuf) -> PathBuf {
        path.0
    }
}

impl Into<RelativeSystemPathBuf> for AnchoredSystemPathBuf {
    fn into(self) -> RelativeSystemPathBuf {
        RelativeSystemPathBuf::unchecked_new(self.0)
    }
}

impl AsRef<Path> for AnchoredSystemPathBuf {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}
