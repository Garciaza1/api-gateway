
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 91,
                        patch: 1,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.91.1 (ed61e7d7e 2025-11-07)".to_owned(),
                    commit_hash: Some("ed61e7d7e242494fb7057f2657300d9e77bb4fcb".to_owned()),
                    commit_date: Some("2025-11-07".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            