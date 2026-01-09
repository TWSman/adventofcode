import pandas as pd
import numpy as np
from dataclasses import dataclass
from copy import copy



@dataclass
class State:
    ore: int = 0
    clay: int = 0
    obsidian: int = 0
    geode: int = 0
    ore_robots: int = 1
    clay_robots: int = 0
    obsidian_robots: int = 0
    geode_robots: int = 0
    time: int = 0

    # Ore, clay, obsidian, total
    ore_robot_cost = (2,0,0,2)
    clay_robot_cost = (3,0,0,3)
    obsidian_robot_cost = (3,8,0,11)
    geode_robot_cost = (3,0,12,15)

    # Optimal strategy for the default robot
    # Minute 3:
    # Build ore robot
    # Minute 5:
    # Build ore robot (now 3 ore robots)
    # Minute 6:
    # Build clay robot
    # Minute 7:
    # Build clay robot
    # Minute 8:
    # Build clay robot
    # Minute 9:
    # Build clay robot
    # Minute 10:
    # Build clay robot (now 5 clay robots)
    # Minute 11:
    # Build obsidian robot
    # Minute 12:
    # Build clay robot (now 6 clay robots)
    # Minute 13:
    # Build obsidian robot
    # Minute 14:
    # Build obsidian robot
    # Minute 15:
    # Build obsidian robot (now 4 obsidian robots)
    # Minute 16:
    # Wait 
    # Minute 17:
    # Build obsidian robot (now 5 obsidian robots)
    # Minute 18:
    # Build geode robot (this will produce 6 geodes)
    # Minute 19:
    # Build Obsidian robot (now 6 obsidian robots)
    # Minute 20:
    # Build geode robot (now 2 geode robots) (this will produce 4 geodes)
    # Minute 21:
    # Build Obsidian robot (now 7 obsidian robots)
    # Minute 22:
    # Build Geode robot (now 3 geode robots) (this will produce 2 geodes)
    # Minute 23:
    # Build Obsidian robot (now 8 obsidian robots)
    # Minute 24:
    # Just wait


    def __str__(self):
        return (f"Time: {self.time}\n"
            f"Resources - Ore: {self.ore}, Clay: {self.clay}, Obsidian: {self.obsidian}, Geode: {self.geode}\n"
            f"Robots - Ore Robots: {self.ore_robots}, Clay Robots: {self.clay_robots}, "
            f"Obsidian Robots: {self.obsidian_robots}, Geode Robots: {self.geode_robots}\n"
            f"Value: {self.get_value()}\n")


    def can_afford(self, what: str) -> bool:
        match what:
            case "ore": return self.ore >= self.ore_robot_cost[0]
            case "clay": return self.ore >= self.clay_robot_cost[0]
            case "obsidian": return (self.ore >= self.obsidian_robot_cost[0] and self.clay >= self.obsidian_robot_cost[1])
            case "geode": return (self.ore >= self.geode_robot_cost[0] and self.obsidian >= self.geode_robot_cost[2])
        return False

    # The version copied from rust
    def get_value_orig(self) -> float:
        remaining_time = 24 - self.time
        geode_value = 1.0
        geode_robot_value = remaining_time * geode_value

        # Obsidian is used to make geode robots
        # Obsidian value needs to converted to geode robots. Thus -1.0
        
        if remaining_time > 1.0:
            obsidian_value = (remaining_time - 1.0) * geode_value / self.geode_robot_cost[2]
        else:
            obsidian_value = 0.0
        obsidian_robot_value = remaining_time * obsidian_value

        # Clay is used to make obsidian robots
        # Clay needs to first converted to obsidian robots, then to geode robots. Thus -2.0
        if remaining_time > 2.0:
            clay_value = obsidian_value * (remaining_time - 2.0) / self.obsidian_robot_cost[1]
        else:
            clay_value = 0.0
        clay_robot_value = remaining_time * clay_value

        # Ore is used for everything. Thus value is a sum of all uses
        # First consider ore used to make geode robots. Through geode robots there is a lag of 1
        # step before value is created
        if remaining_time > 1.0:
            ore_value = geode_value * (remaining_time - 1.0) / self.geode_robot_cost[0]
        else:
            ore_value = 0.0

        if remaining_time > 2.0:
            # Next consider ore used to make obsidian robots. Through obsidian robots there is a
            # lag of 2 steps before value is created
            ore_value += obsidian_value * (remaining_time - 2.0) / self.obsidian_robot_cost[0]

        if remaining_time > 3.0:
            # Next consider ore used to make clay robots. Through clay robots there is a lag of 3
            ore_value += clay_value * (remaining_time - 3.0) / self.clay_robot_cost[0]

        if remaining_time > 4.0:
            # Finally consider ore used to make new ore robots. Through ore robots there is a lag of 4
            ore_value += ore_value * (remaining_time - 4.0) / self.ore_robot_cost[0]

        assert(ore_value >= 0.0)
        ore_robot_value = remaining_time * ore_value

        return (self.geode * geode_value +
        self.obsidian * obsidian_value +
        self.clay * clay_value +
        self.ore * ore_value +
        self.geode_robots * geode_robot_value +
        self.obsidian_robots * obsidian_robot_value +
        self.clay_robots * clay_robot_value +
        self.ore_robots * ore_robot_value)

    # Fixed version
    def get_value(self) -> dict:
        remaining_time = 24 - self.time
        geode_value = 1.0
        geode_robot_value = remaining_time * geode_value

        # Obsidian is used to make geode robots
        # Obsidian value needs to converted to geode robots. Thus -1.0
        
        if remaining_time > 1.0:
            obsidian_value = (remaining_time - 1.0) * geode_value / self.geode_robot_cost[2]
        else:
            obsidian_value = 0.0
        obsidian_robot_value = remaining_time * obsidian_value

        # Clay is used to make obsidian robots
        # Clay needs to first converted to obsidian robots, then to geode robots. Thus -2.0
        if remaining_time > 2.0:
            clay_value = obsidian_value * (remaining_time - 2.0) / self.obsidian_robot_cost[1]
        else:
            clay_value = 0.0
        clay_robot_value = remaining_time * clay_value

        # Ore is used for everything. Thus value is a sum of all uses
        # First consider ore used to make geode robots. Through geode robots there is a lag of 1
        # step before value is created
        if remaining_time > 1.0:
            ore_value = geode_value * (remaining_time - 1.0) / self.geode_robot_cost[3]
        else:
            ore_value = 0.0

        if remaining_time > 2.0:
            # Next consider ore used to make obsidian robots. Through obsidian robots there is a
            # lag of 2 steps before value is created
            ore_value += obsidian_value * (remaining_time - 2.0) / self.obsidian_robot_cost[3]

        if remaining_time > 3.0:
            # Next consider ore used to make clay robots. Through clay robots there is a lag of 3
            ore_value += clay_value * (remaining_time - 3.0) / self.clay_robot_cost[0]

        #if remaining_time > 4.0:
            ## Finally consider ore used to make new ore robots. Through ore robots there is a lag of 4
            #ore_value += ore_value * (remaining_time - 4.0) / self.ore_robot_cost[0]

        assert(ore_value >= 0.0)
        ore_robot_value = remaining_time * ore_value

        return {
            "total": (self.geode * geode_value +
        self.obsidian * obsidian_value +
        self.clay * clay_value +
        self.ore * ore_value +
        self.geode_robots * geode_robot_value +
        self.obsidian_robots * obsidian_robot_value +
        self.clay_robots * clay_robot_value +
        self.ore_robots * ore_robot_value),
            "ore_robot": ore_robot_value,
            "clay_robot": clay_robot_value,
            "obsidian_robot": obsidian_robot_value,
            "geode_robot": geode_robot_value,
            "ore": ore_value,
            "ore_from_geode": geode_value * (remaining_time - 1.0) / self.geode_robot_cost[3],
            "ore_from_obsidian": obsidian_value * (remaining_time - 2.0) / self.obsidian_robot_cost[3],
            "ore_from_clay": clay_value * (remaining_time - 3.0) / self.clay_robot_cost[3],
            "ore_from_ore": ore_value * (remaining_time - 4.0) / self.ore_robot_cost[3],
            "clay": clay_value,
            "obsidian": obsidian_value,
            "geode": geode_value,
        }



