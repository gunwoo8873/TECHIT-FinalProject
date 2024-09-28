#!/bin/bash
function Ubuntu_install() {
  echo Git Install...
  sudo apt install -y git
  git version
  return 1
}

function RHEL_install() {
  echo Git Install...
  sudo yum install -y git
  git version
  return 1
}

function Version_Install() {
  versions=("Ubuntu" "RHEL" "Back")
  select SELECT_VERSION in "${versions[@]}"
  do
    case $SELECT_VERSION in
      Ubuntu) Ubuntu_install ;;
      RHEL) RHEL_install ;;
      Back) Select_menu ;;
      *) echo "invalid option $REPLY";;
    esac
  done
}
Version_Install