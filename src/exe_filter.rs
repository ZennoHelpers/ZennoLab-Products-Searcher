pub(crate) fn check_exe_names(name: &str, ver: &str, lang: &str) -> Result<&'static [&'static str], String> {
    Ok(if name.contains("ZennoPoster") && name.contains("V7") {
        &["ProjectMaker", "ZennoPoster"]
    } else if name.contains("ZennoProjectMaker") {
        &["ProjectMaker", "ProjectMakerZD"]
    } else if name.contains("ZennoDroid") {
        &["ProjectMakerZD", "ZennoDroid"]
    } else if name.contains("ZennoBox") && name.contains("V7") {
        &["ZennoBox"]
    } else if name.contains("ProxyChecker") {
        &["ProxyChecker"]
    } else if name.contains("CapMonster") {
        &["CapMonster", "CapMonsterMCS", "LicenseHelper"]
    } else {
        return Err(format!("Unsupported product: '{} {} {}'", name, ver, lang));
    })
}