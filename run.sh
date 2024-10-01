#!/bin/bash
set -e

#### Path
CURRENT_PATH="$(pwd)"

DOCKER_INSTALL="$CURRENT_PATH/bin/docker-install.sh"
KUBERNETES_INSTALL="$CURRENT_PATH/bin/kubernetes-install.sh"
GIT_INSTALL="$CURRENT_PATH/bin/git-install.sh"

DOCKER_RUN="$CURRENT_PATH/bin/docker-run.sh"
KUBERNETES_RUN="$CURRENT_PATH/bin/kubernetes-run.sh"

#### String Check
YES="^[yY]$"
NO="^[nN]$"

#### Install
function Docker_Install() {
  if [[ -x $DOCKER_INSTALL ]]; then
          source $DOCKER_INSTALL
  else
      echo "Docker Install File Not Found"
      return 1
  fi
}

function Kubernetes_Install() {
    if [[ -x $KUBERNETES_INSTALL ]]; then
        source $KUBERNETES_INSTALL
    else
        echo "Kubernetes Install File Not Found"
        return 1
    fi
}

function Git_Install() {
    if [[ -x $GIT_INSTALL ]]; then
        source $GIT_INSTALL
    else
        echo "Git Install File Not Found"
        return 1
    fi
}

function Rust_Install() {}

function Install_menu() {
  options=("Docker" "Kubernetes" "Git" "Rust" "Back")
  select SELECT_MENU in "${options[@]}"
  do
      case $SELECT_MENU in
          Docker) Docker_Install ;;
          Kubernetes) Kubernetes_Install ;;
          Git) Git_Install ;;
          Rust) Rust_Install ;;
          Back) Select_menu ;;
      esac
  done
}

#### Run
#function Docker_run() {}
#function Kubernetes_run() {}

function Run_menu() {
  clear
  options=("Docker" "Kubernetes" "Back")
  select SELECT_MENU in "${options[@]}"
  do
      case $SELECT_MENU in
          Docker) Docker_run ;;
          Kubernetes) Kubernetes_run ;;
          Back) Select_menu ;;
      esac
  done
}

#### Update
function Update() {
  clear
  options=("Ubuntu" "RHEL" "Back")
  select SELECT_MENU in "${options[@]}"
  do
    case $SELECT_MENU in
      Ubuntu) ;;
      RHEL) ;;
      Back) Select_menu ;;
    esac
  done
}

#### Exit
# -n : 입력키 활당 + 1 : 한글자 허용
# -s : 입력값에 대한 출력을 비공개
# -r : 읽을수 있는지 여부
# -p : 들여쓰기 없이 한라인에 사용
function Exit() {
  read -n 1 -s -r -p "Press any key to exit..."
  exit 1
}

function Select_menu() {
  clear
  options=("Install" "Run" "Exit")
  PS3="Select Menu : "
  select SELECT_MENU in "${options[@]}"
  do
      case $SELECT_MENU in
          Install) Install_menu ;;
          Run) Run_menu ;;
          Update) Update ;;
          Exit) Exit ;;
      esac
  done
}
Select_menu