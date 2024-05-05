<script>
  import { navigate } from './store.js';

  async function goToCreate() {
    navigate("create-deliberation", {});
  }

  async function gotToAllCoordinations() {
    navigate("all-deliberations", {});
  }
</script>

<div class="instructions">

<div class="content">
  <div style="margin-left:auto;margin-right:auto;width:600px;">
      <h1>Welcome to <strong>Converge</strong></h1>
      <h3>Solve problems, resolve conflicts, brainstorm ideas</h3>
      <!-- <h3>Work together with less risk.</h3> -->
  
  <!-- <h3>Plan get-togethers, events or actions of any kind.</h3> -->
  
  
  <h2>How it works</h2>
  <ol>
    <li>Propose a topic for deliberation</li>
    <li>Invite stakeholders to participate</li>
    <li>Collaboratively build and rank a list of criteria</li>
    <li>Propose and evaluate solutions based on those criteria</li>
  </ol>

  <p>Get started and <button class="gotoButton" on:click={() => gotToAllCoordinations()}>View public deliberations</button> or <button class="gotoButton" on:click={() => goToCreate()}>Create a new deliberation</button></p>
  </div>
</div>
</div>

<style>
  .instructions {
      height: calc(100vh - 16px);
      width: calc(100vw - 12px);
      /* height: 100vh;
      width: 100vw; */
      top: 0;
      z-index: 0;
      position: absolute;
      background: url("./assets/fabio.jpg");
      background-size: cover;
      /* slow fade in background */
      -webkit-animation: fadein 1s; /* Safari, Chrome and Opera > 12.1 */
      -moz-animation: fadein 1s; /* Firefox < 16 */
      -ms-animation: fadein 1s; /* Internet Explorer */
      -o-animation: fadein 1s; /* Opera < 12.1 */
      animation: fadein 1s;
  }
  .instructions .content {
      /* margin: 0 auto;
      max-width: 800px;
      padding: 1em; */
      /* margin-left:auto;
      margin-right:auto;
      width:300px; */

      margin-top: 140px;
      width:100%; 
      position:relative;
      z-index:1;
      text-align: left;
      color: white;
  }

  .instructions li {
      /* list-style: none; */
      margin-top: 0.2em;
      /* margin-left: 20px; */
  }
  
  .instructions h1 {
      font-size: 3em;
      font-weight: 700;
      margin: 0;
      padding: 0;
  }

  .instructions h1 strong {
      /* color: #f5a623; */
      color: #d92ed9;
  }
  
  .instructions h2 {
      font-size: 2em;
      font-weight: 700;
      /* margin: 0; */
      margin: 0;
      margin-top: 24px;
      padding: 0;
      color: white;
  }
  
  .instructions h3 {
      font-size: 1.4em;
      font-weight: 700;
      /* margin: 0; */
      margin-top: 10px;
      padding: 0;
  }
  
  .instructions ol {
      margin-top: 7px;
      background-color: rgba(0, 0, 0, 0.4);
      padding: 7px 35px;
      border-radius: 4px;
  }
  
  .instructions li {
      font-size: 1.2em;
      /* margin: 0; */
      /* padding: 0; */
  }
  
  /* .instructions .content {
      margin: 0;
      padding: 0;
      margin-top: 200px;
  }
  
  .instructions article p {
      margin: 0;
      padding: 0;
  } */

  .gotoButton {
    border: 0;
    background: #d92ed9;
    padding: 6px;
    border-radius: 4px;
    color: white;
    cursor: pointer;
  }

  .gotoButton:hover {
    background: #d965d9;
  }
</style>