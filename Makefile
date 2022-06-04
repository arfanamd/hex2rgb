CC=rustc
FLAGS=-C prefer-dynamic -o
ELF_C=termux-elf-cleaner
OUTPUT=hex2rgb
INPUT=hex2rgb.rs
LINK=ln
LINK_FLAGS=-s
ADDITIONAL=rgb2hex

all:
	@ echo 'Compiling...'
	@ ${CC} ${FLAGS} ${OUTPUT} ${INPUT} && ${ELF_C} ${OUTPUT}

build: all
	@ echo 'building hex2rgb and rgb2hex'
	@ ${LINK} ${LINK_FLAGS} ${ADDITIONAL} ${OUTPUT}
