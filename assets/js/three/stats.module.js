var Stats=function(){var e=0,t=document.createElement("div");function l(e){return t.appendChild(e.dom),e}function n(l){for(var n=0;n<t.children.length;n++)t.children[n].style.display=n===l?"block":"none";e=l}t.style.cssText="position:fixed;top:0;left:0;cursor:pointer;opacity:0.9;z-index:10000",t.addEventListener("click",function(l){l.preventDefault(),n(++e%t.children.length)},!1);var i=(performance||Date).now(),a=i,$=0,o=l(new Stats.Panel("FPS","#0ff","#002")),r=l(new Stats.Panel("MS","#0f0","#020"));if(self.performance&&self.performance.memory)var f=l(new Stats.Panel("MB","#f08","#201"));return n(0),{REVISION:16,dom:t,addPanel:l,showPanel:n,begin:function(){i=(performance||Date).now()},end:function(){$++;var e=(performance||Date).now();if(r.update(e-i,200),e>=a+1e3&&(o.update(1e3*$/(e-a),100),a=e,$=0,f)){var t=performance.memory;f.update(t.usedJSHeapSize/1048576,t.jsHeapSizeLimit/1048576)}return e},update:function(){i=this.end()},domElement:t,setMode:n}};Stats.Panel=function(e,t,l){var n=1/0,i=0,a=Math.round,$=a(window.devicePixelRatio||1),o=80*$,r=48*$,f=3*$,d=2*$,c=3*$,s=15*$,p=74*$,u=30*$,S=document.createElement("canvas");S.width=o,S.height=r,S.style.cssText="width:80px;height:48px";var h=S.getContext("2d");return h.font="bold "+9*$+"px Helvetica,Arial,sans-serif",h.textBaseline="top",h.fillStyle=l,h.fillRect(0,0,o,r),h.fillStyle=t,h.fillText(e,f,d),h.fillRect(c,s,p,u),h.fillStyle=l,h.globalAlpha=.9,h.fillRect(c,s,p,u),{dom:S,update:function(r,m){n=Math.min(n,r),i=Math.max(i,r),h.fillStyle=l,h.globalAlpha=1,h.fillRect(0,0,o,s),h.fillStyle=t,h.fillText(a(r)+" "+e+" ("+a(n)+"-"+a(i)+")",f,d),h.drawImage(S,c+$,s,p-$,u,c,s,p-$,u),h.fillRect(c+p-$,s,$,u),h.fillStyle=l,h.globalAlpha=.9,h.fillRect(c+p-$,s,$,a((1-r/m)*u))}}};export default Stats;
