use crate::spec::{
    crt_objects, LinkArgs, LinkOutputKind, LinkerFlavor, PanicStrategy, TargetOptions, TlsModel,
};

pub fn opts() -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Gcc, vec![]);

    TargetOptions {
        os: "twizzler".into(),
        linker_flavor: LinkerFlavor::Gcc,
        linker: Some("clang".into()),
        executables: true,
        pre_link_args,
        pre_link_objects: crt_objects::new(&[
            (LinkOutputKind::DynamicNoPicExe, &["crti.o", "crtbegin.o"]),
            (LinkOutputKind::DynamicPicExe, &["crti.o", "crtbeginS.o"]),
            (LinkOutputKind::StaticNoPicExe, &["crti.o", "crtbegin.o"]),
            (LinkOutputKind::StaticPicExe, &["crti.o", "crtbeginS.o"]),
        ]),
        post_link_objects: crt_objects::new(&[
            (LinkOutputKind::DynamicNoPicExe, &["crtend.o", "crtn.o"]),
            (LinkOutputKind::DynamicPicExe, &["crtendS.o", "crtn.o"]),
            (LinkOutputKind::StaticNoPicExe, &["crtend.o", "crtn.o"]),
            (LinkOutputKind::StaticPicExe, &["crtendS.o", "crtn.o"]),
        ]),
        panic_strategy: PanicStrategy::Unwind,
        position_independent_executables: false,
        static_position_independent_executables: false,
        tls_model: TlsModel::InitialExec,
        crt_static_default: false,
        crt_static_respected: true,
        crt_static_allows_dylibs: true,
        dynamic_linking: true,
        has_thread_local: true,
        ..Default::default()
    }
}
