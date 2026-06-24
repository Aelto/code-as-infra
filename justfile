# prepare the .zip files for a release
release-prepare:
  rm -rf release
  just release-make-directory
  just release-prepare-mailqueue

release-make-directory:
  mkdir -p release

release-prepare-mailqueue: release-make-directory
  just services/mail-queue/quadlet/release-make-zip
  cp services/mail-queue/quadlet/mailqueue.release.zip release/mailqueue.release.zip
