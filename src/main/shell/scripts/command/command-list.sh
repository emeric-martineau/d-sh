command_list(){
  for prog in $(ls "${BASEDIR}/program" | sort); do
    echo "${prog%.*}"
  done
}

COMMAND_DESCRIPTION="List all program available"
COMMAND_MIN_ARGS=0
COMMAND_MAX_ARGS=0
