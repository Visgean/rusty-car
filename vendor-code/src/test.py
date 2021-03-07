import time

from motor import Motor

PWM = Motor()


def test_motor():
    try:
        # PWM.setMotorModel(1000, 1000, 1000, 1000)  # Forward
        # print("The car is moving forward")
        # time.sleep(1)
        print("The car is going backwards")
        PWM.setMotorModel(-1000, -1000, -1000, -1000)  # Back
        time.sleep(1)
        # PWM.setMotorModel(-1500, -1500, 2000, 2000)  # Left
        # print("The car is turning left")
        # time.sleep(1)
        # PWM.setMotorModel(2000, 2000, -1500, -1500)  # Right
        # print("The car is turning right")
        # time.sleep(1)
        print("\nEnd of program")
        PWM.setMotorModel(0, 0, 0, 0)  # Stop
    except KeyboardInterrupt:
        PWM.setMotorModel(0, 0, 0, 0)
        print("\nEnd of program")



# Main program logic follows:
if __name__ == '__main__':
    test_motor()
