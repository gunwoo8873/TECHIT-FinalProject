#!/usr/bin/env bash

#### Path
DOCKER_INSTALL="./bin/docker-install.sh"
KUBERNETES_INSTALL="./bin/kubernetes-install.sh"
GIT_INSTALL="./bin/git-install.sh"


#### Install
function Docker_install() {
  source $DOCKER_INSTALL
}

function Kubernetes_install() {
    source $KUBERNETES_INSTALL
}

function Git_install() {
    source $GIT_INSTALL
}


function Install_menu() {
  options=("Docker" "Kubernetes" "Git" "Exit")
  select SELECT_MENU in "${options[@]}" do
      case $SELECT_MENU in
          Docker) Docker_install ;;
          Kubernetes) Kubernetes_install ;;
          Git) Git_install ;;
          Exit) Exit ;;
      esac
  done
}

#### Run

function Run_menu() {
  options=("Docker" "Kubernetes" "Exit")
  select SELECT_MENU in "${options[@]}" do
      case $SELECT_MENU in
          Docker);;
          Kubernetes);;
          Exit) Exit ;;
      esac
  done
}

function Exit() {
  read -n 1 -s -r -p "Press any key to exit..."
}

function Select_menu() {
  options=("Install" "Run" "Exit")
  PS3="Select Menu : "
  select SELECT_MENU in "${options[@]}" do
      case $SELECT_MENU in
          Install) Install_menu ;;
          Run) Run_menu ;;
          Exit) Exit ;;
      esac
  done
}
Select_menu