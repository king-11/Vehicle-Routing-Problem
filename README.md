# Vehicle-Routing-Problem

## Problem Statement

Route planning is a complex optimization problem that involves distance, rider capacity, geolocation and mapping. Given n items to deliver and a fleet of m riders, (n>m), and some k dynamically added pickup points we have to optimized for distance, time and bag size of drivers.

## Inter IIT

This was the problem statement for **Inter IIT Tech Meet 11.0** @IITKanpur, High Prep event: **Route Planning for optimized on Time Delivery** by [Grow Simplee](https://www.growsimplee.com/)

We solved the problem statement in three parts as described below and were able to bag the first position 🥇 amongst participating 22 IITs.

## Clustering

We apply the clustering algorithm to each delivery/pickup point sequentially. The algorithm decides the optimal allocation of a point by iteratively trying to insert the point into the path of all riders one by one and choosing the best path out of all of them. The best path here means the path which minimizes the resource (distance and time) usage. 

The best path for a particular rider is computed using the genetic algorithm which determines the optimal route of the rider. This route contains the previously allocated points for this rider as well as the new dynamic point. Genetic algorithm also returns the cost associated with this optimal path. It then calls the feasibility function to make sure that the bag size is not exceeding standard given bag size at any point during the route. 

If the bag size exceeds the standard bag size, this route is discarded otherwise we update the global minimum cost if the current cost is lower than that global cost.

After running the clustering algorithm for all the riders, the rider who’s optimal path had the minimum increase in cost, gets updated to the new path which includes our given point. We thus use the algorithm for all our points to get our set of paths.

### Psuedo Code

```
min_increase = INF
for rider in riders:
  route temp = rider.route
  temp.nodes.add_node(new_point)
  route_optimization(temp)

  // check feasibility
  if(not is_feasible(temp)):
    continue;

  increase_cost = temp.cost - rider.cost

  If increase_cost <= min_increase: 
    min_increase = increase_cost

// Update the suitable rider

suitable_rider.nodes.add_node(new_point)
```

## Genetic Algorithm

In order to find the optimal delivery route for the allocated but not yet completed orders for a certain rider, a genetic algorithm considering the special characteristics of the problem is designed. Below steps are repeated for some (k) iterations:

### Step 0 (chromosome representation)
The chromosomes are coded in natural numbers, i.e., the corresponding codes of orders O1 and O2 are 1, 2, and 3, 4, respectively. A small vector is used representing the nodes index in the current population for a particular rider.

### Step 1 (initial population selection) 
The initial populations are randomly generated from node vectors according to the constraints, with the routes taking time more than the maximum allowed not being considered in the population.

### Step 2 (Order Crossover Operator)
First, two parent chromosomes are randomly selected from the population. They are then crossed using an order crossover(OX) operator. 

This operator selects a substring from a parent at random and produces a proto-child by copying the substring into the corresponding position of it. Then it deletes the genes which are already in the substring from the 2nd parent and places the remaining genes into the unfixed positions of the proto-child from left to right according to the order of the sequence in the 1st parent to produce an offspring. 

### Step 3 (Scramble Mutation Operator)

The obtained children are then mutated using scramble mutation if the generated random value is below mutation probability. In this mutation, a subset of genes is picked at random and then randomly rearranged/shuffled between those positions. This introduces diversity in the sample population and finally they are inserted back into the total population.	
							
### Step 4 (fitness function and selection)

Genetic algorithms use fitness functions to quantify the quality of a given solution to a problem. In this problem, we determine that a “fitter” solution path is the one that is shorter and takes less time to travel through.  Thus we define our fitness function as:

Fitness Function f(distance, timeConsumed) = `(e^(-distance/100)) * 1/(1+timeConsumed)`

After obtaining fitness values across the whole population, we scale it with Sigma Scaling, which moderates selection pressure over time so that it is not too strong in early generations and not too weak once the population has stabilized and fitness differences are smaller. The standard deviation of the population fitness is used to scale the fitness scores.

We store the best path obtained till now and in each generation, if the fitness score of a solution exceeds the score of the best path, we update it. We then select some (n) individuals from the population using Stochastic Universal Selection/Sampling as the next generation of population and iterate again for 5000 generations.

## Enhancement Mutation Algorithm 

We further enhance the routes obtained for the static delivery points by genetic algorithm using this additional mutation step. 

The routes obtained by clustering process does not take into account the effect of order of insertion of the delivery points into the routes. There is an inherent assumption that the previously allocated points will not change for a particular rider. But as shown in the diagram, we can have an optimal solution where we may have to change the previously allocated delivery points from one rider to another. 

Algorithm:

Step 0 (Ejection of points): Firstly, we will select k nodes from all the given nodes and eject them from the routes. These k nodes can be selected using Radial selection:

Radial selection: We select a random point and select k-1 closest points to that point among all the available points.

After selecting the (k) points from the radial selection, we eject them from the routes which we got from the clustering process. After ejecting these points from the routes we get new temporary routes without the ejected points.

These k points are first randomly shuffled and then reinserted into these temporary routes using the clustering algorithm. This way we get the final routes. This mutation process is repeated for some (p) number of iterations and the best routes are updated every time the cost of some routes become lower than the best cost. 
