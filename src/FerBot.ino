#include <ArduinoSort.h>

const int motor1 = 43;
const int motor2 = 37;
const int motor3 = 31;
const int motor4 = 25;

const int m1_enable = 41;
const int m2_enable = 35;
const int m3_enable = 29;
const int m4_enable = 27;

const int dir1 = 45; 
const int dir2 = 39;
const int dir3 = 33;
const int dir4 = 23;

const int motors[] = {motor1, motor2, motor3, motor4};
const int dirs[] = {dir1, dir2, dir3, dir4};

const int del = 100;

const int Fan = 9;

int min = 0;
int max = 0;


void setup() {
  Serial.begin(115200);

  //define Outputs
  pinMode(motor1,OUTPUT); 
  pinMode(motor2,OUTPUT); 
  pinMode(motor3,OUTPUT); 
  pinMode(motor4,OUTPUT); 

  // pinMode(22, INPUT);
  // pinMode(24, INPUT);

  pinMode(dir1,OUTPUT);
  pinMode(dir2,OUTPUT);
  pinMode(dir3,OUTPUT);
  pinMode(dir4,OUTPUT);

  pinMode(m1_enable,OUTPUT);
  pinMode(m2_enable,OUTPUT);
  pinMode(m3_enable,OUTPUT);
  pinMode(m4_enable,OUTPUT);

  pinMode(Fan,OUTPUT);
  
  //set States
  digitalWrite(dir1, HIGH);
  digitalWrite(dir2, HIGH);
  digitalWrite(dir3, HIGH);
  digitalWrite(dir4, HIGH);
//  
  digitalWrite(m1_enable, LOW);
  digitalWrite(m2_enable, LOW);
  digitalWrite(m3_enable, LOW);
  digitalWrite(m4_enable, LOW);
}


// void simpleAccel(int motor, long steps) {
//   int lowspeed = 1500;
//   int highspeed = 40;
//   int change = 2;

  
  
//   long rampUpStop = (lowspeed - highspeed) / change;
//   if (rampUpStop > steps / 2) {
//     rampUpStop = steps / 2;  
//   }

//   long rampDownStart = steps - rampUpStop;

//   int d = lowspeed;

//   for (long i = 0; i < steps; i++) {
//     digitalWrite(motors[motor], HIGH);
//     digitalWrite(motors[motor], LOW);
//     delayMicroseconds(d);

//     if (i < rampUpStop)
//       d -= change;
//     else if (i > rampDownStart)
//       d += change;
//   }
// }

#define DEF_INTER 150

void SameSpeedMove(long stepsarr[4]) {
    long stepsLeft[] = {0,0,0,0};


    // set direction for every motor
    for (int motor = 0; motor < 4; motor++) {
      long steps = stepsarr[motor];
      
      if (stepsarr[motor] < 0) {
        digitalWrite(dirs[motor], LOW);
        stepsarr[motor] = -1 * stepsarr[motor];
      }
      else {
        digitalWrite(dirs[motor], HIGH);
      }
    }

    // calculating inters
    long inters[] = {DEF_INTER, DEF_INTER, DEF_INTER, DEF_INTER};
    long max_steps = max(max(stepsarr[0], stepsarr[1]), max(stepsarr[2], stepsarr[3]));
    // Serial.println(max_steps);
    long time = max_steps * DEF_INTER;
    // Serial.println(time);
    // Serial.println();
    for (int i = 0; i < 4; i++) {
      if (stepsarr[i] != 0) {
        inters[i] = time / stepsarr[i];
        // Serial.println(inters[i]);
      }
      
    }
    // Serial.println();
    long inter_gcd = gcd(gcd(inters[0], inters[1]), gcd(inters[2], inters[3])); 
    // Serial.println(inter_gcd);
    long tick_count = 0;

    // move all motors simultaniosly
    while ((stepsarr[0] - stepsLeft[0]) || (stepsarr[1] - stepsLeft[1]) || (stepsarr[2] - stepsLeft[2]) || (stepsarr[3] - stepsLeft[3])) {
      for (int motor = 0; motor < 4; motor++) {
        if ((stepsarr[motor] - stepsLeft[motor]) && ((tick_count * inter_gcd) % inters[motor] == 0)) {
          digitalWrite(motors[motor], HIGH);
          digitalWrite(motors[motor], LOW);
          stepsLeft[motor]++;
        }
      }
      delayMicroseconds(inter_gcd);
      tick_count++;
    }
}

int gcd(int a, int b) {
  while (a!=b) {
    if (a>b) {
      a = a - b;
    } else {
      b = b - a;
    }
  }
  return a;
}

