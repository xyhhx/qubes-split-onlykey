#!/bin/bash
set -xeuo pipefail

TARGET="${1}"
SPEC_TEMPLATE="${TARGET}.spec.in"
VERSION="$(cat version)"
RPMBUILDDIR="${RPMBUILDDIR:-$(mktemp -d "/tmp/rpmbuild_XXXXX")}"
WORKDIR="$(pwd)"

# Set up rpmbuild root
mkdir -p "${RPMBUILDDIR}"/{BUILD,BUILDROOT,RPMS,SOURCES,SPECS,SRPMS,tmp}

# Prepare spec files
sed \
  "s/@VERSION@/${VERSION}/g" \
  "${WORKDIR}/rpm_spec/${SPEC_TEMPLATE}" \
  > "${RPMBUILDDIR}/SPECS/${TARGET}.spec"

# Copy in required source files
rsync -azP \
  --exclude .* --exclude rpm_spec/ "${WORKDIR}/" \
  "${RPMBUILDDIR}/tmp/${TARGET}-${VERSION}" 

# Create source0 tarball
tar \
  czf "${RPMBUILDDIR}/SOURCES/${TARGET}-${VERSION}.tar.gz" \
  -C "${RPMBUILDDIR}/tmp" \
  "${TARGET}-${VERSION}" 

# Run the build
rpmbuild \
  --define "_topdir ${RPMBUILDDIR}" \
  -ba "${RPMBUILDDIR}/SPECS/${TARGET}.spec" 

