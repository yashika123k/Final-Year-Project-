
# Machine Learning–Based Dynamic Clustering for Energy-Efficient WSN Topology

This project implements a simulation-based Wireless Sensor Network (WSN) environment where **machine learning is used for dynamic cluster-head selection** to improve energy efficiency and network lifetime.  
A **real-time visualization using Pygame** is included to demonstrate cluster formation and energy-aware behavior live.

---

## 📌 Problem Statement

Traditional static clustering protocols such as LEACH select cluster heads randomly and fail to adapt to:
- node energy imbalance
- heterogeneous node distribution
- dynamic traffic conditions

This leads to early node death and reduced network lifetime.

---

## 🎯 Objectives

- Simulate a wireless sensor network with energy constraints  
- Implement dynamic cluster-head (CH) selection using machine learning  
- Compare ML-based clustering with classical LEACH protocol  
- Analyze network lifetime, energy consumption, and throughput  
- Visualize clustering behavior using a live Pygame simulation  

---

## 🧠 Proposed Solution

The system uses **node-level features** such as:
- residual energy  
- node degree (number of neighbors)  
- communication cost (distance-based)  

A lightweight ML model (Decision Tree / Random Forest / K-Means) predicts **cluster-head suitability** dynamically in each round.

---

## 🛠️ Technologies Used

- **Python**
- **Scikit-learn** (ML models)
- **NumPy / Matplotlib** (simulation & plots)
- **Pygame** (real-time visualization)

---

## 📊 System Workflow

1. Random deployment of sensor nodes in a 2D field  
2. Energy-aware communication model (distance-based)  
3. Cluster-head selection using:
   - LEACH (baseline)
   - ML-based dynamic approach  
4. Multi-round simulation  
5. Performance evaluation and visualization  

---

## 🎮 Pygame Live Simulation

The Pygame interface displays:
- Sensor nodes as dots  
- Cluster heads in a different color  
- Communication links (node → cluster head)  
- Dead nodes removed from the screen  
- Base station position  

This provides an intuitive understanding of dynamic clustering behavior.

---

## 📈 Performance Metrics

- Network lifetime (alive nodes vs rounds)  
- First Node Dead (FND) / Last Node Dead (LND)  
- Total energy consumption  
- Throughput (packets delivered to base station)  

---

## 📂 Project Structure

