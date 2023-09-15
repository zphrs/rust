use crate::spec::{
    crt_objects, LinkArgs, LinkOutputKind, LinkerFlavor, LldFlavor, PanicStrategy, TargetOptions,
    TlsModel,
};

pub fn opts(static_only: bool) -> TargetOptions {
    let mut pre_link_args = LinkArgs::new();
    pre_link_args.insert(LinkerFlavor::Gcc, vec![]);

    TargetOptions {
        os: "twizzler".into(),
        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".into()),
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
        position_independent_executables: !static_only,
        static_position_independent_executables: !static_only,
        tls_model: if static_only { TlsModel::LocalExec } else { TlsModel::GeneralDynamic },
        crt_static_default: static_only,
        crt_static_respected: !static_only,
        crt_static_allows_dylibs: !static_only,
        dynamic_linking: !static_only,
        has_thread_local: true,
        ..Default::default()
    }
}
