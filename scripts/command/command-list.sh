for prog in $(ls "${BASEDIR}/program" | sort); do
  echo "${prog%.*}"
done
