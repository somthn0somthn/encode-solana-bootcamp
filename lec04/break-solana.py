import pyautogui
import time

pyautogui.PAUSE = 0.01

def press_key(key, repitions):
    for _ in range(repitions):
        pyautogui.press(key)

if __name__ == "__main__":
    key_to_press = 'a'
    number_of_repetitions = 1000000
 
    press_key(key_to_press, number_of_repetitions)

