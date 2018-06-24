for prog in $(ls program); do
  . "${BASEDIR}/program/${prog}"

  NUMBER_IMAGE_EXISTS=$(docker image list ${APPLICATION_IMAGE_DOCKER} | wc -l)
  STATUS

  if [ "${NUMBER_IMAGE_EXISTS}" -lt 2 ]; then
    STATUS="Build need"
  else
    STATUS="Build done"
  fi

  printf "%-34s%-34s%-13s\n" "${prog%.*}" "${APPLICATION_IMAGE_DOCKER}" "${STATUS}"
done
