<script>
</script>

<div class="progress">
  <svg viewBox="0 0 100 100">
    <path d="M 75,50 a 25 25 0 1 1 0,-0.01" pathLength="100" stroke="currentColor" />
  </svg>
</div>

<style>
  .progress {
    color: rgb(170, 170, 170);
    width: 80px;
    height: 80px;
  }

  svg {
    scale: 160%;
  }

  path {
    stroke-width: 8;
    stroke-dasharray: 100;
    stroke-linecap: round;
    fill: none;
    transform-origin: 50px 50px;
    animation: pathStroke 2.1s infinite ease-in-out, pathRotate 2.4s infinite linear;
  }

  @keyframes pathStroke {
    0% {
      stroke-dashoffset: 100;
    }
    50% {
      stroke-dashoffset: 0;
    }
    100% {
      stroke-dashoffset: -100;
    }
  }

  @keyframes pathRotate {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>