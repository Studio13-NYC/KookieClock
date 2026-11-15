# Release KookieClock for Mac on GitHub

Friends get one `KookieClock-macos-universal.zip` for Apple Silicon and Intel. GitHub's hosted Macs build, sign, and notarize it. Apple notarization checks a direct download for Gatekeeper and does not create an App Store listing. [Apple's direct distribution guidance](https://developer.apple.com/macos/distribution/)

## Apple account setup

1. Use an active Apple Developer Program membership with two-factor authentication. If you need to enroll, start on Apple's website. Apple lists US membership at $99 per year and handles identity verification, agreements, and payment. The account can be individual or an eligible legal organization. Use the Account Holder to request the signing certificate. [Apple enrollment](https://developer.apple.com/programs/enroll/)
2. In the Apple developer account, note the **Team ID**. Create a **Developer ID Application** certificate and select the **G2** intermediary. The portal asks for a certificate signing request (CSR). The Windows commands below make it locally. [Apple certificate instructions](https://developer.apple.com/help/account/certificates/create-developer-id-certificates/) · [Apple's G2 guidance](https://developer.apple.com/news/?id=w4atic4c)
3. In the Apple Account's **Sign-In and Security** settings, create an **app-specific password** for KookieClock notarization. Use the Apple Account email that owns that password and has access to the Developer Program team. [Apple's password instructions](https://support.apple.com/en-us/102654)

## Prepare the certificate on Windows

Run this in PowerShell with OpenSSL available. Choose an encrypted private-key passphrase when OpenSSL prompts. Keep the files outside the repository, for example in a private user directory:

```powershell
$signingFolder = Join-Path $env:LOCALAPPDATA 'KookieClockSigning'
New-Item -ItemType Directory -Path $signingFolder -Force | Out-Null
Set-Location -LiteralPath $signingFolder
openssl req -new -newkey rsa:2048 -keyout KookieClock-signing.key -out KookieClock.certSigningRequest
openssl req -in KookieClock.certSigningRequest -noout -verify
```

Enter a common name, such as `KookieClock Developer ID`, when prompted. Upload only `KookieClock.certSigningRequest` in Apple's Developer ID Application form. Save the issued certificate as `developer-id.cer`. Download **Developer ID - G2** from [Apple PKI](https://www.apple.com/certificateauthority/) as `developer-id-g2.cer`. Use the same directory to export the certificate, chain, and private key:

```powershell
openssl x509 -inform DER -in developer-id.cer -out developer-id.pem
openssl x509 -inform DER -in developer-id-g2.cer -out developer-id-g2.pem
openssl pkcs12 -export -inkey KookieClock-signing.key -in developer-id.pem -certfile developer-id-g2.pem -out KookieClock-signing.p12 -keypbe PBE-SHA1-3DES -certpbe PBE-SHA1-3DES -macalg sha1
```

Use a strong unique export password. The PKCS#12 options make the file compatible with the macOS keychain import. Keep the encrypted key and its passphrase in your own backup. Keep `.key` and `.p12` files and all passwords out of chat and Git.

## Store the GitHub credentials

In **Settings → Environments**, create `macos-release` and restrict deployments to `main`. Add these environment secrets. GitHub restricts the signing job to `main` in the workflow as well.

| Secret | Value |
| --- | --- |
| `MACOS_CERTIFICATE_P12` | Base64 of `KookieClock-signing.p12` |
| `MACOS_CERTIFICATE_PASSWORD` | PKCS#12 export password |
| `APPLE_NOTARY_ID` | Apple Account email for notarization |
| `APPLE_TEAM_ID` | Apple Developer Team ID |
| `APPLE_NOTARY_PASSWORD` | Apple app-specific password |

An authenticated GitHub CLI can transfer the certificate directly without printing the Base64 string:

```powershell
[Convert]::ToBase64String([IO.File]::ReadAllBytes((Resolve-Path 'KookieClock-signing.p12').Path)) |
    gh secret set MACOS_CERTIFICATE_P12 --env macos-release --repo Studio13-NYC/KookieClock
```

Enter the other four values through the GitHub environment page. [GitHub's certificate and secret guide](https://docs.github.com/en/actions/how-tos/deploy/deploy-to-third-party-platforms/sign-xcode-applications)

## Build the signed ZIP and publish it

1. Wait for **macOS build and test** to pass for the current `main` commit. Run [Sign macOS release package](https://github.com/Studio13-NYC/KookieClock/actions/workflows/macos-release.yml) on `main`. It uses the universal ZIP from that exact commit. It applies hardened runtime and a secure timestamp, waits for Apple to accept notarization, staples and validates the ticket, checks Gatekeeper, and runs the signed app.
2. Download the `KookieClock-macos-signed` artifact. It contains the final ZIP and `SHA256SUMS.txt`. Have a friend install the ZIP by extracting it and moving `KookieClock.app` to Applications. To test Launch at Login end to end, enable it, sign out, then sign back in and check for the clock. Test the shortcut with another app in front.
3. Create the Mac release in GitHub Releases with the tag `macos-v0.1.0` pointing to the tested commit. Attach the signed ZIP and checksum. Leave **Set as the latest release** unchecked so the Windows download link keeps pointing to the Windows release. Share the Mac release page with friends.

Check the signing run's notarization log if Apple returns a rejection or the job times out. The workflow only produces a distribution artifact after Apple accepts the submission, the signed ZIP verifies, and the desktop check passes. [Apple's notarization workflow](https://developer.apple.com/documentation/security/customizing-the-notarization-workflow)
