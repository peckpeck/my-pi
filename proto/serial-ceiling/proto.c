#include <stdio.h>
#include <wiringPi.h>
#include <sys/time.h>
#include <unistd.h>
#include <time.h>

//data2
#define RED 13
//clk
#define BLUE 6
//wr
#define GREEN 5

#define BLEU BLUE
#define BLANC RED
#define VIOLET GREEN

// clk
#define DATA BLEU
// data2
#define CLOCK BLANC
// wr
#define LINE VIOLET

#define DOWN_SLEEP 5000
#define UP_SLEEP 10000
#define BREAK_SLEEP 70000

static struct timespec tv_down = { 0, DOWN_SLEEP};
static struct timespec tv_up = { 0, UP_SLEEP};
static struct timespec tv_break = { 0, BREAK_SLEEP};


void write_bit(int bit) {
  digitalWrite(DATA, bit);
  digitalWrite(CLOCK, 0);
  nanosleep(&tv_down, NULL);
  digitalWrite(CLOCK, 1);
  nanosleep(&tv_up, NULL);
}

void write_sequence(int* sequence, int len) {
  digitalWrite(LINE, 0);
  nanosleep(&tv_up, NULL);
  for(int i=0;i<len;i++) {
    write_bit(sequence[i]);
  }
  digitalWrite(LINE, 1);
  nanosleep(&tv_break, NULL);
}

void copy_bits(int bits, int *to,int dots) {
  for(int i=0;i<8;i++) {
    if(i==4) { 
      to[i] = dots;
    } else {
      to[i] = 0x1 & (bits>>i);
    }
  }
}
void digit_to_bits(int* start, int value, int direction, int dots) {
  /*    3
   *   --
   * 2| 6|7.
   *   --  4
   * 1|  |5.
   *   --
   *   0
   */
  switch(value + (direction*10)) {
    case 0 : copy_bits(0b10101111, start, dots); break;
    case 1 : copy_bits(0b10100000, start, dots); break;
    case 2 : copy_bits(0b11001011, start, dots); break;
    case 3 : copy_bits(0b11101001, start, dots); break;
    case 4 : copy_bits(0b11100100, start, dots); break;
    case 5 : copy_bits(0b01101101, start, dots); break;
    case 6 : copy_bits(0b01101111, start, dots); break;
    case 7 : copy_bits(0b10101000, start, dots); break;
    case 8 : copy_bits(0b11101111, start, dots); break;
    case 9 : copy_bits(0b11101101, start, dots); break;
    case 10 : copy_bits(0b10101111, start, dots); break;
    case 11 : copy_bits(0b00000110, start, dots); break;
    case 12 : copy_bits(0b11001011, start, dots); break;
    case 13 : copy_bits(0b01001111, start, dots); break;
    case 14 : copy_bits(0b01100110, start, dots); break;
    case 15 : copy_bits(0b01101101, start, dots); break;
    case 16 : copy_bits(0b11101101, start, dots); break;
    case 17 : copy_bits(0b00000111, start, dots); break;
    case 18 : copy_bits(0b11101111, start, dots); break;
    case 19 : copy_bits(0b01101111, start, dots); break;
  }
}

void set_time(int* start, int hour, int minute, int direction) {
  digit_to_bits(&bits4[0], hour/10, direction, 0);
  digit_to_bits(&bits4[8], hour%10, direction, 1);
  digit_to_bits(&bits4[16], minute/10, direction, 0);
  digit_to_bits(&bits4[24], minute%10, direction, 0);
}

int main (void)
{
  wiringPiSetupGpio () ;
  printf("Starting ...\n");

  pinMode(LINE, OUTPUT);
  pinMode(CLOCK, OUTPUT);
  pinMode(DATA, OUTPUT);
  
  int bits1[] = { 1,0,0,0,0,1,0,1,0,0,1,0 };
  int bits2[] = { 1,0,0,0,0,0,0,0,0,0,1,0 };
  int bits3[] = { 1,0,0,0,0,0,0,0,0,1,1,0 };
  int bits4[41] = { 1,0,1,0,1,1,0,0,0 };
  set_time(&bits[9], 10, 24, 0);

  write_sequence(bits1, 12 ); 
  write_sequence(bits2, 12 ); 
  write_sequence(bits3, 12 ); 
  write_sequence(bits4, 41 ); 
}
