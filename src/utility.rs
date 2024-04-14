use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};

pub(crate) fn choose_str(strs: &[impl AsRef<str>]) -> Result<String> {
    let mut rng = thread_rng();

    Ok(strs
        .choose(&mut rng)
        .ok_or(anyhow!("strs argument is empty"))?
        .as_ref()
        .to_string())
}
