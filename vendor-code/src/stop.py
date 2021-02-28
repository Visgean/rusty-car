import time

from motor import Motor

PWM = Motor()


# Main program logic follows:
if __name__ == '__main__':
    PWM.setMotorModel(0, 0, 0, 0)  # Stop
