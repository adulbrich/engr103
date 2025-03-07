---
marp: true
style: |
  section h1 {
    color: #ea580c;
  }
paginate: true
---

<style>
    /* Adds total number of pages to footer */
    /* section::after {
        content: attr(data-marpit-pagination) ' / ' attr(data-marpit-pagination-total);
    } */
    img {
        display: block;
        margin: 0 auto;
    }

    section.reference p {
        font-size: 0.8rem;
        text-align: center;
    }
</style>

<!-- _paginate: false -->

# Special Topic: Machine Learning (ML) and Artificial Intelligence (AI)

ENGR 103: Engineering Computation and Algorithmic Thinking

Alex Ulbrich

---

# How many of you have used AI *today*?

<!-- FaceID, ChatGPT, Google Search, Voice Assistants, Smartphone Keyboard, Social Media, Video and Music Recommendations, Video Games, Assisted Driving, Smart Thermostats, Smartwatches, CAPTCHAs, ... -->

---

# Example

TODO
<!-- <video src="http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4" controls width="90%"></video> -->

---

# Applications

- Agriculture: optimal plant growth
- Banking: fraud detection
- Cybersecurity: attack detection and handling
- Civil: structural health monitoring
- Mechanical: predictive maintenance in machines
- Healthcare: medical diagnosis of patients
- Logistics: routing and optimization

---

# AI, ML, and Deep Learning

**Artificial Intelligence (AI)** refers to the capability of computational systems to perform tasks typically associated with human *intelligence* (learning, reasoning, problem-solving, perception, and decision-making).

**Machine Learning (ML)** is s subset of AI that learns patterns from data.

**Deep learning** is a subset of machine learning that focuses on utilizing *neural networks*.

![width:400px bg right:40%](https://upload.wikimedia.org/wikipedia/commons/thumb/1/1b/AI_hierarchy.svg/1920px-AI_hierarchy.svg.png)

<!-- Think of AI like teaching a baby. Show images of how an ML model learns to distinguish between dogs and cats. -->

---

<!-- _class: reference -->

![width:600px](grokking-machine-learning.jpg)
<p>Image from R. Hurbans, Grokking Artificial Intelligence Algorithms.</p>

---

<!-- _class: reference -->

# Basic ML Workflow

1. Collect data (images, sensor readings, text, ...)
2. Train a model (feeding examples)
3. Make predictions (on new data)
4. Improve over time (learning from mistakes)

![width:600px](grokking-ml-process.jpg)
<p>Image from R. Hurbans, Grokking Artificial Intelligence Algorithms.</p>

---

# Google Colab Exercise

<!-- use https://colab.research.google.com/ -->

---

# Worth Checking

- [AlphaGo Documentary](https://www.youtube.com/watch?v=WXuK6gekU1Y)
- [HuggingFace LeRobot](https://github.com/huggingface/lerobot)
- Conversational AI and Search
  - Grok, ChatGPT, Gemini, LeChat, Copilot
  - Perplexity
