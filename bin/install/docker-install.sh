#!/bin/bash

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