void simpleMove(int motor, long steps) {
  int interval = 100;

  for (int i = 0; i < steps; i++) {
    digitalWrite(motors[motor], HIGH);
    digitalWrite(motors[motor], LOW);
    delayMicroseconds(interval);
  }
}

void notSimpleMove(long stepsarr[4]) {
    // Serial.println(stepsarr[0]);
    // Serial.println(stepsarr[1]);
    // Serial.println(stepsarr[2]);
    // Serial.println(stepsarr[3]);
    long stepsLeft[] = {0,0,0,0};

    // set direction for every motor
    for (int motor = 0; motor < 4; motor++) {
      long steps = stepsarr[motor];
      
      if (stepsarr[motor] < 0) {
        digitalWrite(dirs[motor], LOW);
        stepsarr[motor] = -1 * stepsarr[motor];
      }
      else {
        digitalWrite(dirs[motor], HIGH);
      }
    }

    // move all motors simultaniosly
    while ((stepsarr[0] - stepsLeft[0]) || (stepsarr[1] - stepsLeft[1]) || (stepsarr[2] - stepsLeft[2]) || (stepsarr[3] - stepsLeft[3])) {
      for (int motor = 0; motor < 4; motor++) {
        if (stepsarr[motor] - stepsLeft[motor]) {
          digitalWrite(motors[motor], HIGH);
          digitalWrite(motors[motor], LOW);
          stepsLeft[motor]++;
        }
      }
      delayMicroseconds(100);
    }
}

long calibOne(int motor) {
  // кнопка
  // HIGH - не нажата
  // LOW - нажата
  // while (digitalRead(24) == HIGH) {
  //   // Serial.println(24);
  //   // delay(3000);
  // }
  bool a = false;
  while (digitalRead(24) == LOW) {
    a = true;
  }
  // Serial.println("poshel");
  while (digitalRead(24) == HIGH) {
    // Serial.println("Moving -1");
    for (int i = 0; i<100; i++) {
      digitalWrite(motors[motor], HIGH);
      digitalWrite(motors[motor], LOW);
      delayMicroseconds(100);
    }
  }
  bool b = false;
  while (digitalRead(24) == LOW) {
    b = true;
  }

  long counter = 0;
  // Serial.println("poshel obratno");
    digitalWrite(dirs[motor], LOW);
  while (digitalRead(24) == HIGH) {
    // Serial.println("Moving 1");
    for (int i = 0; i<100; i++) {
      digitalWrite(motors[motor], HIGH);
      digitalWrite(motors[motor], LOW);
      delayMicroseconds(100);
    }
    counter = counter + 100;
  }
  digitalWrite(dirs[motor], HIGH);
  // notSimpleMove(stepsarrc);
  // Serial.println("center");
  for (int i = 0; i<(counter / 2); i++) {
      digitalWrite(motors[motor], HIGH);
      digitalWrite(motors[motor], LOW);
      delayMicroseconds(100);
    }
  
  // Serial.println(counter / 2);
  return (counter / 2);


}

void calibAll() {
  long wz = calibOne(3);
  delay(1000);
  long ez = calibOne(2);
  delay(1000);
  long sz = calibOne(1);
  delay(1000);
  long sy = calibOne(0);
  delay(1000);
  // long sy = calibOne(4);

  Serial.print(sy);
  Serial.print(" ");
  Serial.print(sz);
  Serial.print(" ");
  Serial.print(ez);
  Serial.print(" ");
  Serial.println(wz);
}

int count = 0;
bool calibrated = false;


void loop() {
  digitalWrite(Fan, HIGH);
  // if (!calibrated) {
  //   calibTest(3);
  //   calibrated = true;
  // }

  
  // if (digitalRead(24) == LOW) {
  //   Serial.println(24);
  //   delay(100);
  // }
  // Serial.println(digitalRead(24));
  // Serial.println("qwe", 789, 456, 123);
  
  

// ******************************************

  
  long mode = 5;  
  long stepsarr[] = {0,0,0,0};
  while (Serial.available() > 0) {
    // Serial.println("while");
    mode = Serial.parseInt();
  
    for (int motor = 0; motor < 4; motor++) {
      stepsarr[motor] = Serial.parseInt();
    }
    
    if (mode == 0) {
      notSimpleMove(stepsarr);
    }
    else if (mode == 1) {
      calibAll();
    }
    else if (mode == 2) {
      SameSpeedMove(stepsarr);
    }
  }
// ******************************************
}
