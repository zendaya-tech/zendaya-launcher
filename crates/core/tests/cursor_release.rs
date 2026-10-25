use artcraft_launcher_core::{Arch, Os, PackageKind, Platform, catalog::CURSOR, platform::pick_asset, release::parse_github_release};

#[test]
fn published_cursor_release_installs_from_its_windows_portable_zip() {
    let release = parse_github_release(include_str!("fixtures/zendaya-cursor-v0.1.0.json"), CURSOR.repo).unwrap();
    assert_eq!(release.tag, "v0.1.0");
    let windows = Platform { os: Os::Windows, arch: Arch::X64 };
    let stems: Vec<_> = CURSOR.stems().collect();
    let (asset, kind) = pick_asset(&release, &stems, windows, windows.default_kinds()).unwrap();
    assert_eq!(kind, PackageKind::PortableZip);
    assert_eq!(asset.name, "zendaya-cursor-0.1.0-windows-x64-portable.zip");
    assert!(asset.url.starts_with("https://github.com/zendaya-tech/zendaya-cursor/releases/download/v0.1.0/"));
    assert!(release.sums_asset().is_some());
    let mac = Platform { os: Os::MacOs, arch: Arch::X64 };
    assert!(pick_asset(&release, &stems, mac, mac.default_kinds()).is_none());
}