def main():
    state = State()

    while True:
        can_do = False
        state.time += 1
        
        print(state)
        if state.time == 24:
            print("Reached end with")
            state.ore += state.ore_robots
            state.clay += state.clay_robots
            state.obsidian += state.obsidian_robots
            state.geode += state.geode_robots
            print(state)
            break

        accectable_inputs = [0]
        if state.can_afford("ore"):
            new_state = copy(state)
            new_state.time += 1
            value = new_state.get_value()
            print(f"0: Wait (value: {value['total']:.01f})")
        if state.can_afford("ore"):
            can_do = True
            accectable_inputs.append(1)
            new_state = copy(state)
            new_state.ore -= new_state.ore_robot_cost[0]
            new_state.ore_robots += 1
            new_state.time += 1
            value = new_state.get_value()
            print(f"1: Build Ore Robot (cost: {state.ore_robot_cost[0]} ore) (exp. value: {value['total']:.01f})")
        if state.can_afford("clay"):
            can_do = True
            accectable_inputs.append(2)
            new_state = copy(state)
            new_state.ore -= new_state.clay_robot_cost[0]
            new_state.clay_robots += 1
            new_state.time += 1
            value = new_state.get_value()
            print(f"2: Build Clay Robot (cost: {state.clay_robot_cost[0]} ore) (exp. value: {value['total']:.01f})")
        if state.can_afford("obsidian"):
            can_do = True
            accectable_inputs.append(3)
            new_state = copy(state)
            new_state.ore -= new_state.obsidian_robot_cost[0]
            new_state.clay -= new_state.obsidian_robot_cost[1]
            new_state.obsidian_robots += 1
            new_state.time += 1
            value = new_state.get_value()
            print(f"3: Build Obsidian Robot (cost: {state.obsidian_robot_cost[0]} ore, {state.obsidian_robot_cost[1]} clay) (exp. value: {value['total']:.01f})")
        if state.can_afford("geode"):
            can_do = True
            accectable_inputs.append(4)
            new_state = copy(state)
            new_state.ore -= new_state.geode_robot_cost[0]
            new_state.obsidian -= new_state.geode_robot_cost[2]
            new_state.geode_robots += 1
            new_state.time += 1
            value = new_state.get_value()
            print(f"4: Build Geode Robot (cost: {state.geode_robot_cost[0]} ore, {state.geode_robot_cost[2]} obsidian) (exp. value: {value['total']:.01f})")

        state.ore += state.ore_robots
        state.clay += state.clay_robots
        state.obsidian += state.obsidian_robots
        state.geode += state.geode_robots

        if not can_do:
            print("You can only wait")
            print()
            continue
        else:
            print("What do you want to do: ")
            while True:
                inp = int(input())
                if inp not in accectable_inputs:
                    print("Invalid input, try again: ")
                else:
                    break

        match inp:
            case 0:
                pass
            case 1:
                state.ore -= state.ore_robot_cost[0]
                state.ore_robots += 1
            case 2:
                state.ore -= state.clay_robot_cost[0]
                state.clay_robots += 1
            case 3:
                state.ore -= state.obsidian_robot_cost[0]
                state.clay -= state.obsidian_robot_cost[1]
                state.obsidian_robots += 1
            case 4:
                state.ore -= state.geode_robot_cost[0]
                state.obsidian -= state.geode_robot_cost[2]
                state.geode_robots += 1
        print("New state value: ", state.get_value())
        print()


if __name__ == "__main__":
    main()

