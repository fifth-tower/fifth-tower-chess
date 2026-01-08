export default function myInitializer () {
    function loading_html(progress){
        let progress_str = `<div class="radial-progress" style="--value:${progress};" aria-valuenow="${progress}" role="progressbar">${progress}%</div>`;
      document.getElementById("tower_loading_").innerHTML=progress_str;
    }
  return {
    onStart: () => {
      loading_html(0)
      console.log("Loading...");
    },
    onProgress: ({current, total}) => {
      if (!total) {
        console.log("Loading...", current, "bytes");
      } else {
        let progress = Math.round((current/total) * 100);
        loading_html(progress)
        console.log("Loading...", Math.round((current/total) * 100), "%" )
      }
    },
    onComplete: () => {
      console.log("Loading... done!");
      let loading = document.getElementById("tower_loading_");
      loading.className = "";
      loading.innerHTML = "";
    },
    onSuccess: (wasm) => {
      console.log("Loading... successful!");
    },
    onFailure: (error) => {
      console.warn("Loading... failed!", error);
    }
  }
};