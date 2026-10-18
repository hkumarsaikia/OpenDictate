%global debug_package %{nil}

Name:           opendictate
Version:        2.0.3
Release:        1%{?dist}
Summary:        Voice dictation and AI speech-to-text application for Linux

License:        MIT
URL:            https://github.com/hkumarsaikia/OpenDictate
Source0:        https://github.com/hkumarsaikia/OpenDictate/archive/refs/tags/v%{version}.tar.gz#/opendictate-%{version}.tar.gz
Source1:        opendictate
Source2:        opendictate-avx2

BuildRoot:      %{_tmppath}/%{name}-%{version}-%{release}-root-%(%{__id_u} -n)
ExclusiveArch:  x86_64

Requires:       gtk4
Requires:       libadwaita
Requires:       alsa-lib
Requires:       glibc >= 2.38

%description
OpenDictate is a native Linux voice dictation and AI speech-to-text
desktop application built with Rust, GTK4, and Libadwaita. It combines
private offline Whisper transcription with multi-provider Cloud AI models
to turn spoken voice into clean, polished text.

%prep
%autosetup

%build
# Native binaries pre-compiled with full vector optimization

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}%{_bindir}
mkdir -p %{buildroot}%{_datadir}/applications
mkdir -p %{buildroot}%{_datadir}/metainfo
mkdir -p %{buildroot}%{_datadir}/icons/hicolor/scalable/apps
mkdir -p %{buildroot}%{_docdir}/opendictate
mkdir -p %{buildroot}%{_licensedir}/opendictate

install -m 755 %{SOURCE1} %{buildroot}%{_bindir}/opendictate
install -m 755 %{SOURCE2} %{buildroot}%{_bindir}/opendictate-avx2

install -m 644 data/io.github.hkumarsaikia.OpenDictate.desktop %{buildroot}%{_datadir}/applications/io.github.hkumarsaikia.OpenDictate.desktop
install -m 644 data/io.github.hkumarsaikia.OpenDictate.metainfo.xml %{buildroot}%{_datadir}/metainfo/io.github.hkumarsaikia.OpenDictate.metainfo.xml

install -m 644 src/ui/assets/brand/opendictate.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/opendictate.svg
install -m 644 src/ui/assets/brand/opendictate.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate.svg
install -m 644 src/ui/assets/brand/opendictate-dark.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/opendictate-dark.svg
install -m 644 src/ui/assets/brand/opendictate-light.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/opendictate-light.svg
install -m 644 src/ui/assets/brand/opendictate-dark.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate-dark.svg
install -m 644 src/ui/assets/brand/opendictate-light.svg %{buildroot}%{_datadir}/icons/hicolor/scalable/apps/io.github.hkumarsaikia.OpenDictate-light.svg

for size in 16x16 24x24 32x32 48x48 64x64 128x128 256x256 512x512; do
    mkdir -p %{buildroot}%{_datadir}/icons/hicolor/${size}/apps
    install -m 644 src/ui/assets/brand/png/${size}.png %{buildroot}%{_datadir}/icons/hicolor/${size}/apps/opendictate.png
    install -m 644 src/ui/assets/brand/png/${size}.png %{buildroot}%{_datadir}/icons/hicolor/${size}/apps/io.github.hkumarsaikia.OpenDictate.png
done

install -m 644 LICENSE %{buildroot}%{_licensedir}/opendictate/LICENSE
install -m 644 licenses/THIRD_PARTY_LICENSES.md %{buildroot}%{_docdir}/opendictate/THIRD_PARTY_LICENSES.md
install -m 644 README.md %{buildroot}%{_docdir}/opendictate/README.md

%check
%{buildroot}%{_bindir}/opendictate --version
%{buildroot}%{_bindir}/opendictate-avx2 --version

%post
/bin/touch --no-create %{_datadir}/icons/hicolor &>/dev/null || :
if [ -x /usr/bin/gtk-update-icon-cache ]; then
    /usr/bin/gtk-update-icon-cache %{_datadir}/icons/hicolor &>/dev/null || :
fi
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database &>/dev/null || :
fi

%postun
/bin/touch --no-create %{_datadir}/icons/hicolor &>/dev/null || :
if [ -x /usr/bin/gtk-update-icon-cache ]; then
    /usr/bin/gtk-update-icon-cache %{_datadir}/icons/hicolor &>/dev/null || :
fi
if [ -x /usr/bin/update-desktop-database ]; then
    /usr/bin/update-desktop-database &>/dev/null || :
fi

%files
%license %{_licensedir}/opendictate/LICENSE
%doc %{_docdir}/opendictate/README.md
%doc %{_docdir}/opendictate/THIRD_PARTY_LICENSES.md
%{_bindir}/opendictate
%{_bindir}/opendictate-avx2
%{_datadir}/applications/io.github.hkumarsaikia.OpenDictate.desktop
%{_datadir}/metainfo/io.github.hkumarsaikia.OpenDictate.metainfo.xml
%{_datadir}/icons/hicolor/*/apps/*

%changelog
* Sun Oct 18 2026 H. K. Saikia <hkumarsaikia@users.noreply.github.com> - 2.0.3-1
- OpenDictate v2.0.3 release
- Dual-binary x86_64 launcher and runtime AVX2 auto-dispatch
