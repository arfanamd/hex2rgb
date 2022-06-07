CC=rustc
IN=hex2rgb.rs
LN=command ln
FLGS=-C prefer-dynamic -o
OUT1=hex2rgb
OUT2=rgb2hex
LN_FLGS=-s

all:
	@ echo 'Compile in dynamic & create the soft link...'
	@ ${CC} ${FLGS} ${OUT1} ${IN} && ${LN} ${LN_FLGS} ${OUT1} ${OUT2}
