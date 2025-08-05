use vergen_git2::{CargoBuilder, Emitter, Git2Builder, RustcBuilder};

fn main() -> anyhow::Result<()> {
    let git2 = Git2Builder::default().sha(true).build()?;

    let cargo = CargoBuilder::default()
        .debug(true)
        .target_triple(true)
        .features(true)
        .build()?;

    let rustc = RustcBuilder::default().semver(true).build()?;

    Emitter::default()
        .add_instructions(&git2)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .fail_on_error()
        .emit()
}
