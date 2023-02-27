# feenk-signer

A utility to sign applications. It presently supports signing Mac and Android apps.

## Sign Mac .app:

```bash
./feenk-signer mac MyApp.app --certificate certificate.p12 --password "pass"
```

Integrates with Jenkins secrets by accepting `CERT` and `CERT_PASSWORD` env.vars. instead of plain text options:

```bash
export CERT=path/to/cert.p12
export CERT_PASSWORD="pass"

./feenk-signer mac MyApp.app
```

See `./feenk-signer mac --help` for more options.

## Sign Android .apk:

```bash
./feenk-signer android MyApp.apk --keystore release.keystore --password "pass"
```

Integrates with Jenkins secrets  and `cargo-apk` by accepting
`CARGO_APK_RELEASE_KEYSTORE`and `CARGO_APK_RELEASE_KEYSTORE_PASSWORD` env.vars. instead of plain text options:

```bash
export CARGO_APK_RELEASE_KEYSTORE=path/to/release.keystore
export CARGO_APK_RELEASE_KEYSTORE_PASSWORD="pass"

./feenk-signer android MyApp.apk
```

See `./feenk-signer android --help` for more options.

