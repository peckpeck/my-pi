#include <stdio.h>
#include <wiringPi.h>
#include <sys/time.h>
#include <unistd.h>

#define DATA2_o 22
#define CLK_o 17
#define WR_o 27

#define DATA2_i 13
#define CLK_i 6
#define WR_i 5

#define LED 24
#define V33 23

void write_bits(char *bits,int count) {
  digitalWrite(WR_o, 1);
  digitalWrite(CLK_o, 1);
  digitalWrite(DATA2_o, 1);
  usleep(50);
  digitalWrite(CLK_o, 0);
  digitalWrite(DATA2_o, 0);
  usleep(30);
  for(int i=0;i<count;i++) {
    digitalWrite(CLK_o, 1);
    usleep(30);
    digitalWrite(CLK_o, 0);
    usleep(5);
    digitalWrite(DATA2_o, bits[i]);
    usleep(30);
    digitalWrite(DATA2_o, 0);
    usleep(5);
  }
}

int main (void)
{
  wiringPiSetupGpio () ;
  printf("Starting ...\n");
  
  // led and 3.3v on
  pinMode (LED, OUTPUT) ;
  digitalWrite(LED, 1);
  pinMode (V33, OUTPUT) ;
  digitalWrite(V33, 1);
usleep(5);
  // data transfer
  pinMode (DATA2_o, OUTPUT) ;
  pinMode (CLK_o, OUTPUT) ;
  pinMode (WR_o, OUTPUT) ;
  pinMode (DATA2_i, INPUT) ;
  pullUpDnControl(DATA2_i, PUD_OFF);
  pinMode (CLK_i, INPUT) ;
  pullUpDnControl(CLK_i, PUD_OFF);
  pinMode (WR_i, INPUT) ;
  pullUpDnControl(WR_i, PUD_OFF);

  struct timeval tv;
  int data2, clk, wr, data0, clk0, wr0;
  int dataval = 0;
  int usec0 = 0;

  clk = digitalRead(CLK_i);
  digitalWrite(CLK_o, clk);
  wr = digitalRead(WR_i);
  digitalWrite(WR_o, wr);
  data2 = digitalRead(DATA2_i);
  digitalWrite(DATA2_o, data2);
  printf("Started!\n");

  char bits[] = {0,0,0,1,0,1,0,1, 0,0,0,0,0,0,0,1};
  for (;;)
  {
    write_bits(bits,16);
    sleep(1);
  }
  return 0 ;
}


