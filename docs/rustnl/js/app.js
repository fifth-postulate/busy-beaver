(function(){
  let slideshow = remark.create({
    sourceUrl: 'presentation.md'
  });
 
  // Setup MathJax
  MathJax.Hub.Config({
    tex2jax: {
      skipTags: ['script', 'noscript', 'style', 'textarea', 'pre']
    }
  });
  MathJax.Hub.Configured();

  const turingMachineDescriptionClass = 'turing-machine-description'
  slideshow.on('showSlide', function(slide){
    let descriptionPresent = slide.content.some(function(c){
      return c instanceof Object && c.class === turingMachineDescriptionClass;
    });
    if (descriptionPresent) {
      let slideIndex = slide.getSlideIndex();
      let parentSlide = document.getElementsByClassName('remark-slide')[slideIndex];
      parentSlide.getElementsByClassName(turingMachineDescriptionClass)
      .forEach(function(node){
        let description = node.innerText;
        let app = Elm.TM.init({
          node: node
        });
        app.ports.restart.send(description);
      });
    }
  });
})();
