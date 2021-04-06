#include <wiringPi.h>
#include <softPwm.h>
#include <time.h>
#include <stdlib.h>

// in loop count
#define REFRESH_PERIOD 1000
// in us (real duration is 8x)
#define LOOP_DURATION 1000
float luminosity = 1;

#define LED_1  8
#define LED_2  25
#define LED_3  24
#define LED_4  23
#define LED_5  18
#define LED_6  15
#define LED_7  14
#define LED_8  4
#define LED_9  17
#define LED_10 27
#define LED_11 22
#define LED_12 10
#define LED_13 9
#define LED_14 11

#define D_A LED_13
#define D_B LED_9
#define D_C LED_4
#define D_D LED_2
#define D_E LED_1
#define D_F LED_12
#define D_G LED_5
#define D_P LED_3

#define D_1 LED_14
#define D_2 LED_11
#define D_3 LED_10
#define D_4 LED_6

#define COL LED_8

int out_list[] = { LED_1, LED_2, LED_3, LED_4, LED_5, LED_6, LED_7, LED_8, LED_9, LED_10, LED_11, LED_12, LED_13, LED_14 };
int d0[] = { D_A, D_B, D_C, D_D, D_E, D_F };
int d1[] = { D_B, D_C };
int d2[] = { D_A, D_B, D_G, D_E, D_D };
int d3[] = { D_A, D_B, D_G, D_C, D_D };
int d4[] = { D_F, D_G, D_B, D_C };
int d5[] = { D_A, D_F, D_G, D_C, D_D };
int d6[] = { D_A, D_F, D_E, D_D, D_C, D_G };
int d7[] = { D_A, D_B, D_C };
int d8[] = { D_A, D_B, D_C, D_D, D_E, D_F, D_G };
int d9[] = { D_A, D_B, D_C, D_D, D_F, D_G };
int dX[] = { D_A, D_B, D_C, D_D, D_E, D_F, D_G, D_P };


void set_list(int leds[], int size, int state) {
  for(int i=0; i<size; i++) {
    digitalWrite(leds[i], state);
  }
}

void set_digit(int d, int dot) {
  switch(d) {
    case 0:
      set_list(d0, sizeof(d0)/sizeof(d0[0]), HIGH);
      break;
    case 1:
      set_list(d1, sizeof(d1)/sizeof(d1[0]), HIGH);
      break;
    case 2:
      set_list(d2, sizeof(d2)/sizeof(d2[0]), HIGH);
      break;
    case 3:
      set_list(d3, sizeof(d3)/sizeof(d3[0]), HIGH);
      break;
    case 4:
      set_list(d4, sizeof(d4)/sizeof(d4[0]), HIGH);
      break;
    case 5:
      set_list(d5, sizeof(d5)/sizeof(d5[0]), HIGH);
      break;
    case 6:
      set_list(d6, sizeof(d6)/sizeof(d6[0]), HIGH);
      break;
    case 7:
      set_list(d7, sizeof(d7)/sizeof(d7[0]), HIGH);
      break;
    case 8:
      set_list(d8, sizeof(d8)/sizeof(d8[0]), HIGH);
      break;
    case 9:
      set_list(d9, sizeof(d9)/sizeof(d9[0]), HIGH);
      break;
  }
  if(dot) {
    digitalWrite(D_P, HIGH);
  }
}

void set_col(int state) {
  digitalWrite(COL, state);
}

void clear_digit() {
  set_list(dX, sizeof(dX)/sizeof(dX[0]), LOW);
}

void select_digit(int id, int selected) {
  int state;
  if(selected) {
    state = LOW;
  } else {
    state = HIGH;
  }
  switch(id) {
    case 1:
      digitalWrite(D_1, state);
      break;
    case 2:
      digitalWrite(D_2, state);
      break;
    case 3:
      digitalWrite(D_3, state);
      break;
    case 4:
      digitalWrite(D_4, state);
      break;
  }
}

void do_delay(int type) {
  int delay;
  if(type == 0) {
    delay = 0 + 4*(1-luminosity)*LOOP_DURATION;
  } else {
    delay = LOOP_DURATION*luminosity;
  }
  if(delay != 0)
    delayMicroseconds(delay);
}
void digit(int id, int value) {
  set_digit(value, 0);
  select_digit(id, 1);
  do_delay(1);
  select_digit(id, 0);
  clear_digit();
}

void show_time(int digits[4]) {
  if(digits[0] != 0) {
    digit(1, digits[0]);
  }
  set_col(HIGH);
  digit(2, digits[1]);
  set_col(LOW);
  digit(3, digits[2]);
  digit(4, digits[3]);
}

void get_time(int digits[4]) {
  time_t rawtime;
  struct tm * timeinfo;

  time ( &rawtime );
  timeinfo = localtime ( &rawtime );
  digits[0] = timeinfo->tm_hour/10;
  digits[1] = timeinfo->tm_hour%10;
  digits[2] = timeinfo->tm_min/10;
  digits[3] = timeinfo->tm_min%10;
}

int main(int argc, char *argv[]) {
  if(argc == 2) {
    // luminosity given
    luminosity = atof(argv[1]);
  }
  wiringPiSetupGpio();
  for(int i=0; i<sizeof(out_list)/sizeof(out_list[0]); i++) {
    pinMode(out_list[i], OUTPUT);
    digitalWrite(out_list[i], LOW);
  }
  clear_digit();
  select_digit(1, 0);
  select_digit(2, 0);
  select_digit(3, 0);
  select_digit(4, 0);
  set_col(HIGH);
  piHiPri(10);

  int digits[4];
  while(1) {
    get_time(digits);
    for(int i=0; i<100; i++) {
      show_time(digits);
      do_delay(0);
    }
  }
  return 0;
}

