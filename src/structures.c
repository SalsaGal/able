#include "structures.h"
#include <stdio.h>

int parseInt(char **file) {
	int toReturn = ((*file)[0] & 0xff) * 0x00000001
		+ ((*file)[1] & 0xff) * 0x00000100
		+ ((*file)[2] & 0xff) * 0x00010000
		+ ((*file)[3] & 0xff) * 0x01000000;
	*file += 4;
	return toReturn;
}

// NEEDS TO BE 0xEE8C
unsigned short int parseWord(char **file) {
	unsigned short int toReturn = ((*file)[0] & 0xff) * 0x0100 + ((*file)[1] & 0xff);
	*file += 2;
	return toReturn;
}

unsigned char parseByte(char **file) {
	unsigned char toReturn = (*file)[0] & 0xff;
	*file += 1;
	return toReturn;
}

SndHeader parseHeader(char **file) {
	SndHeader header;
	header.magicNumber = parseInt(file);
	header.headerSize = parseInt(file);
	header.bankVersion = parseInt(file);
	header.numPrograms = parseInt(file);
	header.numZones = parseInt(file);
	header.numWaves = parseInt(file);
	header.numSequences = parseInt(file);
	header.numLabels = parseInt(file);
	header.reverbMode = parseInt(file);
	header.reverbDepth = parseInt(file);
	return header;
}

SndProgram parseProgram(char **file) {
	SndProgram program;
	program.numZones = parseWord(file);
	program.firstTone = parseWord(file);
	program.volume = parseByte(file);
	program.panPos = parseByte(file);
	parseWord(file);
	return program;
}

SndZone parseZone(char **file) {
	SndZone zone;
	zone.priority = parseByte(file);
	zone.parentProgram = parseByte(file);
	zone.volume = parseByte(file);
	zone.panPos = parseByte(file);
	zone.rootKey = parseByte(file);
	zone.pitchFinetuning = parseByte(file);
	zone.noteLow = parseByte(file);
	zone.noteHigh = parseByte(file);
	zone.mode = parseByte(file);
	zone.maxPitchRange = parseByte(file);
	zone.ADSR1 = parseWord(file);
	zone.ADSR2 = parseWord(file);
	zone.waveIndex = parseWord(file);
	return zone;
}
