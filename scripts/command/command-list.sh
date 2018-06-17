for prog in $(ls "${BASEDIR}/program"); do
  echo "${prog%.*}"
done
