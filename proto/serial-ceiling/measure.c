#include <stdio.h>
#include <wiringPi.h>
#include <sys/time.h>
#include <unistd.h>

#define RED 13
#define BLUE 6
#define GREEN 5

#define BLEU BLUE
#define BLANC RED
#define VIOLET GREEN

int main (void)
{
  wiringPiSetupGpio () ;
  printf("Starting ...\n");
  
  // data transfer
  pinMode (RED, INPUT) ;
  pullUpDnControl(RED, PUD_OFF);
  pinMode (BLUE, INPUT) ;
  pullUpDnControl(BLUE, PUD_OFF);
  pinMode (GREEN, INPUT) ;
  pullUpDnControl(GREEN, PUD_OFF);

  struct timeval tv_start;
  struct timeval tv_stop;
  int bleu0=2, blanc0=2, violet0=2;
  int bleu, blanc, violet;
  int usec0, id=0,same=0;

  int m_bleu[10000];
  int m_blanc[10000];
  int m_violet[10000];
  for (;;)
  {

    // read and transmit
    bleu = digitalRead(BLEU);
    blanc = digitalRead(BLANC);
    violet = digitalRead(VIOLET);

    // process
    if(id==0 && ((bleu != bleu0) || (blanc != blanc0) || (violet != violet0))) {
      id=1;
      same=0;
      gettimeofday(&tv_start, NULL);
    }
    if(id>0) {
      m_bleu[id] = bleu;
      m_blanc[id] = blanc;
      m_violet[id] = violet;
      id++;
    }
    if((bleu != bleu0) || (blanc != blanc0) || (violet != violet0)) {
      bleu0 = bleu;
      blanc0 = blanc;
      violet0 = violet;
      same=0;
    } else {
      same++;
    }
    if((same > 1000 && id > 0) || id >= 9999) {
      gettimeofday(&tv_stop, NULL);
      for(int i=1; i<id; i++) {
	printf("%04d %d %d %d\n", i, m_bleu[i],m_blanc[i],m_violet[i]);
      }
      printf("Time=%d\n",tv_stop.tv_usec - tv_start.tv_usec);
      fflush(stdout);
      id=0;
      same=0;
    }
    
  }
  return 0 ;
}
