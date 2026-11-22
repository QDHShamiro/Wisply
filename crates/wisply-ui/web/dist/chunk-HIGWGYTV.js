import{a as ia,b as $e,d as sr,h as re,k as Wt,l as lr,m as oa,n as ur,p as ce}from"./chunk-YIODUVYZ.js";var vt=ia((ic,sa)=>{"use strict";var pt=typeof Reflect=="object"?Reflect:null,dr=pt&&typeof pt.apply=="function"?pt.apply:function(n,t,e){return Function.prototype.apply.call(n,t,e)},vn;pt&&typeof pt.ownKeys=="function"?vn=pt.ownKeys:Object.getOwnPropertySymbols?vn=function(n){return Object.getOwnPropertyNames(n).concat(Object.getOwnPropertySymbols(n))}:vn=function(n){return Object.getOwnPropertyNames(n)};function Zo(a){console&&console.warn&&console.warn(a)}var cr=Number.isNaN||function(n){return n!==n};function de(){de.init.call(this)}sa.exports=de;sa.exports.once=ts;de.EventEmitter=de;de.prototype._events=void 0;de.prototype._eventsCount=0;de.prototype._maxListeners=void 0;var hr=10;function mn(a){if(typeof a!="function")throw new TypeError('The "listener" argument must be of type Function. Received type '+typeof a)}Object.defineProperty(de,"defaultMaxListeners",{enumerable:!0,get:function(){return hr},set:function(a){if(typeof a!="number"||a<0||cr(a))throw new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received '+a+".");hr=a}});de.init=function(){(this._events===void 0||this._events===Object.getPrototypeOf(this)._events)&&(this._events=Object.create(null),this._eventsCount=0),this._maxListeners=this._maxListeners||void 0};de.prototype.setMaxListeners=function(n){if(typeof n!="number"||n<0||cr(n))throw new RangeError('The value of "n" is out of range. It must be a non-negative number. Received '+n+".");return this._maxListeners=n,this};function fr(a){return a._maxListeners===void 0?de.defaultMaxListeners:a._maxListeners}de.prototype.getMaxListeners=function(){return fr(this)};de.prototype.emit=function(n){for(var t=[],e=1;e<arguments.length;e++)t.push(arguments[e]);var r=n==="error",i=this._events;if(i!==void 0)r=r&&i.error===void 0;else if(!r)return!1;if(r){var o;if(t.length>0&&(o=t[0]),o instanceof Error)throw o;var s=new Error("Unhandled error."+(o?" ("+o.message+")":""));throw s.context=o,s}var l=i[n];if(l===void 0)return!1;if(typeof l=="function")dr(l,this,t);else for(var u=l.length,d=br(l,u),e=0;e<u;++e)dr(d[e],this,t);return!0};function gr(a,n,t,e){var r,i,o;if(mn(t),i=a._events,i===void 0?(i=a._events=Object.create(null),a._eventsCount=0):(i.newListener!==void 0&&(a.emit("newListener",n,t.listener?t.listener:t),i=a._events),o=i[n]),o===void 0)o=i[n]=t,++a._eventsCount;else if(typeof o=="function"?o=i[n]=e?[t,o]:[o,t]:e?o.unshift(t):o.push(t),r=fr(a),r>0&&o.length>r&&!o.warned){o.warned=!0;var s=new Error("Possible EventEmitter memory leak detected. "+o.length+" "+String(n)+" listeners added. Use emitter.setMaxListeners() to increase limit");s.name="MaxListenersExceededWarning",s.emitter=a,s.type=n,s.count=o.length,Zo(s)}return a}de.prototype.addListener=function(n,t){return gr(this,n,t,!1)};de.prototype.on=de.prototype.addListener;de.prototype.prependListener=function(n,t){return gr(this,n,t,!0)};function Qo(){if(!this.fired)return this.target.removeListener(this.type,this.wrapFn),this.fired=!0,arguments.length===0?this.listener.call(this.target):this.listener.apply(this.target,arguments)}function pr(a,n,t){var e={fired:!1,wrapFn:void 0,target:a,type:n,listener:t},r=Qo.bind(e);return r.listener=t,e.wrapFn=r,r}de.prototype.once=function(n,t){return mn(t),this.on(n,pr(this,n,t)),this};de.prototype.prependOnceListener=function(n,t){return mn(t),this.prependListener(n,pr(this,n,t)),this};de.prototype.removeListener=function(n,t){var e,r,i,o,s;if(mn(t),r=this._events,r===void 0)return this;if(e=r[n],e===void 0)return this;if(e===t||e.listener===t)--this._eventsCount===0?this._events=Object.create(null):(delete r[n],r.removeListener&&this.emit("removeListener",n,e.listener||t));else if(typeof e!="function"){for(i=-1,o=e.length-1;o>=0;o--)if(e[o]===t||e[o].listener===t){s=e[o].listener,i=o;break}if(i<0)return this;i===0?e.shift():Jo(e,i),e.length===1&&(r[n]=e[0]),r.removeListener!==void 0&&this.emit("removeListener",n,s||t)}return this};de.prototype.off=de.prototype.removeListener;de.prototype.removeAllListeners=function(n){var t,e,r;if(e=this._events,e===void 0)return this;if(e.removeListener===void 0)return arguments.length===0?(this._events=Object.create(null),this._eventsCount=0):e[n]!==void 0&&(--this._eventsCount===0?this._events=Object.create(null):delete e[n]),this;if(arguments.length===0){var i=Object.keys(e),o;for(r=0;r<i.length;++r)o=i[r],o!=="removeListener"&&this.removeAllListeners(o);return this.removeAllListeners("removeListener"),this._events=Object.create(null),this._eventsCount=0,this}if(t=e[n],typeof t=="function")this.removeListener(n,t);else if(t!==void 0)for(r=t.length-1;r>=0;r--)this.removeListener(n,t[r]);return this};function vr(a,n,t){var e=a._events;if(e===void 0)return[];var r=e[n];return r===void 0?[]:typeof r=="function"?t?[r.listener||r]:[r]:t?es(r):br(r,r.length)}de.prototype.listeners=function(n){return vr(this,n,!0)};de.prototype.rawListeners=function(n){return vr(this,n,!1)};de.listenerCount=function(a,n){return typeof a.listenerCount=="function"?a.listenerCount(n):mr.call(a,n)};de.prototype.listenerCount=mr;function mr(a){var n=this._events;if(n!==void 0){var t=n[a];if(typeof t=="function")return 1;if(t!==void 0)return t.length}return 0}de.prototype.eventNames=function(){return this._eventsCount>0?vn(this._events):[]};function br(a,n){for(var t=new Array(n),e=0;e<n;++e)t[e]=a[e];return t}function Jo(a,n){for(;n+1<a.length;n++)a[n]=a[n+1];a.pop()}function es(a){for(var n=new Array(a.length),t=0;t<n.length;++t)n[t]=a[t].listener||a[t];return n}function ts(a,n){return new Promise(function(t,e){function r(o){a.removeListener(n,i),e(o)}function i(){typeof a.removeListener=="function"&&a.removeListener("error",r),t([].slice.call(arguments))}yr(a,n,i,{once:!0}),n!=="error"&&ns(a,r,{once:!0})})}function ns(a,n,t){typeof a.on=="function"&&yr(a,"error",n,t)}function yr(a,n,t,e){if(typeof a.on=="function")e.once?a.once(n,t):a.on(n,t);else if(typeof a.addEventListener=="function")a.addEventListener(n,function r(i){e.once&&a.removeEventListener(n,r),t(i)});else throw new TypeError('The "emitter" argument must be of type EventEmitter. Received type '+typeof a)}});var Ur=ia((uc,Wr)=>{Wr.exports=function(){var n,t,e={};(function(){var i=0,o=1,s=2,l=3,u=4,d=5,h=6,f=7,c=8,g=9,b=0,m=1,p=2,v=0,y=1,T=2,_=3,S=4,D=5,w=6,k=7,L=8,N=3,R=10,z=3,C=9,I=10;e.exports=function(F,x,P){var W,G,A,q,V,Y,pe,se,Z,ue,ve=x.length,De=P.length,Te=F.adjustSizes,Mt=F.barnesHutTheta*F.barnesHutTheta,Me,O,X,ee,he,Q,K,M=[];for(A=0;A<ve;A+=R)x[A+u]=x[A+s],x[A+d]=x[A+l],x[A+s]=0,x[A+l]=0;if(F.outboundAttractionDistribution){for(Me=0,A=0;A<ve;A+=R)Me+=x[A+h];Me/=ve/R}if(F.barnesHutOptimize){var Je=1/0,ct=-1/0,et=1/0,ft=-1/0,we,gt,ra;for(A=0;A<ve;A+=R)Je=Math.min(Je,x[A+i]),ct=Math.max(ct,x[A+i]),et=Math.min(et,x[A+o]),ft=Math.max(ft,x[A+o]);var dn=ct-Je,hn=ft-et;for(dn>hn?(et-=(dn-hn)/2,ft=et+dn):(Je-=(hn-dn)/2,ct=Je+hn),M[0+v]=-1,M[0+y]=(Je+ct)/2,M[0+T]=(et+ft)/2,M[0+_]=Math.max(ct-Je,ft-et),M[0+S]=-1,M[0+D]=-1,M[0+w]=0,M[0+k]=0,M[0+L]=0,W=1,A=0;A<ve;A+=R)for(G=0,ra=N;;)if(M[G+D]>=0){x[A+i]<M[G+y]?x[A+o]<M[G+T]?we=M[G+D]:we=M[G+D]+C:x[A+o]<M[G+T]?we=M[G+D]+C*2:we=M[G+D]+C*3,M[G+k]=(M[G+k]*M[G+w]+x[A+i]*x[A+h])/(M[G+w]+x[A+h]),M[G+L]=(M[G+L]*M[G+w]+x[A+o]*x[A+h])/(M[G+w]+x[A+h]),M[G+w]+=x[A+h],G=we;continue}else if(M[G+v]<0){M[G+v]=A;break}else{if(M[G+D]=W*C,se=M[G+_]/2,Z=M[G+D],M[Z+v]=-1,M[Z+y]=M[G+y]-se,M[Z+T]=M[G+T]-se,M[Z+_]=se,M[Z+S]=Z+C,M[Z+D]=-1,M[Z+w]=0,M[Z+k]=0,M[Z+L]=0,Z+=C,M[Z+v]=-1,M[Z+y]=M[G+y]-se,M[Z+T]=M[G+T]+se,M[Z+_]=se,M[Z+S]=Z+C,M[Z+D]=-1,M[Z+w]=0,M[Z+k]=0,M[Z+L]=0,Z+=C,M[Z+v]=-1,M[Z+y]=M[G+y]+se,M[Z+T]=M[G+T]-se,M[Z+_]=se,M[Z+S]=Z+C,M[Z+D]=-1,M[Z+w]=0,M[Z+k]=0,M[Z+L]=0,Z+=C,M[Z+v]=-1,M[Z+y]=M[G+y]+se,M[Z+T]=M[G+T]+se,M[Z+_]=se,M[Z+S]=M[G+S],M[Z+D]=-1,M[Z+w]=0,M[Z+k]=0,M[Z+L]=0,W+=4,x[M[G+v]+i]<M[G+y]?x[M[G+v]+o]<M[G+T]?we=M[G+D]:we=M[G+D]+C:x[M[G+v]+o]<M[G+T]?we=M[G+D]+C*2:we=M[G+D]+C*3,M[G+w]=x[M[G+v]+h],M[G+k]=x[M[G+v]+i],M[G+L]=x[M[G+v]+o],M[we+v]=M[G+v],M[G+v]=-1,x[A+i]<M[G+y]?x[A+o]<M[G+T]?gt=M[G+D]:gt=M[G+D]+C:x[A+o]<M[G+T]?gt=M[G+D]+C*2:gt=M[G+D]+C*3,we===gt)if(ra--){G=we;continue}else{ra=N;break}M[gt+v]=A;break}}if(F.barnesHutOptimize)for(O=F.scalingRatio,A=0;A<ve;A+=R)for(G=0;;)if(M[G+D]>=0)if(Q=Math.pow(x[A+i]-M[G+k],2)+Math.pow(x[A+o]-M[G+L],2),ue=M[G+_],4*ue*ue/Q<Mt){if(X=x[A+i]-M[G+k],ee=x[A+o]-M[G+L],Te===!0?Q>0?(K=O*x[A+h]*M[G+w]/Q,x[A+s]+=X*K,x[A+l]+=ee*K):Q<0&&(K=-O*x[A+h]*M[G+w]/Math.sqrt(Q),x[A+s]+=X*K,x[A+l]+=ee*K):Q>0&&(K=O*x[A+h]*M[G+w]/Q,x[A+s]+=X*K,x[A+l]+=ee*K),G=M[G+S],G<0)break;continue}else{G=M[G+D];continue}else{if(Y=M[G+v],Y>=0&&Y!==A&&(X=x[A+i]-x[Y+i],ee=x[A+o]-x[Y+o],Q=X*X+ee*ee,Te===!0?Q>0?(K=O*x[A+h]*x[Y+h]/Q,x[A+s]+=X*K,x[A+l]+=ee*K):Q<0&&(K=-O*x[A+h]*x[Y+h]/Math.sqrt(Q),x[A+s]+=X*K,x[A+l]+=ee*K):Q>0&&(K=O*x[A+h]*x[Y+h]/Q,x[A+s]+=X*K,x[A+l]+=ee*K)),G=M[G+S],G<0)break;continue}else for(O=F.scalingRatio,q=0;q<ve;q+=R)for(V=0;V<q;V+=R)X=x[q+i]-x[V+i],ee=x[q+o]-x[V+o],Te===!0?(Q=Math.sqrt(X*X+ee*ee)-x[q+c]-x[V+c],Q>0?(K=O*x[q+h]*x[V+h]/Q/Q,x[q+s]+=X*K,x[q+l]+=ee*K,x[V+s]-=X*K,x[V+l]-=ee*K):Q<0&&(K=100*O*x[q+h]*x[V+h],x[q+s]+=X*K,x[q+l]+=ee*K,x[V+s]-=X*K,x[V+l]-=ee*K)):(Q=Math.sqrt(X*X+ee*ee),Q>0&&(K=O*x[q+h]*x[V+h]/Q/Q,x[q+s]+=X*K,x[q+l]+=ee*K,x[V+s]-=X*K,x[V+l]-=ee*K));for(Z=F.gravity/F.scalingRatio,O=F.scalingRatio,A=0;A<ve;A+=R)K=0,X=x[A+i],ee=x[A+o],Q=Math.sqrt(Math.pow(X,2)+Math.pow(ee,2)),F.strongGravityMode?Q>0&&(K=O*x[A+h]*Z):Q>0&&(K=O*x[A+h]*Z/Q),x[A+s]-=X*K,x[A+l]-=ee*K;for(O=1*(F.outboundAttractionDistribution?Me:1),pe=0;pe<De;pe+=z)q=P[pe+b],V=P[pe+m],se=P[pe+p],he=Math.pow(se,F.edgeWeightInfluence),X=x[q+i]-x[V+i],ee=x[q+o]-x[V+o],Te===!0?(Q=Math.sqrt(X*X+ee*ee)-x[q+c]-x[V+c],F.linLogMode?F.outboundAttractionDistribution?Q>0&&(K=-O*he*Math.log(1+Q)/Q/x[q+h]):Q>0&&(K=-O*he*Math.log(1+Q)/Q):F.outboundAttractionDistribution?Q>0&&(K=-O*he/x[q+h]):Q>0&&(K=-O*he)):(Q=Math.sqrt(Math.pow(X,2)+Math.pow(ee,2)),F.linLogMode?F.outboundAttractionDistribution?Q>0&&(K=-O*he*Math.log(1+Q)/Q/x[q+h]):Q>0&&(K=-O*he*Math.log(1+Q)/Q):F.outboundAttractionDistribution?(Q=1,K=-O*he/x[q+h]):(Q=1,K=-O*he)),Q>0&&(x[q+s]+=X*K,x[q+l]+=ee*K,x[V+s]-=X*K,x[V+l]-=ee*K);var cn,Bt,fn,tt,gn,pn;if(Te===!0)for(A=0;A<ve;A+=R)x[A+g]!==1&&(cn=Math.sqrt(Math.pow(x[A+s],2)+Math.pow(x[A+l],2)),cn>I&&(x[A+s]=x[A+s]*I/cn,x[A+l]=x[A+l]*I/cn),Bt=x[A+h]*Math.sqrt((x[A+u]-x[A+s])*(x[A+u]-x[A+s])+(x[A+d]-x[A+l])*(x[A+d]-x[A+l])),fn=Math.sqrt((x[A+u]+x[A+s])*(x[A+u]+x[A+s])+(x[A+d]+x[A+l])*(x[A+d]+x[A+l]))/2,tt=.1*Math.log(1+fn)/(1+Math.sqrt(Bt)),gn=x[A+i]+x[A+s]*(tt/F.slowDown),x[A+i]=gn,pn=x[A+o]+x[A+l]*(tt/F.slowDown),x[A+o]=pn);else for(A=0;A<ve;A+=R)x[A+g]!==1&&(Bt=x[A+h]*Math.sqrt((x[A+u]-x[A+s])*(x[A+u]-x[A+s])+(x[A+d]-x[A+l])*(x[A+d]-x[A+l])),fn=Math.sqrt((x[A+u]+x[A+s])*(x[A+u]+x[A+s])+(x[A+d]+x[A+l])*(x[A+d]+x[A+l]))/2,tt=x[A+f]*Math.log(1+fn)/(1+Math.sqrt(Bt)),x[A+f]=Math.min(1,Math.sqrt(tt*(Math.pow(x[A+s],2)+Math.pow(x[A+l],2))/(1+Math.sqrt(Bt)))),gn=x[A+i]+x[A+s]*(tt/F.slowDown),x[A+i]=gn,pn=x[A+o]+x[A+l]*(tt/F.slowDown),x[A+o]=pn);return{}}})();var r=e.exports;self.addEventListener("message",function(i){var o=i.data;n=new Float32Array(o.nodes),o.edges&&(t=new Float32Array(o.edges)),r(o.settings,n,t),self.postMessage({nodes:n.buffer},[n.buffer])})}});var Pa=ia((Dc,Ci)=>{Ci.exports=function(n){return n!==null&&typeof n=="object"&&typeof n.addUndirectedEdgeWithKey=="function"&&typeof n.dropNode=="function"&&typeof n.multi=="boolean"}});var Sr=$e(vt(),1);function as(){let a=arguments[0];for(let n=1,t=arguments.length;n<t;n++)if(arguments[n])for(let e in arguments[n])a[e]=arguments[n][e];return a}var be=as;typeof Object.assign=="function"&&(be=Object.assign);function Le(a,n,t,e){let r=a._nodes.get(n),i=null;return r&&(e==="mixed"?i=r.out&&r.out[t]||r.undirected&&r.undirected[t]:e==="directed"?i=r.out&&r.out[t]:i=r.undirected&&r.undirected[t]),i}function xe(a){return typeof a=="object"&&a!==null}function Er(a){let n;for(n in a)return!1;return!0}function Re(a,n,t){Object.defineProperty(a,n,{enumerable:!1,configurable:!1,writable:!0,value:t})}function Ie(a,n,t){let e={enumerable:!0,configurable:!0};typeof t=="function"?e.get=t:(e.value=t,e.writable=!1),Object.defineProperty(a,n,e)}function xr(a){return!(!xe(a)||a.attributes&&!Array.isArray(a.attributes))}function rs(){let a=Math.floor(Math.random()*256)&255;return()=>a++}function Be(){let a=arguments,n=null,t=-1;return{[Symbol.iterator](){return this},next(){let e=null;do{if(n===null){if(t++,t>=a.length)return{done:!0};n=a[t][Symbol.iterator]()}if(e=n.next(),e.done){n=null;continue}break}while(!0);return e}}}function bt(){return{[Symbol.iterator](){return this},next(){return{done:!0}}}}var Ht=class extends Error{constructor(n){super(),this.name="GraphError",this.message=n}},j=class a extends Ht{constructor(n){super(n),this.name="InvalidArgumentsGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},$=class a extends Ht{constructor(n){super(n),this.name="NotFoundGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},te=class a extends Ht{constructor(n){super(n),this.name="UsageGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}};function Ar(a,n){this.key=a,this.attributes=n,this.clear()}Ar.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.undirectedDegree=0,this.undirectedLoops=0,this.directedLoops=0,this.in={},this.out={},this.undirected={}};function Dr(a,n){this.key=a,this.attributes=n,this.clear()}Dr.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.directedLoops=0,this.in={},this.out={}};function wr(a,n){this.key=a,this.attributes=n,this.clear()}wr.prototype.clear=function(){this.undirectedDegree=0,this.undirectedLoops=0,this.undirected={}};function yt(a,n,t,e,r){this.key=n,this.attributes=r,this.undirected=a,this.source=t,this.target=e}yt.prototype.attach=function(){let a="out",n="in";this.undirected&&(a=n="undirected");let t=this.source.key,e=this.target.key;this.source[a][e]=this,!(this.undirected&&t===e)&&(this.target[n][t]=this)};yt.prototype.attachMulti=function(){let a="out",n="in",t=this.source.key,e=this.target.key;this.undirected&&(a=n="undirected");let r=this.source[a],i=r[e];if(typeof i>"u"){r[e]=this,this.undirected&&t===e||(this.target[n][t]=this);return}i.previous=this,this.next=i,r[e]=this,this.target[n][t]=this};yt.prototype.detach=function(){let a=this.source.key,n=this.target.key,t="out",e="in";this.undirected&&(t=e="undirected"),delete this.source[t][n],delete this.target[e][a]};yt.prototype.detachMulti=function(){let a=this.source.key,n=this.target.key,t="out",e="in";this.undirected&&(t=e="undirected"),this.previous===void 0?this.next===void 0?(delete this.source[t][n],delete this.target[e][a]):(this.next.previous=void 0,this.source[t][n]=this.next,this.target[e][a]=this.next):(this.previous.next=this.next,this.next!==void 0&&(this.next.previous=this.previous))};var Rr=0,Lr=1,is=2,Cr=3;function We(a,n,t,e,r,i,o){let s,l,u,d;if(e=""+e,t===Rr){if(s=a._nodes.get(e),!s)throw new $(`Graph.${n}: could not find the "${e}" node in the graph.`);u=r,d=i}else if(t===Cr){if(r=""+r,l=a._edges.get(r),!l)throw new $(`Graph.${n}: could not find the "${r}" edge in the graph.`);let h=l.source.key,f=l.target.key;if(e===h)s=l.target;else if(e===f)s=l.source;else throw new $(`Graph.${n}: the "${e}" node is not attached to the "${r}" edge (${h}, ${f}).`);u=i,d=o}else{if(l=a._edges.get(e),!l)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`);t===Lr?s=l.source:s=l.target,u=r,d=i}return[s,u,d]}function os(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);return o.attributes[s]}}function ss(a,n,t){a.prototype[n]=function(e,r){let[i]=We(this,n,t,e,r);return i.attributes}}function ls(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);return o.attributes.hasOwnProperty(s)}}function us(a,n,t){a.prototype[n]=function(e,r,i,o){let[s,l,u]=We(this,n,t,e,r,i,o);return s.attributes[l]=u,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function ds(a,n,t){a.prototype[n]=function(e,r,i,o){let[s,l,u]=We(this,n,t,e,r,i,o);if(typeof u!="function")throw new j(`Graph.${n}: updater should be a function.`);let d=s.attributes,h=u(d[l]);return d[l]=h,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function hs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);return delete o.attributes[s],this.emit("nodeAttributesUpdated",{key:o.key,type:"remove",attributes:o.attributes,name:s}),this}}function cs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);if(!xe(s))throw new j(`Graph.${n}: provided attributes are not a plain object.`);return o.attributes=s,this.emit("nodeAttributesUpdated",{key:o.key,type:"replace",attributes:o.attributes}),this}}function fs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);if(!xe(s))throw new j(`Graph.${n}: provided attributes are not a plain object.`);return be(o.attributes,s),this.emit("nodeAttributesUpdated",{key:o.key,type:"merge",attributes:o.attributes,data:s}),this}}function gs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=We(this,n,t,e,r,i);if(typeof s!="function")throw new j(`Graph.${n}: provided updater is not a function.`);return o.attributes=s(o.attributes),this.emit("nodeAttributesUpdated",{key:o.key,type:"update",attributes:o.attributes}),this}}var ps=[{name:a=>`get${a}Attribute`,attacher:os},{name:a=>`get${a}Attributes`,attacher:ss},{name:a=>`has${a}Attribute`,attacher:ls},{name:a=>`set${a}Attribute`,attacher:us},{name:a=>`update${a}Attribute`,attacher:ds},{name:a=>`remove${a}Attribute`,attacher:hs},{name:a=>`replace${a}Attributes`,attacher:cs},{name:a=>`merge${a}Attributes`,attacher:fs},{name:a=>`update${a}Attributes`,attacher:gs}];function vs(a){ps.forEach(function({name:n,attacher:t}){t(a,n("Node"),Rr),t(a,n("Source"),Lr),t(a,n("Target"),is),t(a,n("Opposite"),Cr)})}function ms(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return i.attributes[r]}}function bs(a,n,t){a.prototype[n]=function(e){let r;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>1){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let i=""+e,o=""+arguments[1];if(r=Le(this,i,o,t),!r)throw new $(`Graph.${n}: could not find an edge for the given path ("${i}" - "${o}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,r=this._edges.get(e),!r)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return r.attributes}}function ys(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return i.attributes.hasOwnProperty(r)}}function xs(a,n,t){a.prototype[n]=function(e,r,i){let o;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+e,l=""+r;if(r=arguments[2],i=arguments[3],o=Le(this,s,l,t),!o)throw new $(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,o=this._edges.get(e),!o)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return o.attributes[r]=i,this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function _s(a,n,t){a.prototype[n]=function(e,r,i){let o;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+e,l=""+r;if(r=arguments[2],i=arguments[3],o=Le(this,s,l,t),!o)throw new $(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,o=this._edges.get(e),!o)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(typeof i!="function")throw new j(`Graph.${n}: updater should be a function.`);return o.attributes[r]=i(o.attributes[r]),this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function Ts(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return delete i.attributes[r],this.emit("edgeAttributesUpdated",{key:i.key,type:"remove",attributes:i.attributes,name:r}),this}}function Ss(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(!xe(r))throw new j(`Graph.${n}: provided attributes are not a plain object.`);return i.attributes=r,this.emit("edgeAttributesUpdated",{key:i.key,type:"replace",attributes:i.attributes}),this}}function Es(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(!xe(r))throw new j(`Graph.${n}: provided attributes are not a plain object.`);return be(i.attributes,r),this.emit("edgeAttributesUpdated",{key:i.key,type:"merge",attributes:i.attributes,data:r}),this}}function As(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new te(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new te(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Le(this,o,s,t),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new te(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new $(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(typeof r!="function")throw new j(`Graph.${n}: provided updater is not a function.`);return i.attributes=r(i.attributes),this.emit("edgeAttributesUpdated",{key:i.key,type:"update",attributes:i.attributes}),this}}var Ds=[{name:a=>`get${a}Attribute`,attacher:ms},{name:a=>`get${a}Attributes`,attacher:bs},{name:a=>`has${a}Attribute`,attacher:ys},{name:a=>`set${a}Attribute`,attacher:xs},{name:a=>`update${a}Attribute`,attacher:_s},{name:a=>`remove${a}Attribute`,attacher:Ts},{name:a=>`replace${a}Attributes`,attacher:Ss},{name:a=>`merge${a}Attributes`,attacher:Es},{name:a=>`update${a}Attributes`,attacher:As}];function ws(a){Ds.forEach(function({name:n,attacher:t}){t(a,n("Edge"),"mixed"),t(a,n("DirectedEdge"),"directed"),t(a,n("UndirectedEdge"),"undirected")})}var Rs=[{name:"edges",type:"mixed"},{name:"inEdges",type:"directed",direction:"in"},{name:"outEdges",type:"directed",direction:"out"},{name:"inboundEdges",type:"mixed",direction:"in"},{name:"outboundEdges",type:"mixed",direction:"out"},{name:"directedEdges",type:"directed"},{name:"undirectedEdges",type:"undirected"}];function Ls(a,n,t,e){let r=!1;for(let i in n){if(i===e)continue;let o=n[i];if(r=t(o.key,o.attributes,o.source.key,o.target.key,o.source.attributes,o.target.attributes,o.undirected),a&&r)return o.key}}function Cs(a,n,t,e){let r,i,o,s=!1;for(let l in n)if(l!==e){r=n[l];do{if(i=r.source,o=r.target,s=t(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected),a&&s)return r.key;r=r.next}while(r!==void 0)}}function la(a,n){let t=Object.keys(a),e=t.length,r,i=0;return{[Symbol.iterator](){return this},next(){do if(r)r=r.next;else{if(i>=e)return{done:!0};let o=t[i++];if(o===n){r=void 0;continue}r=a[o]}while(!r);return{done:!1,value:{edge:r.key,attributes:r.attributes,source:r.source.key,target:r.target.key,sourceAttributes:r.source.attributes,targetAttributes:r.target.attributes,undirected:r.undirected}}}}}function ks(a,n,t,e){let r=n[t];if(!r)return;let i=r.source,o=r.target;if(e(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected)&&a)return r.key}function Ps(a,n,t,e){let r=n[t];if(!r)return;let i=!1;do{if(i=e(r.key,r.attributes,r.source.key,r.target.key,r.source.attributes,r.target.attributes,r.undirected),a&&i)return r.key;r=r.next}while(r!==void 0)}function ua(a,n){let t=a[n];if(t.next!==void 0)return{[Symbol.iterator](){return this},next(){if(!t)return{done:!0};let r={edge:t.key,attributes:t.attributes,source:t.source.key,target:t.target.key,sourceAttributes:t.source.attributes,targetAttributes:t.target.attributes,undirected:t.undirected};return t=t.next,{done:!1,value:r}}};let e=!1;return{[Symbol.iterator](){return this},next(){return e===!0?{done:!0}:(e=!0,{done:!1,value:{edge:t.key,attributes:t.attributes,source:t.source.key,target:t.target.key,sourceAttributes:t.source.attributes,targetAttributes:t.target.attributes,undirected:t.undirected}})}}}function Fs(a,n){if(a.size===0)return[];if(n==="mixed"||n===a.type)return Array.from(a._edges.keys());let t=n==="undirected"?a.undirectedSize:a.directedSize,e=new Array(t),r=n==="undirected",i=a._edges.values(),o=0,s,l;for(;s=i.next(),s.done!==!0;)l=s.value,l.undirected===r&&(e[o++]=l.key);return e}function kr(a,n,t,e){if(n.size===0)return;let r=t!=="mixed"&&t!==n.type,i=t==="undirected",o,s,l=!1,u=n._edges.values();for(;o=u.next(),o.done!==!0;){if(s=o.value,r&&s.undirected!==i)continue;let{key:d,attributes:h,source:f,target:c}=s;if(l=e(d,h,f.key,c.key,f.attributes,c.attributes,s.undirected),a&&l)return d}}function Is(a,n){if(a.size===0)return bt();let t=n!=="mixed"&&n!==a.type,e=n==="undirected",r=a._edges.values();return{[Symbol.iterator](){return this},next(){let i,o;for(;;){if(i=r.next(),i.done)return i;if(o=i.value,!(t&&o.undirected!==e))break}return{value:{edge:o.key,attributes:o.attributes,source:o.source.key,target:o.target.key,sourceAttributes:o.source.attributes,targetAttributes:o.target.attributes,undirected:o.undirected},done:!1}}}}function da(a,n,t,e,r,i){let o=n?Cs:Ls,s;if(t!=="undirected"&&(e!=="out"&&(s=o(a,r.in,i),a&&s)||e!=="in"&&(s=o(a,r.out,i,e?void 0:r.key),a&&s))||t!=="directed"&&(s=o(a,r.undirected,i),a&&s))return s}function Gs(a,n,t,e){let r=[];return da(!1,a,n,t,e,function(i){r.push(i)}),r}function Ns(a,n,t){let e=bt();return a!=="undirected"&&(n!=="out"&&typeof t.in<"u"&&(e=Be(e,la(t.in))),n!=="in"&&typeof t.out<"u"&&(e=Be(e,la(t.out,n?void 0:t.key)))),a!=="directed"&&typeof t.undirected<"u"&&(e=Be(e,la(t.undirected))),e}function ha(a,n,t,e,r,i,o){let s=t?Ps:ks,l;if(n!=="undirected"&&(typeof r.in<"u"&&e!=="out"&&(l=s(a,r.in,i,o),a&&l)||typeof r.out<"u"&&e!=="in"&&(e||r.key!==i)&&(l=s(a,r.out,i,o),a&&l))||n!=="directed"&&typeof r.undirected<"u"&&(l=s(a,r.undirected,i,o),a&&l))return l}function zs(a,n,t,e,r){let i=[];return ha(!1,a,n,t,e,r,function(o){i.push(o)}),i}function Ms(a,n,t,e){let r=bt();return a!=="undirected"&&(typeof t.in<"u"&&n!=="out"&&e in t.in&&(r=Be(r,ua(t.in,e))),typeof t.out<"u"&&n!=="in"&&e in t.out&&(n||t.key!==e)&&(r=Be(r,ua(t.out,e)))),a!=="directed"&&typeof t.undirected<"u"&&e in t.undirected&&(r=Be(r,ua(t.undirected,e))),r}function Bs(a,n){let{name:t,type:e,direction:r}=n;a.prototype[t]=function(i,o){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return[];if(!arguments.length)return Fs(this,e);if(arguments.length===1){i=""+i;let s=this._nodes.get(i);if(typeof s>"u")throw new $(`Graph.${t}: could not find the "${i}" node in the graph.`);return Gs(this.multi,e==="mixed"?this.type:e,r,s)}if(arguments.length===2){i=""+i,o=""+o;let s=this._nodes.get(i);if(!s)throw new $(`Graph.${t}:  could not find the "${i}" source node in the graph.`);if(!this._nodes.has(o))throw new $(`Graph.${t}:  could not find the "${o}" target node in the graph.`);return zs(e,this.multi,r,s,o)}throw new j(`Graph.${t}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function Ws(a,n){let{name:t,type:e,direction:r}=n,i="forEach"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(u,d,h){if(!(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)){if(arguments.length===1)return h=u,kr(!1,this,e,h);if(arguments.length===2){u=""+u,h=d;let f=this._nodes.get(u);if(typeof f>"u")throw new $(`Graph.${i}: could not find the "${u}" node in the graph.`);return da(!1,this.multi,e==="mixed"?this.type:e,r,f,h)}if(arguments.length===3){u=""+u,d=""+d;let f=this._nodes.get(u);if(!f)throw new $(`Graph.${i}:  could not find the "${u}" source node in the graph.`);if(!this._nodes.has(d))throw new $(`Graph.${i}:  could not find the "${d}" target node in the graph.`);return ha(!1,e,this.multi,r,f,d,h)}throw new j(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)}};let o="map"+t[0].toUpperCase()+t.slice(1);a.prototype[o]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),h;if(u.length===0){let f=0;e!=="directed"&&(f+=this.undirectedSize),e!=="undirected"&&(f+=this.directedSize),h=new Array(f);let c=0;u.push((g,b,m,p,v,y,T)=>{h[c++]=d(g,b,m,p,v,y,T)})}else h=[],u.push((f,c,g,b,m,p,v)=>{h.push(d(f,c,g,b,m,p,v))});return this[i].apply(this,u),h};let s="filter"+t[0].toUpperCase()+t.slice(1);a.prototype[s]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),h=[];return u.push((f,c,g,b,m,p,v)=>{d(f,c,g,b,m,p,v)&&h.push(f)}),this[i].apply(this,u),h};let l="reduce"+t[0].toUpperCase()+t.slice(1);a.prototype[l]=function(){let u=Array.prototype.slice.call(arguments);if(u.length<2||u.length>4)throw new j(`Graph.${l}: invalid number of arguments (expecting 2, 3 or 4 and got ${u.length}).`);if(typeof u[u.length-1]=="function"&&typeof u[u.length-2]!="function")throw new j(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let d,h;u.length===2?(d=u[0],h=u[1],u=[]):u.length===3?(d=u[1],h=u[2],u=[u[0]]):u.length===4&&(d=u[2],h=u[3],u=[u[0],u[1]]);let f=h;return u.push((c,g,b,m,p,v,y)=>{f=d(f,c,g,b,m,p,v,y)}),this[i].apply(this,u),f}}function Us(a,n){let{name:t,type:e,direction:r}=n,i="find"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(l,u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return!1;if(arguments.length===1)return d=l,kr(!0,this,e,d);if(arguments.length===2){l=""+l,d=u;let h=this._nodes.get(l);if(typeof h>"u")throw new $(`Graph.${i}: could not find the "${l}" node in the graph.`);return da(!0,this.multi,e==="mixed"?this.type:e,r,h,d)}if(arguments.length===3){l=""+l,u=""+u;let h=this._nodes.get(l);if(!h)throw new $(`Graph.${i}:  could not find the "${l}" source node in the graph.`);if(!this._nodes.has(u))throw new $(`Graph.${i}:  could not find the "${u}" target node in the graph.`);return ha(!0,e,this.multi,r,h,u,d)}throw new j(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)};let o="some"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[o]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((h,f,c,g,b,m,p)=>u(h,f,c,g,b,m,p)),!!this[i].apply(this,l)};let s="every"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[s]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((h,f,c,g,b,m,p)=>!u(h,f,c,g,b,m,p)),!this[i].apply(this,l)}}function Os(a,n){let{name:t,type:e,direction:r}=n,i=t.slice(0,-1)+"Entries";a.prototype[i]=function(o,s){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return bt();if(!arguments.length)return Is(this,e);if(arguments.length===1){o=""+o;let l=this._nodes.get(o);if(!l)throw new $(`Graph.${i}: could not find the "${o}" node in the graph.`);return Ns(e,r,l)}if(arguments.length===2){o=""+o,s=""+s;let l=this._nodes.get(o);if(!l)throw new $(`Graph.${i}:  could not find the "${o}" source node in the graph.`);if(!this._nodes.has(s))throw new $(`Graph.${i}:  could not find the "${s}" target node in the graph.`);return Ms(e,r,l,s)}throw new j(`Graph.${i}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function Hs(a){Rs.forEach(n=>{Bs(a,n),Ws(a,n),Us(a,n),Os(a,n)})}var $s=[{name:"neighbors",type:"mixed"},{name:"inNeighbors",type:"directed",direction:"in"},{name:"outNeighbors",type:"directed",direction:"out"},{name:"inboundNeighbors",type:"mixed",direction:"in"},{name:"outboundNeighbors",type:"mixed",direction:"out"},{name:"directedNeighbors",type:"directed"},{name:"undirectedNeighbors",type:"undirected"}];function En(){this.A=null,this.B=null}En.prototype.wrap=function(a){this.A===null?this.A=a:this.B===null&&(this.B=a)};En.prototype.has=function(a){return this.A!==null&&a in this.A||this.B!==null&&a in this.B};function Ut(a,n,t,e,r){for(let i in e){let o=e[i],s=o.source,l=o.target,u=s===t?l:s;if(n&&n.has(u.key))continue;let d=r(u.key,u.attributes);if(a&&d)return u.key}}function ca(a,n,t,e,r){if(n!=="mixed"){if(n==="undirected")return Ut(a,null,e,e.undirected,r);if(typeof t=="string")return Ut(a,null,e,e[t],r)}let i=new En,o;if(n!=="undirected"){if(t!=="out"){if(o=Ut(a,null,e,e.in,r),a&&o)return o;i.wrap(e.in)}if(t!=="in"){if(o=Ut(a,i,e,e.out,r),a&&o)return o;i.wrap(e.out)}}if(n!=="directed"&&(o=Ut(a,i,e,e.undirected,r),a&&o))return o}function Vs(a,n,t){if(a!=="mixed"){if(a==="undirected")return Object.keys(t.undirected);if(typeof n=="string")return Object.keys(t[n])}let e=[];return ca(!1,a,n,t,function(r){e.push(r)}),e}function Ot(a,n,t){let e=Object.keys(t),r=e.length,i=0;return{[Symbol.iterator](){return this},next(){let o=null;do{if(i>=r)return a&&a.wrap(t),{done:!0};let s=t[e[i++]],l=s.source,u=s.target;if(o=l===n?u:l,a&&a.has(o.key)){o=null;continue}}while(o===null);return{done:!1,value:{neighbor:o.key,attributes:o.attributes}}}}}function js(a,n,t){if(a!=="mixed"){if(a==="undirected")return Ot(null,t,t.undirected);if(typeof n=="string")return Ot(null,t,t[n])}let e=bt(),r=new En;return a!=="undirected"&&(n!=="out"&&(e=Be(e,Ot(r,t,t.in))),n!=="in"&&(e=Be(e,Ot(r,t,t.out)))),a!=="directed"&&(e=Be(e,Ot(r,t,t.undirected))),e}function Xs(a,n){let{name:t,type:e,direction:r}=n;a.prototype[t]=function(i){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return[];i=""+i;let o=this._nodes.get(i);if(typeof o>"u")throw new $(`Graph.${t}: could not find the "${i}" node in the graph.`);return Vs(e==="mixed"?this.type:e,r,o)}}function qs(a,n){let{name:t,type:e,direction:r}=n,i="forEach"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return;u=""+u;let h=this._nodes.get(u);if(typeof h>"u")throw new $(`Graph.${i}: could not find the "${u}" node in the graph.`);ca(!1,e==="mixed"?this.type:e,r,h,d)};let o="map"+t[0].toUpperCase()+t.slice(1);a.prototype[o]=function(u,d){let h=[];return this[i](u,(f,c)=>{h.push(d(f,c))}),h};let s="filter"+t[0].toUpperCase()+t.slice(1);a.prototype[s]=function(u,d){let h=[];return this[i](u,(f,c)=>{d(f,c)&&h.push(f)}),h};let l="reduce"+t[0].toUpperCase()+t.slice(1);a.prototype[l]=function(u,d,h){if(arguments.length<3)throw new j(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let f=h;return this[i](u,(c,g)=>{f=d(f,c,g)}),f}}function Ks(a,n){let{name:t,type:e,direction:r}=n,i=t[0].toUpperCase()+t.slice(1,-1),o="find"+i;a.prototype[o]=function(u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return;u=""+u;let h=this._nodes.get(u);if(typeof h>"u")throw new $(`Graph.${o}: could not find the "${u}" node in the graph.`);return ca(!0,e==="mixed"?this.type:e,r,h,d)};let s="some"+i;a.prototype[s]=function(u,d){return!!this[o](u,d)};let l="every"+i;a.prototype[l]=function(u,d){return!this[o](u,(f,c)=>!d(f,c))}}function Ys(a,n){let{name:t,type:e,direction:r}=n,i=t.slice(0,-1)+"Entries";a.prototype[i]=function(o){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return bt();o=""+o;let s=this._nodes.get(o);if(typeof s>"u")throw new $(`Graph.${i}: could not find the "${o}" node in the graph.`);return js(e==="mixed"?this.type:e,r,s)}}function Zs(a){$s.forEach(n=>{Xs(a,n),qs(a,n),Ks(a,n),Ys(a,n)})}function bn(a,n,t,e,r){let i=e._nodes.values(),o=e.type,s,l,u,d,h,f,c;for(;s=i.next(),s.done!==!0;){let g=!1;if(l=s.value,o!=="undirected"){d=l.out;for(u in d){h=d[u];do{if(f=h.target,g=!0,c=r(l.key,f.key,l.attributes,f.attributes,h.key,h.attributes,h.undirected),a&&c)return h;h=h.next}while(h)}}if(o!=="directed"){d=l.undirected;for(u in d)if(!(n&&l.key>u)){h=d[u];do{if(f=h.target,f.key!==u&&(f=h.source),g=!0,c=r(l.key,f.key,l.attributes,f.attributes,h.key,h.attributes,h.undirected),a&&c)return h;h=h.next}while(h)}}if(t&&!g&&(c=r(l.key,null,l.attributes,null,null,null,null),a&&c))return null}}function Qs(a,n){let t={key:a};return Er(n.attributes)||(t.attributes=be({},n.attributes)),t}function Js(a,n,t){let e={key:n,source:t.source.key,target:t.target.key};return Er(t.attributes)||(e.attributes=be({},t.attributes)),a==="mixed"&&t.undirected&&(e.undirected=!0),e}function el(a){if(!xe(a))throw new j('Graph.import: invalid serialized node. A serialized node should be a plain object with at least a "key" property.');if(!("key"in a))throw new j("Graph.import: serialized node is missing its key.");if("attributes"in a&&(!xe(a.attributes)||a.attributes===null))throw new j("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.")}function tl(a){if(!xe(a))throw new j('Graph.import: invalid serialized edge. A serialized edge should be a plain object with at least a "source" & "target" property.');if(!("source"in a))throw new j("Graph.import: serialized edge is missing its source.");if(!("target"in a))throw new j("Graph.import: serialized edge is missing its target.");if("attributes"in a&&(!xe(a.attributes)||a.attributes===null))throw new j("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.");if("undirected"in a&&typeof a.undirected!="boolean")throw new j("Graph.import: invalid undirectedness information. Undirected should be boolean or omitted.")}var nl=rs(),al=new Set(["directed","undirected","mixed"]),_r=new Set(["domain","_events","_eventsCount","_maxListeners"]),rl=[{name:a=>`${a}Edge`,generateKey:!0},{name:a=>`${a}DirectedEdge`,generateKey:!0,type:"directed"},{name:a=>`${a}UndirectedEdge`,generateKey:!0,type:"undirected"},{name:a=>`${a}EdgeWithKey`},{name:a=>`${a}DirectedEdgeWithKey`,type:"directed"},{name:a=>`${a}UndirectedEdgeWithKey`,type:"undirected"}],il={allowSelfLoops:!0,multi:!1,type:"mixed"};function ol(a,n,t){if(t&&!xe(t))throw new j(`Graph.addNode: invalid attributes. Expecting an object but got "${t}"`);if(n=""+n,t=t||{},a._nodes.has(n))throw new te(`Graph.addNode: the "${n}" node already exist in the graph.`);let e=new a.NodeDataClass(n,t);return a._nodes.set(n,e),a.emit("nodeAdded",{key:n,attributes:t}),e}function Tr(a,n,t){let e=new a.NodeDataClass(n,t);return a._nodes.set(n,e),a.emit("nodeAdded",{key:n,attributes:t}),e}function Pr(a,n,t,e,r,i,o,s){if(!e&&a.type==="undirected")throw new te(`Graph.${n}: you cannot add a directed edge to an undirected graph. Use the #.addEdge or #.addUndirectedEdge instead.`);if(e&&a.type==="directed")throw new te(`Graph.${n}: you cannot add an undirected edge to a directed graph. Use the #.addEdge or #.addDirectedEdge instead.`);if(s&&!xe(s))throw new j(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`);if(i=""+i,o=""+o,s=s||{},!a.allowSelfLoops&&i===o)throw new te(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let l=a._nodes.get(i),u=a._nodes.get(o);if(!l)throw new $(`Graph.${n}: source node "${i}" not found.`);if(!u)throw new $(`Graph.${n}: target node "${o}" not found.`);let d={key:null,undirected:e,source:i,target:o,attributes:s};if(t)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new te(`Graph.${n}: the "${r}" edge already exists in the graph.`);if(!a.multi&&(e?typeof l.undirected[o]<"u":typeof l.out[o]<"u"))throw new te(`Graph.${n}: an edge linking "${i}" to "${o}" already exists. If you really want to add multiple edges linking those nodes, you should create a multi graph by using the 'multi' option.`);let h=new yt(e,r,l,u,s);a._edges.set(r,h);let f=i===o;return e?(l.undirectedDegree++,u.undirectedDegree++,f&&(l.undirectedLoops++,a._undirectedSelfLoopCount++)):(l.outDegree++,u.inDegree++,f&&(l.directedLoops++,a._directedSelfLoopCount++)),a.multi?h.attachMulti():h.attach(),e?a._undirectedSize++:a._directedSize++,d.key=r,a.emit("edgeAdded",d),r}function sl(a,n,t,e,r,i,o,s,l){if(!e&&a.type==="undirected")throw new te(`Graph.${n}: you cannot merge/update a directed edge to an undirected graph. Use the #.mergeEdge/#.updateEdge or #.addUndirectedEdge instead.`);if(e&&a.type==="directed")throw new te(`Graph.${n}: you cannot merge/update an undirected edge to a directed graph. Use the #.mergeEdge/#.updateEdge or #.addDirectedEdge instead.`);if(s){if(l){if(typeof s!="function")throw new j(`Graph.${n}: invalid updater function. Expecting a function but got "${s}"`)}else if(!xe(s))throw new j(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`)}i=""+i,o=""+o;let u;if(l&&(u=s,s=void 0),!a.allowSelfLoops&&i===o)throw new te(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let d=a._nodes.get(i),h=a._nodes.get(o),f,c;if(!t&&(f=a._edges.get(r),f)){if((f.source.key!==i||f.target.key!==o)&&(!e||f.source.key!==o||f.target.key!==i))throw new te(`Graph.${n}: inconsistency detected when attempting to merge the "${r}" edge with "${i}" source & "${o}" target vs. ("${f.source.key}", "${f.target.key}").`);c=f}if(!c&&!a.multi&&d&&(c=e?d.undirected[o]:d.out[o]),c){let v=[c.key,!1,!1,!1];if(l?!u:!s)return v;if(l){let y=c.attributes;c.attributes=u(y),a.emit("edgeAttributesUpdated",{type:"replace",key:c.key,attributes:c.attributes})}else be(c.attributes,s),a.emit("edgeAttributesUpdated",{type:"merge",key:c.key,attributes:c.attributes,data:s});return v}s=s||{},l&&u&&(s=u(s));let g={key:null,undirected:e,source:i,target:o,attributes:s};if(t)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new te(`Graph.${n}: the "${r}" edge already exists in the graph.`);let b=!1,m=!1;d||(d=Tr(a,i,{}),b=!0,i===o&&(h=d,m=!0)),h||(h=Tr(a,o,{}),m=!0),f=new yt(e,r,d,h,s),a._edges.set(r,f);let p=i===o;return e?(d.undirectedDegree++,h.undirectedDegree++,p&&(d.undirectedLoops++,a._undirectedSelfLoopCount++)):(d.outDegree++,h.inDegree++,p&&(d.directedLoops++,a._directedSelfLoopCount++)),a.multi?f.attachMulti():f.attach(),e?a._undirectedSize++:a._directedSize++,g.key=r,a.emit("edgeAdded",g),[r,!0,b,m]}function mt(a,n){a._edges.delete(n.key);let{source:t,target:e,attributes:r}=n,i=n.undirected,o=t===e;i?(t.undirectedDegree--,e.undirectedDegree--,o&&(t.undirectedLoops--,a._undirectedSelfLoopCount--)):(t.outDegree--,e.inDegree--,o&&(t.directedLoops--,a._directedSelfLoopCount--)),a.multi?n.detachMulti():n.detach(),i?a._undirectedSize--:a._directedSize--,a.emit("edgeDropped",{key:n.key,attributes:r,source:t.key,target:e.key,undirected:i})}var le=class a extends Sr.EventEmitter{constructor(n){if(super(),n=be({},il,n),typeof n.multi!="boolean")throw new j(`Graph.constructor: invalid 'multi' option. Expecting a boolean but got "${n.multi}".`);if(!al.has(n.type))throw new j(`Graph.constructor: invalid 'type' option. Should be one of "mixed", "directed" or "undirected" but got "${n.type}".`);if(typeof n.allowSelfLoops!="boolean")throw new j(`Graph.constructor: invalid 'allowSelfLoops' option. Expecting a boolean but got "${n.allowSelfLoops}".`);let t=n.type==="mixed"?Ar:n.type==="directed"?Dr:wr;Re(this,"NodeDataClass",t);let e="geid_"+nl()+"_",r=0,i=()=>{let o;do o=e+r++;while(this._edges.has(o));return o};Re(this,"_attributes",{}),Re(this,"_nodes",new Map),Re(this,"_edges",new Map),Re(this,"_directedSize",0),Re(this,"_undirectedSize",0),Re(this,"_directedSelfLoopCount",0),Re(this,"_undirectedSelfLoopCount",0),Re(this,"_edgeKeyGenerator",i),Re(this,"_options",n),_r.forEach(o=>Re(this,o,this[o])),Ie(this,"order",()=>this._nodes.size),Ie(this,"size",()=>this._edges.size),Ie(this,"directedSize",()=>this._directedSize),Ie(this,"undirectedSize",()=>this._undirectedSize),Ie(this,"selfLoopCount",()=>this._directedSelfLoopCount+this._undirectedSelfLoopCount),Ie(this,"directedSelfLoopCount",()=>this._directedSelfLoopCount),Ie(this,"undirectedSelfLoopCount",()=>this._undirectedSelfLoopCount),Ie(this,"multi",this._options.multi),Ie(this,"type",this._options.type),Ie(this,"allowSelfLoops",this._options.allowSelfLoops),Ie(this,"implementation",()=>"graphology")}_resetInstanceCounters(){this._directedSize=0,this._undirectedSize=0,this._directedSelfLoopCount=0,this._undirectedSelfLoopCount=0}hasNode(n){return this._nodes.has(""+n)}hasDirectedEdge(n,t){if(this.type==="undirected")return!1;if(arguments.length===1){let e=""+n,r=this._edges.get(e);return!!r&&!r.undirected}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?e.out.hasOwnProperty(t):!1}throw new j(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasUndirectedEdge(n,t){if(this.type==="directed")return!1;if(arguments.length===1){let e=""+n,r=this._edges.get(e);return!!r&&r.undirected}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?e.undirected.hasOwnProperty(t):!1}throw new j(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasEdge(n,t){if(arguments.length===1){let e=""+n;return this._edges.has(e)}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?typeof e.out<"u"&&e.out.hasOwnProperty(t)||typeof e.undirected<"u"&&e.undirected.hasOwnProperty(t):!1}throw new j(`Graph.hasEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}directedEdge(n,t){if(this.type==="undirected")return;if(n=""+n,t=""+t,this.multi)throw new te("Graph.directedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.directedEdges instead.");let e=this._nodes.get(n);if(!e)throw new $(`Graph.directedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new $(`Graph.directedEdge: could not find the "${t}" target node in the graph.`);let r=e.out&&e.out[t]||void 0;if(r)return r.key}undirectedEdge(n,t){if(this.type==="directed")return;if(n=""+n,t=""+t,this.multi)throw new te("Graph.undirectedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.undirectedEdges instead.");let e=this._nodes.get(n);if(!e)throw new $(`Graph.undirectedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new $(`Graph.undirectedEdge: could not find the "${t}" target node in the graph.`);let r=e.undirected&&e.undirected[t]||void 0;if(r)return r.key}edge(n,t){if(this.multi)throw new te("Graph.edge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.edges instead.");n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.edge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new $(`Graph.edge: could not find the "${t}" target node in the graph.`);let r=e.out&&e.out[t]||e.undirected&&e.undirected[t]||void 0;if(r)return r.key}areDirectedNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areDirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.in||t in e.out}areOutNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areOutNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.out}areInNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areInNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.in}areUndirectedNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areUndirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="directed"?!1:t in e.undirected}areNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&(t in e.in||t in e.out)||this.type!=="directed"&&t in e.undirected}areInboundNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areInboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&t in e.in||this.type!=="directed"&&t in e.undirected}areOutboundNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new $(`Graph.areOutboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&t in e.out||this.type!=="directed"&&t in e.undirected}inDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.inDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree}outDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.outDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.outDegree}directedDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.directedDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree+t.outDegree}undirectedDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.undirectedDegree: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:t.undirectedDegree}inboundDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.inboundDegree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.inDegree),e}outboundDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.outboundDegree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.outDegree),e}degree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.degree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.inDegree+t.outDegree),e}inDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.inDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree-t.directedLoops}outDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.outDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.outDegree-t.directedLoops}directedDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.directedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree+t.outDegree-t.directedLoops*2}undirectedDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.undirectedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:t.undirectedDegree-t.undirectedLoops*2}inboundDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.inboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.inDegree,r+=t.directedLoops),e-r}outboundDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.outboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.outDegree,r+=t.directedLoops),e-r}degreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.degreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.inDegree+t.outDegree,r+=t.directedLoops*2),e-r}source(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.source: could not find the "${n}" edge in the graph.`);return t.source.key}target(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.target: could not find the "${n}" edge in the graph.`);return t.target.key}extremities(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.extremities: could not find the "${n}" edge in the graph.`);return[t.source.key,t.target.key]}opposite(n,t){n=""+n,t=""+t;let e=this._edges.get(t);if(!e)throw new $(`Graph.opposite: could not find the "${t}" edge in the graph.`);let r=e.source.key,i=e.target.key;if(n===r)return i;if(n===i)return r;throw new $(`Graph.opposite: the "${n}" node is not attached to the "${t}" edge (${r}, ${i}).`)}hasExtremity(n,t){n=""+n,t=""+t;let e=this._edges.get(n);if(!e)throw new $(`Graph.hasExtremity: could not find the "${n}" edge in the graph.`);return e.source.key===t||e.target.key===t}isUndirected(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.isUndirected: could not find the "${n}" edge in the graph.`);return t.undirected}isDirected(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.isDirected: could not find the "${n}" edge in the graph.`);return!t.undirected}isSelfLoop(n){n=""+n;let t=this._edges.get(n);if(!t)throw new $(`Graph.isSelfLoop: could not find the "${n}" edge in the graph.`);return t.source===t.target}addNode(n,t){return ol(this,n,t).key}mergeNode(n,t){if(t&&!xe(t))throw new j(`Graph.mergeNode: invalid attributes. Expecting an object but got "${t}"`);n=""+n,t=t||{};let e=this._nodes.get(n);return e?(t&&(be(e.attributes,t),this.emit("nodeAttributesUpdated",{type:"merge",key:n,attributes:e.attributes,data:t})),[n,!1]):(e=new this.NodeDataClass(n,t),this._nodes.set(n,e),this.emit("nodeAdded",{key:n,attributes:t}),[n,!0])}updateNode(n,t){if(t&&typeof t!="function")throw new j(`Graph.updateNode: invalid updater function. Expecting a function but got "${t}"`);n=""+n;let e=this._nodes.get(n);if(e){if(t){let i=e.attributes;e.attributes=t(i),this.emit("nodeAttributesUpdated",{type:"replace",key:n,attributes:e.attributes})}return[n,!1]}let r=t?t({}):{};return e=new this.NodeDataClass(n,r),this._nodes.set(n,e),this.emit("nodeAdded",{key:n,attributes:r}),[n,!0]}dropNode(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new $(`Graph.dropNode: could not find the "${n}" node in the graph.`);let e;if(this.type!=="undirected"){for(let r in t.out){e=t.out[r];do mt(this,e),e=e.next;while(e)}for(let r in t.in){e=t.in[r];do mt(this,e),e=e.next;while(e)}}if(this.type!=="directed")for(let r in t.undirected){e=t.undirected[r];do mt(this,e),e=e.next;while(e)}this._nodes.delete(n),this.emit("nodeDropped",{key:n,attributes:t.attributes})}dropEdge(n){let t;if(arguments.length>1){let e=""+arguments[0],r=""+arguments[1];if(t=Le(this,e,r,this.type),!t)throw new $(`Graph.dropEdge: could not find the "${e}" -> "${r}" edge in the graph.`)}else if(n=""+n,t=this._edges.get(n),!t)throw new $(`Graph.dropEdge: could not find the "${n}" edge in the graph.`);return mt(this,t),this}dropDirectedEdge(n,t){if(arguments.length<2)throw new te("Graph.dropDirectedEdge: it does not make sense to try and drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new te("Graph.dropDirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");n=""+n,t=""+t;let e=Le(this,n,t,"directed");if(!e)throw new $(`Graph.dropDirectedEdge: could not find a "${n}" -> "${t}" edge in the graph.`);return mt(this,e),this}dropUndirectedEdge(n,t){if(arguments.length<2)throw new te("Graph.dropUndirectedEdge: it does not make sense to drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new te("Graph.dropUndirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");let e=Le(this,n,t,"undirected");if(!e)throw new $(`Graph.dropUndirectedEdge: could not find a "${n}" -> "${t}" edge in the graph.`);return mt(this,e),this}clear(){this._edges.clear(),this._nodes.clear(),this._resetInstanceCounters(),this.emit("cleared")}clearEdges(){let n=this._nodes.values(),t;for(;t=n.next(),t.done!==!0;)t.value.clear();this._edges.clear(),this._resetInstanceCounters(),this.emit("edgesCleared")}getAttribute(n){return this._attributes[n]}getAttributes(){return this._attributes}hasAttribute(n){return this._attributes.hasOwnProperty(n)}setAttribute(n,t){return this._attributes[n]=t,this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}updateAttribute(n,t){if(typeof t!="function")throw new j("Graph.updateAttribute: updater should be a function.");let e=this._attributes[n];return this._attributes[n]=t(e),this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}removeAttribute(n){return delete this._attributes[n],this.emit("attributesUpdated",{type:"remove",attributes:this._attributes,name:n}),this}replaceAttributes(n){if(!xe(n))throw new j("Graph.replaceAttributes: provided attributes are not a plain object.");return this._attributes=n,this.emit("attributesUpdated",{type:"replace",attributes:this._attributes}),this}mergeAttributes(n){if(!xe(n))throw new j("Graph.mergeAttributes: provided attributes are not a plain object.");return be(this._attributes,n),this.emit("attributesUpdated",{type:"merge",attributes:this._attributes,data:n}),this}updateAttributes(n){if(typeof n!="function")throw new j("Graph.updateAttributes: provided updater is not a function.");return this._attributes=n(this._attributes),this.emit("attributesUpdated",{type:"update",attributes:this._attributes}),this}updateEachNodeAttributes(n,t){if(typeof n!="function")throw new j("Graph.updateEachNodeAttributes: expecting an updater function.");if(t&&!xr(t))throw new j("Graph.updateEachNodeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let e=this._nodes.values(),r,i;for(;r=e.next(),r.done!==!0;)i=r.value,i.attributes=n(i.key,i.attributes);this.emit("eachNodeAttributesUpdated",{hints:t||null})}updateEachEdgeAttributes(n,t){if(typeof n!="function")throw new j("Graph.updateEachEdgeAttributes: expecting an updater function.");if(t&&!xr(t))throw new j("Graph.updateEachEdgeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let e=this._edges.values(),r,i,o,s;for(;r=e.next(),r.done!==!0;)i=r.value,o=i.source,s=i.target,i.attributes=n(i.key,i.attributes,o.key,s.key,o.attributes,s.attributes,i.undirected);this.emit("eachEdgeAttributesUpdated",{hints:t||null})}forEachAdjacencyEntry(n){if(typeof n!="function")throw new j("Graph.forEachAdjacencyEntry: expecting a callback.");bn(!1,!1,!1,this,n)}forEachAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new j("Graph.forEachAdjacencyEntryWithOrphans: expecting a callback.");bn(!1,!1,!0,this,n)}forEachAssymetricAdjacencyEntry(n){if(typeof n!="function")throw new j("Graph.forEachAssymetricAdjacencyEntry: expecting a callback.");bn(!1,!0,!1,this,n)}forEachAssymetricAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new j("Graph.forEachAssymetricAdjacencyEntryWithOrphans: expecting a callback.");bn(!1,!0,!0,this,n)}nodes(){return Array.from(this._nodes.keys())}forEachNode(n){if(typeof n!="function")throw new j("Graph.forEachNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)r=e.value,n(r.key,r.attributes)}findNode(n){if(typeof n!="function")throw new j("Graph.findNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,n(r.key,r.attributes))return r.key}mapNodes(n){if(typeof n!="function")throw new j("Graph.mapNode: expecting a callback.");let t=this._nodes.values(),e,r,i=new Array(this.order),o=0;for(;e=t.next(),e.done!==!0;)r=e.value,i[o++]=n(r.key,r.attributes);return i}someNode(n){if(typeof n!="function")throw new j("Graph.someNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,n(r.key,r.attributes))return!0;return!1}everyNode(n){if(typeof n!="function")throw new j("Graph.everyNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,!n(r.key,r.attributes))return!1;return!0}filterNodes(n){if(typeof n!="function")throw new j("Graph.filterNodes: expecting a callback.");let t=this._nodes.values(),e,r,i=[];for(;e=t.next(),e.done!==!0;)r=e.value,n(r.key,r.attributes)&&i.push(r.key);return i}reduceNodes(n,t){if(typeof n!="function")throw new j("Graph.reduceNodes: expecting a callback.");if(arguments.length<2)throw new j("Graph.reduceNodes: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.");let e=t,r=this._nodes.values(),i,o;for(;i=r.next(),i.done!==!0;)o=i.value,e=n(e,o.key,o.attributes);return e}nodeEntries(){let n=this._nodes.values();return{[Symbol.iterator](){return this},next(){let t=n.next();if(t.done)return t;let e=t.value;return{value:{node:e.key,attributes:e.attributes},done:!1}}}}export(){let n=new Array(this._nodes.size),t=0;this._nodes.forEach((r,i)=>{n[t++]=Qs(i,r)});let e=new Array(this._edges.size);return t=0,this._edges.forEach((r,i)=>{e[t++]=Js(this.type,i,r)}),{options:{type:this.type,multi:this.multi,allowSelfLoops:this.allowSelfLoops},attributes:this.getAttributes(),nodes:n,edges:e}}import(n,t=!1){if(n instanceof a)return n.forEachNode((l,u)=>{t?this.mergeNode(l,u):this.addNode(l,u)}),n.forEachEdge((l,u,d,h,f,c,g)=>{t?g?this.mergeUndirectedEdgeWithKey(l,d,h,u):this.mergeDirectedEdgeWithKey(l,d,h,u):g?this.addUndirectedEdgeWithKey(l,d,h,u):this.addDirectedEdgeWithKey(l,d,h,u)}),this;if(!xe(n))throw new j("Graph.import: invalid argument. Expecting a serialized graph or, alternatively, a Graph instance.");if(n.attributes){if(!xe(n.attributes))throw new j("Graph.import: invalid attributes. Expecting a plain object.");t?this.mergeAttributes(n.attributes):this.replaceAttributes(n.attributes)}let e,r,i,o,s;if(n.nodes){if(i=n.nodes,!Array.isArray(i))throw new j("Graph.import: invalid nodes. Expecting an array.");for(e=0,r=i.length;e<r;e++){o=i[e],el(o);let{key:l,attributes:u}=o;t?this.mergeNode(l,u):this.addNode(l,u)}}if(n.edges){let l=!1;if(this.type==="undirected"&&(l=!0),i=n.edges,!Array.isArray(i))throw new j("Graph.import: invalid edges. Expecting an array.");for(e=0,r=i.length;e<r;e++){s=i[e],tl(s);let{source:u,target:d,attributes:h,undirected:f=l}=s,c;"key"in s?(c=t?f?this.mergeUndirectedEdgeWithKey:this.mergeDirectedEdgeWithKey:f?this.addUndirectedEdgeWithKey:this.addDirectedEdgeWithKey,c.call(this,s.key,u,d,h)):(c=t?f?this.mergeUndirectedEdge:this.mergeDirectedEdge:f?this.addUndirectedEdge:this.addDirectedEdge,c.call(this,u,d,h))}}return this}nullCopy(n){let t=new a(be({},this._options,n));return t.replaceAttributes(be({},this.getAttributes())),t}emptyCopy(n){let t=this.nullCopy(n);return this._nodes.forEach((e,r)=>{let i=be({},e.attributes);e=new t.NodeDataClass(r,i),t._nodes.set(r,e)}),t}copy(n){if(n=n||{},typeof n.type=="string"&&n.type!==this.type&&n.type!=="mixed")throw new te(`Graph.copy: cannot create an incompatible copy from "${this.type}" type to "${n.type}" because this would mean losing information about the current graph.`);if(typeof n.multi=="boolean"&&n.multi!==this.multi&&n.multi!==!0)throw new te("Graph.copy: cannot create an incompatible copy by downgrading a multi graph to a simple one because this would mean losing information about the current graph.");if(typeof n.allowSelfLoops=="boolean"&&n.allowSelfLoops!==this.allowSelfLoops&&n.allowSelfLoops!==!0)throw new te("Graph.copy: cannot create an incompatible copy from a graph allowing self loops to one that does not because this would mean losing information about the current graph.");let t=this.emptyCopy(n),e=this._edges.values(),r,i;for(;r=e.next(),r.done!==!0;)i=r.value,Pr(t,"copy",!1,i.undirected,i.key,i.source.key,i.target.key,be({},i.attributes));return t}toJSON(){return this.export()}toString(){return"[object Graph]"}inspect(){let n={};this._nodes.forEach((i,o)=>{n[o]=i.attributes});let t={},e={};this._edges.forEach((i,o)=>{let s=i.undirected?"--":"->",l="",u=i.source.key,d=i.target.key,h;i.undirected&&u>d&&(h=u,u=d,d=h);let f=`(${u})${s}(${d})`;o.startsWith("geid_")?this.multi&&(typeof e[f]>"u"?e[f]=0:e[f]++,l+=`${e[f]}. `):l+=`[${o}]: `,l+=f,t[l]=i.attributes});let r={};for(let i in this)this.hasOwnProperty(i)&&!_r.has(i)&&typeof this[i]!="function"&&typeof i!="symbol"&&(r[i]=this[i]);return r.attributes=this._attributes,r.nodes=n,r.edges=t,Re(r,"constructor",this.constructor),r}};typeof Symbol<"u"&&(le.prototype[Symbol.for("nodejs.util.inspect.custom")]=le.prototype.inspect);rl.forEach(a=>{["add","merge","update"].forEach(n=>{let t=a.name(n),e=n==="add"?Pr:sl;a.generateKey?le.prototype[t]=function(r,i,o){return e(this,t,!0,(a.type||this.type)==="undirected",null,r,i,o,n==="update")}:le.prototype[t]=function(r,i,o,s){return e(this,t,!1,(a.type||this.type)==="undirected",r,i,o,s,n==="update")}})});vs(le);ws(le);Hs(le);Zs(le);var yn=class extends le{constructor(n){let t=be({type:"directed"},n);if("multi"in t&&t.multi!==!1)throw new j("DirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(t.type!=="directed")throw new j('DirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},xn=class extends le{constructor(n){let t=be({type:"undirected"},n);if("multi"in t&&t.multi!==!1)throw new j("UndirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(t.type!=="undirected")throw new j('UndirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},_n=class extends le{constructor(n){let t=be({multi:!0},n);if("multi"in t&&t.multi!==!0)throw new j("MultiGraph.from: inconsistent indication that the graph should be simple in given options!");super(t)}},Tn=class extends le{constructor(n){let t=be({type:"directed",multi:!0},n);if("multi"in t&&t.multi!==!0)throw new j("MultiDirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(t.type!=="directed")throw new j('MultiDirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},Sn=class extends le{constructor(n){let t=be({type:"undirected",multi:!0},n);if("multi"in t&&t.multi!==!0)throw new j("MultiUndirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(t.type!=="undirected")throw new j('MultiUndirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}};function xt(a){a.from=function(n,t){let e=be({},n.options,t),r=new a(e);return r.import(n),r}}xt(le);xt(yn);xt(xn);xt(_n);xt(Tn);xt(Sn);le.Graph=le;le.DirectedGraph=yn;le.UndirectedGraph=xn;le.MultiGraph=_n;le.MultiDirectedGraph=Tn;le.MultiUndirectedGraph=Sn;le.InvalidArgumentsGraphError=j;le.NotFoundGraphError=$;le.UsageGraphError=te;var Fr=["#4f7cff","#ff3d8b","#12c99b","#ff9f1a","#a259ff","#ff5c39","#1bb8f0","#8bc34a","#e040fb","#00bfa5"],Ir=["note","project","session"],Gr=3e3,fa="wisply.graph.v3:",ll=3,ul=15;function Nr(a){try{return JSON.parse(localStorage.getItem(fa+a)??"{}")}catch{return{}}}function zr(a,n){let t={};n.forEachNode((e,r)=>{t[e]=[Math.round(r.x*10)/10,Math.round(r.y*10)/10]});try{localStorage.setItem(fa+a,JSON.stringify(t))}catch{localStorage.removeItem(fa+a)}}function Mr(a,n){let t=0;return a.forEachNode((e,r)=>{let i=n[e];if(i){r.x=i[0],r.y=i[1];return}t++;let o=a.neighbors(e).map(s=>n[s]).filter(s=>!!s);o.length&&(r.x=o.reduce((s,l)=>s+l[0],0)/o.length+(Math.random()-.5)*4,r.y=o.reduce((s,l)=>s+l[1],0)/o.length+(Math.random()-.5)*4)}),t}function dl(a){let n=new Map(a.nodes().map(t=>[t,t]));for(let t=0;t<ul;t++){let e=!1;if(a.forEachNode(r=>{let i=new Map;a.forEachNeighbor(r,l=>{let u=n.get(l);i.set(u,(i.get(u)??0)+1)});let o=n.get(r),s=0;for(let[l,u]of i)(u>s||u===s&&l<o)&&(o=l,s=u);s>0&&o!==n.get(r)&&(n.set(r,o),e=!0)}),!e)break}return n}function Br(a,n){let t=new le({type:"undirected",multi:!1,allowSelfLoops:!1}),e=new Array(a.nodes.length).fill(0);for(let[d,h]of a.edges)e[d]++,e[h]++;a.nodes.forEach(([d,h,f,c,g],b)=>{let m=f;t.mergeNode(d,{label:h||d,tag:"",kind:m,group:c,cluster:"",t:g,x:(Math.random()-.5)*100,y:(Math.random()-.5)*100,size:m==="project"?3+Math.sqrt(e[b])*.5:1.2+Math.sqrt(e[b])*.5,color:n,deg:e[b]})});for(let[d,h]of a.edges){let f=a.nodes[d][0],c=a.nodes[h][0];f!==c&&!t.hasEdge(f,c)&&t.addEdge(f,c)}let i=new Set(a.nodes.map(d=>d[3]).filter(Boolean)).size>=2?null:dl(t);t.forEachNode((d,h)=>{h.cluster=i?i.get(d):h.group});let o=new Map;t.forEachNode((d,h)=>o.set(h.cluster,(o.get(h.cluster)??0)+1));let s=[...o.entries()].filter(([d,h])=>d&&h>=ll).sort((d,h)=>h[1]-d[1]).slice(0,Fr.length),l=new Map,u=new Map;return s.forEach(([d],h)=>{if(l.set(d,Fr[h]),i){let f=t.filterNodes((c,g)=>g.cluster===d).sort((c,g)=>t.getNodeAttribute(g,"deg")-t.getNodeAttribute(c,"deg"))[0];u.set(d,t.getNodeAttribute(f,"label"))}else u.set(d,d)}),t.forEachNode((d,h)=>{h.color=l.get(h.cluster)??n}),{g:t,colors:l,names:u}}var Vr=$e(Ur(),1);var ga=10,Or=3,hl=500,cl=400,fl=60,Hr=8e3,gl=900,pl=16,vl=4,$r={linLogMode:!1,outboundAttractionDistribution:!1,adjustSizes:!1,edgeWeightInfluence:1,strongGravityMode:!0,slowDown:2,barnesHutTheta:.5};function jr(a,n){let t=n>=a;return a<=Gr?{settings:{...$r,scalingRatio:14,gravity:.6,barnesHutOptimize:a>hl},rounds:t?cl:fl,ms:Hr}:{settings:{...$r,scalingRatio:10,gravity:.05,barnesHutOptimize:!0},rounds:1/0,ms:t?Math.min(Hr,1500+a/3):gl}}function ml(a){let n=new Float32Array(a.order*ga),t=new Float32Array(a.size*Or),e=new Map,r=0;return a.forEachNode((i,o)=>{e.set(i,r),n[r]=o.x,n[r+1]=o.y,n[r+6]=1,n[r+7]=1,n[r+8]=o.size||1,r+=ga}),r=0,a.forEachEdge((i,o,s,l)=>{let u=e.get(s),d=e.get(l);n[u+6]++,n[d+6]++,t[r]=u,t[r+1]=d,t[r+2]=1,r+=Or}),{nodes:n,edges:t}}function bl(a,n){let t=0;a.forEachNode((e,r)=>{r.x=n[t],r.y=n[t+1],t+=ga})}function Xr(a,n,t,e){let{nodes:r,edges:i}=ml(a),o=URL.createObjectURL(new Blob([`(${Vr.default.toString()}).call(this);`],{type:"text/javascript"})),s=new Worker(o),l=()=>URL.revokeObjectURL(o);s.addEventListener("message",l,{once:!0});let u=performance.now()+n.ms,d=0,h=0;return s.onmessage=f=>{let c=new Float32Array(f.data.nodes),g=performance.now();d++;let b=d>=n.rounds||g>=u,m=b||g>=h;if(m&&bl(a,c),b?s.terminate():s.postMessage({settings:n.settings,nodes:c.buffer},[c.buffer]),m){t();let p=performance.now();h=p+Math.max(pl,(p-g)*vl)}b&&e()},s.postMessage({settings:n.settings,nodes:r.buffer,edges:i.buffer},[r.buffer,i.buffer]),()=>{l(),s.terminate()}}function qr(a,n){if(typeof requestIdleCallback=="function"){let e=requestIdleCallback(a,{timeout:n});return()=>cancelIdleCallback(e)}let t=setTimeout(a,n);return()=>clearTimeout(t)}var Kr={off:!1,named:!1,mute:!1},Yr={off:!1},Zr={hush:!1};function Qr(a,n){let t=new le({type:a.type,multi:a.multi,allowSelfLoops:a.allowSelfLoops});return{top:t,spot:(r,i)=>{t.order&&t.clear();let o=(s,l)=>{!n.has(s)&&!t.hasNode(s)&&t.addNode(s,{...a.getNodeAttributes(s),pin:l})};return r!==null&&(o(r,!0),a.forEachNeighbor(r,s=>o(s,!1)),a.forEachEdge(r,(s,l,u,d)=>{t.hasNode(u)&&t.hasNode(d)&&t.addEdgeWithKey(s,u,d)})),i!==null&&o(i,!0),t.order}}}function Jr(a,n){let t=new Set;return{gone:t,sift:r=>{let i=[],o=[];a.forEachNode((h,f)=>{let c=!r(f);c!==t.has(h)&&(c?(t.add(h),i.push(h)):(t.delete(h),o.push(h)))});let s=i.concat(o);if(!s.length)return s;n.setNodesState(i,{off:!0}),n.setNodesState(o,{off:!1});let l=new Set,u=[],d=[];for(let h of s)a.forEachEdge(h,(f,c,g,b)=>{l.has(f)||(l.add(f),(t.has(g)||t.has(b)?u:d).push(f))});return n.setEdgesState(u,{off:!0}),n.setEdgesState(d,{off:!1}),s}}}function wn(a,n){(n==null||n>a.length)&&(n=a.length);for(var t=0,e=Array(n);t<n;t++)e[t]=a[t];return e}function Rn(a,n){if(a){if(typeof a=="string")return wn(a,n);var t={}.toString.call(a).slice(8,-1);return t==="Object"&&a.constructor&&(t=a.constructor.name),t==="Map"||t==="Set"?Array.from(a):t==="Arguments"||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(t)?wn(a,n):void 0}}function yl(a){if(Array.isArray(a))return a}function xl(a,n){var t=a==null?null:typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(t!=null){var e,r,i,o,s=[],l=!0,u=!1;try{if(i=(t=t.call(a)).next,n===0){if(Object(t)!==t)return;l=!1}else for(;!(l=(e=i.call(t)).done)&&(s.push(e.value),s.length!==n);l=!0);}catch(d){u=!0,r=d}finally{try{if(!l&&t.return!=null&&(o=t.return(),Object(o)!==o))return}finally{if(u)throw r}}return s}}function _l(){throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function oe(a,n){return yl(a)||xl(a,n)||Rn(a,n)||_l()}function U(a,n){var t=typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(!t){if(Array.isArray(a)||(t=Rn(a))||n&&a&&typeof a.length=="number"){t&&(a=t);var e=0,r=function(){};return{s:r,n:function(){return e>=a.length?{done:!0}:{done:!1,value:a[e++]}},e:function(l){throw l},f:r}}throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}var i,o=!0,s=!1;return{s:function(){t=t.call(a)},n:function(){var l=t.next();return o=l.done,l},e:function(l){s=!0,i=l},f:function(){try{o||t.return==null||t.return()}finally{if(s)throw i}}}}function Ce(){return Float32Array.of(1,0,0,0,1,0,0,0,1)}function An(a,n,t){return a[0]=n,a[4]=typeof t=="number"?t:n,a}function ei(a,n){var t=Math.sin(n),e=Math.cos(n);return a[0]=e,a[1]=t,a[3]=-t,a[4]=e,a}function ti(a,n,t){return a[6]=n,a[7]=t,a}function Ve(a,n){var t=a[0],e=a[1],r=a[2],i=a[3],o=a[4],s=a[5],l=a[6],u=a[7],d=a[8],h=n[0],f=n[1],c=n[2],g=n[3],b=n[4],m=n[5],p=n[6],v=n[7],y=n[8];return a[0]=h*t+f*i+c*l,a[1]=h*e+f*o+c*u,a[2]=h*r+f*s+c*d,a[3]=g*t+b*i+m*l,a[4]=g*e+b*o+m*u,a[5]=g*r+b*s+m*d,a[6]=p*t+v*i+y*l,a[7]=p*e+v*o+y*u,a[8]=p*r+v*s+y*d,a}function pa(a,n){var t=Math.cos(n),e=Math.sin(n);return{x:t*a.x+e*a.y,y:-e*a.x+t*a.y}}function Tt(a,n){var t=arguments.length>2&&arguments[2]!==void 0?arguments[2]:1,e=a[0],r=a[1],i=a[3],o=a[4],s=a[6],l=a[7],u=n.x,d=n.y;return{x:u*e+d*i+s*t,y:u*r+d*o+l*t}}function Tl(a,n){var t=a.height/a.width,e=n.height/n.width;return t<1&&e>1||t>1&&e<1?1:Math.min(Math.max(e,1/e),Math.max(1/t,t))}function je(a,n,t,e,r){var i=a.angle,o=a.ratio,s=a.x,l=a.y,u=n.width,d=n.height,h=Ce(),f=Math.min(u,d)-2*e,c=Tl(n,t);return r?(Ve(h,ti(Ce(),s,l)),Ve(h,An(Ce(),o)),Ve(h,ei(Ce(),i)),Ve(h,An(Ce(),u/f/2/c,d/f/2/c))):(Ve(h,An(Ce(),2*(f/u)*c,2*(f/d)*c)),Ve(h,ei(Ce(),-i)),Ve(h,An(Ce(),1/o)),Ve(h,ti(Ce(),-s,-l))),h}function ni(a,n,t){var e=Tt(a,{x:Math.cos(n.angle),y:Math.sin(n.angle)},0),r=e.x,i=e.y;return 1/Math.sqrt(Math.pow(r,2)+Math.pow(i,2))/t.width}function va(a,n,t,e,r){var i=Math.floor(n/r*e),o=Math.floor(a.drawingBufferHeight/r-t/r*e);return[i,o]}var Vt={black:"#000000",silver:"#C0C0C0",gray:"#808080",grey:"#808080",white:"#FFFFFF",maroon:"#800000",red:"#FF0000",purple:"#800080",fuchsia:"#FF00FF",green:"#008000",lime:"#00FF00",olive:"#808000",yellow:"#FFFF00",navy:"#000080",blue:"#0000FF",teal:"#008080",aqua:"#00FFFF",darkblue:"#00008B",mediumblue:"#0000CD",darkgreen:"#006400",darkcyan:"#008B8B",deepskyblue:"#00BFFF",darkturquoise:"#00CED1",mediumspringgreen:"#00FA9A",springgreen:"#00FF7F",cyan:"#00FFFF",midnightblue:"#191970",dodgerblue:"#1E90FF",lightseagreen:"#20B2AA",forestgreen:"#228B22",seagreen:"#2E8B57",darkslategray:"#2F4F4F",darkslategrey:"#2F4F4F",limegreen:"#32CD32",mediumseagreen:"#3CB371",turquoise:"#40E0D0",royalblue:"#4169E1",steelblue:"#4682B4",darkslateblue:"#483D8B",mediumturquoise:"#48D1CC",indigo:"#4B0082",darkolivegreen:"#556B2F",cadetblue:"#5F9EA0",cornflowerblue:"#6495ED",rebeccapurple:"#663399",mediumaquamarine:"#66CDAA",dimgray:"#696969",dimgrey:"#696969",slateblue:"#6A5ACD",olivedrab:"#6B8E23",slategray:"#708090",slategrey:"#708090",lightslategray:"#778899",lightslategrey:"#778899",mediumslateblue:"#7B68EE",lawngreen:"#7CFC00",chartreuse:"#7FFF00",aquamarine:"#7FFFD4",skyblue:"#87CEEB",lightskyblue:"#87CEFA",blueviolet:"#8A2BE2",darkred:"#8B0000",darkmagenta:"#8B008B",saddlebrown:"#8B4513",darkseagreen:"#8FBC8F",lightgreen:"#90EE90",mediumpurple:"#9370DB",darkviolet:"#9400D3",palegreen:"#98FB98",darkorchid:"#9932CC",yellowgreen:"#9ACD32",sienna:"#A0522D",brown:"#A52A2A",darkgray:"#A9A9A9",darkgrey:"#A9A9A9",lightblue:"#ADD8E6",greenyellow:"#ADFF2F",paleturquoise:"#AFEEEE",lightsteelblue:"#B0C4DE",powderblue:"#B0E0E6",firebrick:"#B22222",darkgoldenrod:"#B8860B",mediumorchid:"#BA55D3",rosybrown:"#BC8F8F",darkkhaki:"#BDB76B",mediumvioletred:"#C71585",indianred:"#CD5C5C",peru:"#CD853F",chocolate:"#D2691E",tan:"#D2B48C",lightgray:"#D3D3D3",lightgrey:"#D3D3D3",thistle:"#D8BFD8",orchid:"#DA70D6",goldenrod:"#DAA520",palevioletred:"#DB7093",crimson:"#DC143C",gainsboro:"#DCDCDC",plum:"#DDA0DD",burlywood:"#DEB887",lightcyan:"#E0FFFF",lavender:"#E6E6FA",darksalmon:"#E9967A",violet:"#EE82EE",palegoldenrod:"#EEE8AA",lightcoral:"#F08080",khaki:"#F0E68C",aliceblue:"#F0F8FF",honeydew:"#F0FFF0",azure:"#F0FFFF",sandybrown:"#F4A460",wheat:"#F5DEB3",beige:"#F5F5DC",whitesmoke:"#F5F5F5",mintcream:"#F5FFFA",ghostwhite:"#F8F8FF",salmon:"#FA8072",antiquewhite:"#FAEBD7",linen:"#FAF0E6",lightgoldenrodyellow:"#FAFAD2",oldlace:"#FDF5E6",magenta:"#FF00FF",deeppink:"#FF1493",orangered:"#FF4500",tomato:"#FF6347",hotpink:"#FF69B4",coral:"#FF7F50",darkorange:"#FF8C00",lightsalmon:"#FFA07A",orange:"#FFA500",lightpink:"#FFB6C1",pink:"#FFC0CB",gold:"#FFD700",peachpuff:"#FFDAB9",navajowhite:"#FFDEAD",moccasin:"#FFE4B5",bisque:"#FFE4C4",mistyrose:"#FFE4E1",blanchedalmond:"#FFEBCD",papayawhip:"#FFEFD5",lavenderblush:"#FFF0F5",seashell:"#FFF5EE",cornsilk:"#FFF8DC",lemonchiffon:"#FFFACD",floralwhite:"#FFFAF0",snow:"#FFFAFA",lightyellow:"#FFFFE0",ivory:"#FFFFF0"},ai=new Int8Array(4),Dn=new Int32Array(ai.buffer,0,1),ri=new Float32Array(ai.buffer,0,1),Sl=/^\s*rgba?\s*\(/,El=/^\s*rgba?\s*\(\s*([0-9]*)\s*,\s*([0-9]*)\s*,\s*([0-9]*)(?:\s*,\s*(.*)?)?\)\s*$/;function Ln(a){var n=0,t=0,e=0,r=1,i=a.toLowerCase();if(i==="transparent")return{r:0,g:0,b:0,a:0};if(i in Vt)return Ln(Vt[i]);if(a[0]==="#")a.length===4?(n=parseInt(a.charAt(1)+a.charAt(1),16),t=parseInt(a.charAt(2)+a.charAt(2),16),e=parseInt(a.charAt(3)+a.charAt(3),16)):(n=parseInt(a.charAt(1)+a.charAt(2),16),t=parseInt(a.charAt(3)+a.charAt(4),16),e=parseInt(a.charAt(5)+a.charAt(6),16)),a.length===9&&(r=parseInt(a.charAt(7)+a.charAt(8),16)/255);else if(Sl.test(a)){var o=a.match(El);o&&(n=+o[1],t=+o[2],e=+o[3],o[4]&&(r=+o[4]))}return{r:n,g:t,b:e,a:r}}function ii(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,t=Ln(a),e=t.r,r=t.g,i=t.b,o=t.a;return n?[e/255*o,r/255*o,i/255*o,o]:[e/255,r/255,i/255,o]}var _t={};for($t in Vt)_t[$t]=ke(Vt[$t]),_t[Vt[$t]]=_t[$t];var $t;function Al(a,n,t,e,r){return Dn[0]=e<<24|t<<16|n<<8|a,r&&(Dn[0]=Dn[0]&4278190079),ri[0]}function ke(a){if(a=a.toLowerCase(),typeof _t[a]<"u")return _t[a];var n=Ln(a),t=n.r,e=n.g,r=n.b,i=n.a;i=i*255|0;var o=Al(t,e,r,i,!0);return _t[a]=o,o}function Xe(a,n){ri[0]=ke(a);var t=Dn[0];n&&(t=t|16777216);var e=t&255,r=t>>8&255,i=t>>16&255,o=t>>24&255;return[e,r,i,o]}function St(a){var n=a>>>8&255,t=a>>>16&255,e=a>>>24&255,r=a&255;return(r<<24|e<<16|t<<8|n)>>>0}function ma(a,n,t,e){return(t<<24|n<<16|a<<8|e)>>>0}function oi(a,n,t,e,r,i){var o=va(a,t,e,r,i),s=oe(o,2),l=s[0],u=s[1],d=new Uint8Array(4);a.bindFramebuffer(a.FRAMEBUFFER,n),a.readPixels(l,u,1,1,a.RGBA,a.UNSIGNED_BYTE,d);var h=oe(d,4),f=h[0],c=h[1],g=h[2],b=h[3];return[f,c,g,b]}function si(a){var n=Ln(a),t=n.r,e=n.g,r=n.b,i=n.a,o=(t/255).toFixed(6),s=(e/255).toFixed(6),l=(r/255).toFixed(6),u=i.toFixed(6);return"vec4(".concat(o,", ").concat(s,", ").concat(l,", ").concat(u,")")}function Dl(a,n){if(typeof a!="object"||!a)return a;var t=a[Symbol.toPrimitive];if(t!==void 0){var e=t.call(a,n||"default");if(typeof e!="object")return e;throw new TypeError("@@toPrimitive must return a primitive value.")}return(n==="string"?String:Number)(a)}function di(a){var n=Dl(a,"string");return typeof n=="symbol"?n:n+""}function E(a,n,t){return(n=di(n))in a?Object.defineProperty(a,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):a[n]=t,a}function li(a,n){var t=Object.keys(a);if(Object.getOwnPropertySymbols){var e=Object.getOwnPropertySymbols(a);n&&(e=e.filter(function(r){return Object.getOwnPropertyDescriptor(a,r).enumerable})),t.push.apply(t,e)}return t}function H(a){for(var n=1;n<arguments.length;n++){var t=arguments[n]!=null?arguments[n]:{};n%2?li(Object(t),!0).forEach(function(e){E(a,e,t[e])}):Object.getOwnPropertyDescriptors?Object.defineProperties(a,Object.getOwnPropertyDescriptors(t)):li(Object(t)).forEach(function(e){Object.defineProperty(a,e,Object.getOwnPropertyDescriptor(t,e))})}return a}function ne(a,n){if(!(a instanceof n))throw new TypeError("Cannot call a class as a function")}function ui(a,n){for(var t=0;t<n.length;t++){var e=n[t];e.enumerable=e.enumerable||!1,e.configurable=!0,"value"in e&&(e.writable=!0),Object.defineProperty(a,di(e.key),e)}}function ae(a,n,t){return n&&ui(a.prototype,n),t&&ui(a,t),Object.defineProperty(a,"prototype",{writable:!1}),a}function nt(a){return nt=Object.setPrototypeOf?Object.getPrototypeOf.bind():function(n){return n.__proto__||Object.getPrototypeOf(n)},nt(a)}function hi(){try{var a=!Boolean.prototype.valueOf.call(Reflect.construct(Boolean,[],function(){}))}catch{}return(hi=function(){return!!a})()}function wl(a){if(a===void 0)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return a}function Rl(a,n){if(n&&(typeof n=="object"||typeof n=="function"))return n;if(n!==void 0)throw new TypeError("Derived constructors may only return object or undefined");return wl(a)}function fe(a,n,t){return n=nt(n),Rl(a,hi()?Reflect.construct(n,t||[],nt(a).constructor):n.apply(a,t))}function ba(a,n){return ba=Object.setPrototypeOf?Object.setPrototypeOf.bind():function(t,e){return t.__proto__=e,t},ba(a,n)}function ge(a,n){if(typeof n!="function"&&n!==null)throw new TypeError("Super expression must either be null or a function");a.prototype=Object.create(n&&n.prototype,{constructor:{value:a,writable:!0,configurable:!0}}),Object.defineProperty(a,"prototype",{writable:!1}),n&&ba(a,n)}function Ll(a){if(Array.isArray(a))return wn(a)}function Cl(a){if(typeof Symbol<"u"&&a[Symbol.iterator]!=null||a["@@iterator"]!=null)return Array.from(a)}function kl(){throw new TypeError(`Invalid attempt to spread non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function J(a){return Ll(a)||Cl(a)||Rn(a)||kl()}function ie(a){"@babel/helpers - typeof";return ie=typeof Symbol=="function"&&typeof Symbol.iterator=="symbol"?function(n){return typeof n}:function(n){return n&&typeof Symbol=="function"&&n.constructor===Symbol&&n!==Symbol.prototype?"symbol":typeof n},ie(a)}function Ue(a){return ie(a)==="object"&&a!==null&&"attribute"in a}function ci(){var a=`
float sdf_circle(vec2 uv, float size) {
  return length(uv) - size;
}
`;return{name:"circle",glsl:a,uniforms:[]}}function fi(a){var n,t=WebGL2RenderingContext,e=t.UNSIGNED_BYTE,r=(n=a?.color)!==null&&n!==void 0?n:{attribute:"color"};if(!Ue(r)){var i=r,o=`
vec4 layer_fill() {
  return `.concat(si(i),`;
}
`);return{name:"fill",uniforms:[],attributes:[],glsl:o}}var s=r.attribute,l=`
vec4 layer_fill(vec4 v_fillColor) {
  return v_fillColor;
}
`;return{name:"fill",uniforms:[],attributes:[{name:"fillColor",size:4,type:e,normalized:!0,source:s}],glsl:l}}function Pl(a,n){if(typeof a=="string"){var t=ii(a).map(function(u){return u.toFixed(6)}),e=oe(t,4),r=e[0],i=e[1],o=e[2],s=e[3];return{glsl:"vec4(".concat(r,", ").concat(i,", ").concat(o,", ").concat(s,")"),attributes:[],needsNodeColors:!1}}if("node"in a&&a.node)return{glsl:a.node==="source"?"v_sourceColor":"v_targetColor",attributes:[],needsNodeColors:!0};var l=a;return{glsl:"v_".concat(n),attributes:[{name:"a_".concat(n),size:4,type:WebGL2RenderingContext.UNSIGNED_BYTE,normalized:!0,source:l.attribute,defaultValue:l.default}],needsNodeColors:!1}}function at(a){var n=a?.color;if(n===void 0){var t=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return v_color;
}
`;return{name:"plain",glsl:t,uniforms:[],attributes:[]}}var e=Pl(n,"plainColor"),r=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return `.concat(e.glsl,`;
}
`);return{name:"plain",glsl:r,uniforms:[],attributes:e.attributes,needsNodeColors:e.needsNodeColors}}function gi(){var a=`
// Position at parameter t \u2208 [0, 1]
vec2 path_straight_position(float t, vec2 source, vec2 target) {
  return mix(source, target, t);
}

// Total length of the path (analytical - more efficient than sampling)
float path_straight_length(vec2 source, vec2 target) {
  return length(target - source);
}
`;return{name:"straight",segments:1,minBodyLengthRatio:0,linearParameterization:!0,glsl:a,uniforms:[],attributes:[]}}function pi(a){var n=a??{},t=n.segments,e=t===void 0?32:t,r=`
const float LOOP_PI = 3.141592653589793;

float loopWorldRadius() {
  float nodeWorldRadius = v_sourceNodeSize * u_correctionRatio / u_sizeRatio;
  return max(v_loopRadius * nodeWorldRadius, 0.001);
}

// Cubic B\xE9zier with P0 = P3 = source.
// Control points extend outward orthogonal to the node surface at exit/entry angles.
vec2 path_loop_position(float t, vec2 source, vec2 target) {
  float R = loopWorldRadius();
  float angle = v_loopAngle + (v_loopFixedOrientation > 0.5 ? u_cameraAngle : 0.0);
  float halfSpread = v_loopSpread * 0.5;

  float exitAngle = angle - halfSpread;
  float entryAngle = angle + halfSpread;

  // Control point distance: at t=0.5 the B\xE9zier reaches 0.75 * cpDist * cos(halfSpread)
  // from source. Solve for cpDist so the loop tip reaches exactly R.
  float cpDist = R / (0.75 * cos(halfSpread));

  vec2 cp1 = source + cpDist * vec2(cos(exitAngle), sin(exitAngle));
  vec2 cp2 = source + cpDist * vec2(cos(entryAngle), sin(entryAngle));

  float u = 1.0 - t;
  vec2 d1 = cp1 - source;
  vec2 d2 = cp2 - source;
  return source + 3.0 * t * u * (u * d1 + t * d2);
}

// Approximate arc length via chord sampling
float path_loop_length(vec2 source, vec2 target) {
  float len = 0.0;
  vec2 prev = path_loop_position(0.0, source, target);
  const int STEPS = 16;
  for (int i = 1; i <= STEPS; i++) {
    float t = float(i) / float(STEPS);
    vec2 cur = path_loop_position(t, source, target);
    len += length(cur - prev);
    prev = cur;
  }
  return len;
}
`;return{name:"loop",segments:e,glsl:r,needsNodeSize:!0,uniforms:[],attributes:[{name:"loopRadius",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopAngle",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopSpread",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopFixedOrientation",size:1,type:WebGL2RenderingContext.FLOAT}],variables:{loopRadius:{type:"number",default:4},loopAngle:{type:"number",default:Math.PI/4},loopSpread:{type:"number",default:80*Math.PI/180},loopFixedOrientation:{type:"number",default:0}},spread:{variable:"loopRadius",compute:function(o){return o+4}}}}var _i=$e(vt());function vi(a){return ie(a)==="object"&&"glsl"in a&&!("attributes"in a)}function mi(a){return ie(a)==="object"&&"glsl"in a&&!("attributes"in a)}var jt={shapes:[ci()],variables:{},layers:[fi()],label:{},backdrop:{},labelAttachments:{}},Et={paths:[gi(),pi()],extremities:[],variables:{},layers:[at()],defaultHead:"none",defaultTail:"none",label:{}},Cn=["nodes","topNodes"],kn=["edges","topEdges"],Pn=[].concat(kn,Cn),bi={nodes:jt,edges:Et,depthLayers:J(Pn)};var Fl=function(n){return n},Il=function(n){return n*n},Gl=function(n){return n*(2-n)},Nl=function(n){return(n*=2)<1?.5*n*n:-.5*(--n*(n-2)-1)},zl=function(n){return n*n*n},Ml=function(n){return--n*n*n+1},Bl=function(n){return(n*=2)<1?.5*n*n*n:.5*((n-=2)*n*n+2)},Wl=function(n){return n===0?0:Math.pow(2,10*(n-1))},Ul=function(n){return n===1?1:1-Math.pow(2,-10*n)},Ol=function(n){return n===0?0:n===1?1:n<.5?Math.pow(2,10*(2*n-1))/2:(2-Math.pow(2,-10*(2*n-1)))/2},ya={linear:Fl,quadraticIn:Il,quadraticOut:Gl,quadraticInOut:Nl,cubicIn:zl,cubicOut:Ml,cubicInOut:Bl,exponentialIn:Wl,exponentialOut:Ul,exponentialInOut:Ol};function Fn(a){return a?typeof a=="function"?a:ya[a]:ya.linear}function _a(a){return ie(a)==="object"&&a!==null&&"attribute"in a}function Ti(a){return typeof a=="function"}function Ta(a){return ie(a)==="object"&&a!==null&&"when"in a&&typeof a.when=="function"&&"then"in a}function Sa(a){return ie(a)==="object"&&a!==null&&"whenState"in a&&"then"in a}function Ea(a){return ie(a)==="object"&&a!==null&&"whenData"in a&&"then"in a}var Hl={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,isDragged:!1},$l={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,parallelIndex:0,parallelCount:1},Vl={isIdle:!0,isPanning:!1,isZooming:!1,isDragging:!1,hasHovered:!1,hasHighlighted:!1};function Si(a){return H(H({},Hl),a)}function Ei(a){return H(H({},$l),a)}function Aa(a){return H(H({},Vl),a)}var Ai={x:{attribute:"x"},y:{attribute:"y"},size:{whenState:"isHovered",then:{attribute:"size",defaultValue:12},else:{attribute:"size",defaultValue:10}},color:{attribute:"color",defaultValue:"#666"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"},labelVisibility:{whenState:"isHovered",then:"visible",else:"auto"},backdropVisibility:{whenState:"isHovered",then:"visible",else:"hidden"},backdropColor:"#ffffff",backdropShadowColor:"rgba(0, 0, 0, 0.5)",backdropShadowBlur:12,backdropPadding:6},Di={size:{attribute:"size",defaultValue:1},color:{attribute:"color",defaultValue:"#ccc"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"}},jl={nodes:Ai,edges:Di},In={nodes:H(H({},Ai),{},{depth:{whenState:"isHovered",then:"topNodes",else:"nodes"}}),edges:H(H({},Di),{},{depth:{whenState:["isHighlighted","isHovered"],then:"topEdges",else:"edges"}})};function Xl(a){return"min"in a||"max"in a||"minValue"in a||"maxValue"in a||"easing"in a}function ql(a){return"dict"in a}function qt(a,n){return typeof a=="string"?n[a]===!0:Array.isArray(a)?a.every(function(t){return n[t]===!0}):ie(a)==="object"&&a!==null?Object.entries(a).every(function(t){var e=oe(t,2),r=e[0],i=e[1];return n[r]===i}):!1}function Kl(a,n){var t=n[a.attribute];return t===void 0?a.defaultValue:t}function Yl(a,n,t){var e,r,i,o,s=n[a.attribute];if(s===void 0)return a.defaultValue;var l=Number(s);if(isNaN(l))return a.defaultValue;if(a.min===void 0&&a.max===void 0)return l;var u=(e=a.minValue)!==null&&e!==void 0?e:l,d=(r=a.maxValue)!==null&&r!==void 0?r:l;if(d===u){var h;return(h=a.min)!==null&&h!==void 0?h:l}var f=(l-u)/(d-u);f=Math.max(0,Math.min(1,f));var c=Fn(a.easing);f=c(f);var g=(i=a.min)!==null&&i!==void 0?i:0,b=(o=a.max)!==null&&o!==void 0?o:1;return g+f*(b-g)}function Zl(a,n){var t=n[a.attribute];if(t===void 0)return a.defaultValue;var e=String(t);return e in a.dict?a.dict[e]:a.defaultValue}function Ql(a,n,t){return ql(a)?Zl(a,n):Xl(a)?Yl(a,n):Kl(a,n)}function Da(a,n){return typeof a=="string"?!!n[a]:Array.isArray(a)?a.every(function(t){return!!n[t]}):ie(a)==="object"&&a!==null?Object.entries(a).every(function(t){var e=oe(t,2),r=e[0],i=e[1];return n[r]===i}):!1}function Xt(a,n,t,e,r,i){if(a==null)return i;if(ie(a)!=="object"&&typeof a!="function")return a;if(Ta(a)){var o=a.when(n,t,e,r)?a.then:a.else;return o===void 0?i:Xt(o,n,t,e,r,i)}if(Sa(a)){var s=qt(a.whenState,t)?a.then:a.else;return s===void 0?i:Xt(s,n,t,e,r,i)}if(Ea(a)){var l=Da(a.whenData,n)?a.then:a.else;return l===void 0?i:Xt(l,n,t,e,r,i)}if(Ti(a)){var u=a(n,t,e,r);return u??i}if(_a(a)){var d=Ql(a,n);return d??i}return a}function Ge(a){if(a==null)return"static";if(Ta(a))return"graph-state";if(Sa(a)){var n=Ge(a.then),t=a.else!==void 0?Ge(a.else):"static";return Pe("item-state",Pe(n,t))}if(Ea(a)){var e=Ge(a.then),r=a.else!==void 0?Ge(a.else):"static";return Pe(e,r)}return Ti(a)?"graph-state":(_a(a),"static")}function Pe(a,n){return a==="graph-state"||n==="graph-state"?"graph-state":a==="item-state"||n==="item-state"?"item-state":"static"}function Gn(a){if(!a)return{dependency:"static",xAttribute:null,yAttribute:null};var n=Array.isArray(a)?a:[a],t="static",e=null,r=null,i=U(n),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if("matchData"in s&&"cases"in s)for(var l=s.cases,u=0,d=Object.values(l);u<d.length;u++)for(var h=d[u],f=0,c=Object.values(h);f<c.length;f++){var g=c[f];t=Pe(t,Ge(g))}else if("matchState"in s&&"cases"in s){t=Pe(t,"item-state");for(var b=s.cases,m=0,p=Object.values(b);m<p.length;m++)for(var v=p[m],y=0,T=Object.values(v);y<T.length;y++){var _=T[y];t=Pe(t,Ge(_))}}else if("when"in s&&"then"in s)t="graph-state";else if("whenState"in s){t=Pe(t,"item-state");var S=s.then;if(S&&ie(S)==="object")for(var D=0,w=Object.values(S);D<w.length;D++){var k=w[D];t=Pe(t,Ge(k))}var L=s.else;if(L&&ie(L)==="object")for(var N=0,R=Object.values(L);N<R.length;N++){var z=R[N];t=Pe(t,Ge(z))}}else if("whenData"in s){var C=s.then;if(C&&ie(C)==="object")for(var I=0,B=Object.values(C);I<B.length;I++){var F=B[I];t=Pe(t,Ge(F))}var x=s.else;if(x&&ie(x)==="object")for(var P=0,W=Object.values(x);P<W.length;P++){var G=W[P];t=Pe(t,Ge(G))}}else for(var A=0,q=Object.entries(s);A<q.length;A++){var V=oe(q[A],2),Y=V[0],pe=V[1];if(!Ra.has(Y)&&(t=Pe(t,Ge(pe)),(Y==="x"||Y==="y")&&_a(pe))){var se=pe.attribute;Y==="x"&&!e&&(e=se),Y==="y"&&!r&&(r=se)}}}}catch(Z){i.e(Z)}finally{i.f()}return{dependency:t,xAttribute:e,yAttribute:r}}var Oe={size:10,color:"#666",opacity:1,shape:"circle",rotationAlignment:"viewport",labelRotationAlignment:"viewport",visibility:"visible",depth:"nodes",zIndex:0,label:"",labelColor:"#000",labelSize:12,labelFont:"sans-serif",labelVisibility:"auto",labelPosition:"right",labelAngle:0,labelDepth:"nodes",backdropVisibility:"hidden",backdropColor:"transparent",backdropShadowColor:"transparent",backdropShadowBlur:0,backdropPadding:0,backdropBorderColor:"transparent",backdropBorderWidth:0,backdropCornerRadius:0,backdropLabelPadding:-1,backdropArea:"both",labelAttachment:null,labelAttachmentPlacement:"below"},wa={size:1,color:"#ccc",opacity:1,path:"straight",selfLoopPath:"loop",parallelSpread:.25,tail:"none",head:"none",visibility:"visible",depth:"edges",zIndex:0,label:"",labelColor:"#666",labelVisibility:"auto",labelDepth:"edges"},Ra=new Set(["when","whenState","whenData","then","else"]),yi=Object.keys(Oe),Jl=Object.values(Oe),xi=Object.keys(wa),eu=Object.values(wa);function At(a,n,t,e,r,i,o){for(var s in n){var l;if(!Ra.has(s)){var u=(l=a[s])!==null&&l!==void 0?l:o[s];a[s]=Xt(n[s],t,e,r,i,u)}}"depth"in n&&!("labelDepth"in n)&&(a.labelDepth=a.depth)}function wi(a,n,t,e,r,i,o){var s=U(n),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if("matchData"in u&&"cases"in u){var d,h=String((d=t[u.matchData])!==null&&d!==void 0?d:""),f=u.cases;h in f&&At(a,f[h],t,e,r,i,o)}else if("matchState"in u&&"cases"in u){var c,g=String((c=e[u.matchState])!==null&&c!==void 0?c:""),b=u.cases;g in b&&At(a,b[g],t,e,r,i,o)}else if("when"in u&&"then"in u){var m=u.when;if(!m(t,e,r,i))continue;At(a,u.then,t,e,r,i,o)}else if("whenState"in u&&"then"in u){if(!qt(u.whenState,e))continue;At(a,u.then,t,e,r,i,o)}else if("whenData"in u&&"then"in u){if(!Da(u.whenData,t))continue;At(a,u.then,t,e,r,i,o)}else At(a,u,t,e,r,i,o)}}catch(p){s.e(p)}finally{s.f()}}function Nn(a,n,t,e,r,i){for(var o=i||{},s=o,l=0,u=yi.length;l<u;l++)s[yi[l]]=Jl[l];if(s.x=void 0,s.y=void 0,s.labelBackgroundColor=void 0,s.labelBackgroundPadding=void 0,s.labelCursor=void 0,!a){var d,h,f,c,g;return s.x=(d=n.x)!==null&&d!==void 0?d:0,s.y=(h=n.y)!==null&&h!==void 0?h:0,o.size=(f=n.size)!==null&&f!==void 0?f:10,o.color=(c=n.color)!==null&&c!==void 0?c:"#666",o.label=(g=n.label)!==null&&g!==void 0?g:"",o}var b=Array.isArray(a)?a:[a];return wi(s,b,n,t,e,r,Oe),o}function zn(a,n,t,e,r,i){for(var o=i||{},s=o,l=0,u=xi.length;l<u;l++)s[xi[l]]=eu[l];if(!a){var d,h,f;return o.size=(d=n.size)!==null&&d!==void 0?d:1,o.color=(h=n.color)!==null&&h!==void 0?h:"#ccc",o.label=(f=n.label)!==null&&f!==void 0?f:"",o}var c=Array.isArray(a)?a:[a];return wi(s,c,n,t,e,r,wa),typeof s.labelPosition=="number"&&(s.labelPosition=void 0),o}function tu(a,n,t){var e,r;if(a==null)return t;if(typeof a=="function")return(e=a(n))!==null&&e!==void 0?e:t;if(ie(a)==="object"&&"when"in a){var i,o=a,s=o.when(n),l=s?o.then:o.else;return l===void 0?t:typeof l=="function"?(i=l(n))!==null&&i!==void 0?i:t:l??t}if(ie(a)==="object"&&"whenState"in a){var u,d=a,h=qt(d.whenState,n),f=h?d.then:d.else;return f===void 0?t:typeof f=="function"?(u=f(n))!==null&&u!==void 0?u:t:f??t}return(r=a)!==null&&r!==void 0?r:t}function La(a,n){var t={};if(!a)return t;var e=Array.isArray(a)?a:[a],r=U(e),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;if("when"in o&&"then"in o){var s=o.when(n),l=s?o.then:o.else;l&&ie(l)==="object"&&xa(t,l,n)}else if("whenState"in o&&"then"in o){var u=qt(o.whenState,n),d=u?o.then:o.else;d&&ie(d)==="object"&&xa(t,d,n)}else xa(t,o,n)}}catch(h){r.e(h)}finally{r.f()}return t}function xa(a,n,t){for(var e=0,r=Object.entries(n);e<r.length;e++){var i=oe(r[e],2),o=i[0],s=i[1];Ra.has(o)||(a[o]=tu(s,t))}}var Kt=(function(a){function n(){var t;return ne(this,n),t=fe(this,n),t.rawEmitter=t,t}return ge(n,a),ae(n)})(_i.EventEmitter);var Dt=jl,Ca=bi;function ka(a,n){var t=n.size;if(t!==0){var e=a.length;a.length+=t;var r=0;n.forEach(function(i){a[e+r]=i,r++})}}function Ri(a){a=a||{};for(var n=0,t=arguments.length<=1?0:arguments.length-1;n<t;n++){var e=n+1<1||arguments.length<=n+1?void 0:arguments[n+1];e&&Object.assign(a,e)}return a}function wt(a,n){for(var t in n)if(Object.prototype.hasOwnProperty.call(n,t)&&n[t]!==a[t])return!0;return!1}function Li(a,n){var t=a,e=n;for(var r in t)if(Object.prototype.hasOwnProperty.call(t,r)&&t[r]!==e[r])return!1;for(var i in e)if(Object.prototype.hasOwnProperty.call(e,i)&&e[i]!==t[i])return!1;return!0}function rt(a,n,t){t?a.add(n):a.delete(n)}var ki=$e(Pa());var Pi={easing:"quadraticInOut",duration:150};function qe(a){return a.labelVisibility==="visible"&&a.visibility!=="hidden"}function Fa(a){return a.backdropVisibility==="visible"&&a.visibility!=="hidden"}function Mn(a){return[a.rotationAlignment==="graph"?1:0,a.labelRotationAlignment==="graph"?1:0]}function Ia(a,n,t){var e=a[n];if(e)for(var r=0;r<e.length;r++){var i=e[r];if(!(t<i.offset||t>=i.offset+i.count)){if(i.count===1)e.splice(r,1);else if(t===i.offset)i.offset++,i.count--;else if(t===i.offset+i.count-1)i.count--;else{var o=t+1,s=i.offset+i.count-o;i.count=t-i.offset,e.splice(r+1,0,{offset:o,count:s})}return}}}function Ga(a,n,t){if(!a[n]){a[n]=[{offset:t,count:1}];return}for(var e=a[n],r=e.length,i=0;i<e.length;i++)if(t<e[i].offset){r=i;break}var o=r>0?e[r-1]:null,s=r<e.length?e[r]:null,l=o&&o.offset+o.count===t,u=s&&t+1===s.offset;l&&u?(o.count+=1+s.count,e.splice(r,1)):l?o.count++:u?(s.offset--,s.count++):e.splice(r,0,{offset:t,count:1})}function Fi(a){if(!(0,ki.default)(a))throw new Error("Sigma: invalid graph instance.")}function Bn(a){var n=oe(a.x,2),t=n[0],e=n[1],r=oe(a.y,2),i=r[0],o=r[1],s=Math.max(e-t,o-i),l=(e+t)/2,u=(o+i)/2;(s===0||Math.abs(s)===1/0||isNaN(s))&&(s=1),isNaN(l)&&(l=0),isNaN(u)&&(u=0);var d=function(f){return{x:.5+(f.x-l)/s,y:.5+(f.y-u)/s}};return d.applyTo=function(h){h.x=.5+(h.x-l)/s,h.y=.5+(h.y-u)/s},d.inverse=function(h){return{x:l+s*(h.x-.5),y:u+s*(h.y-.5)}},d.ratio=s,d}var Wn={hideEdgesOnMove:!1,hideLabelsOnMove:!1,renderLabels:!0,renderEdgeLabels:!1,edgeLabelAnchors:"nodeLabels",enableEdgeEvents:!1,nodeLabelEvents:!1,edgeLabelEvents:!1,pickingDownSizingRatio:2,nodePickingPadding:0,edgePickingPadding:4,labelPickingPadding:10,stagePadding:30,minEdgeThickness:1.7,antiAliasingFeather:1,antialiasEdges:!0,antialiasNodes:!0,dragTimeout:100,draggedEventsTolerance:3,inertiaDuration:200,inertiaRatio:3,zoomDuration:250,zoomingRatio:1.7,doubleClickTimeout:300,doubleClickZoomingRatio:2.2,doubleClickZoomingDuration:200,tapMoveTolerance:10,zoomToSizeRatioFunction:Math.sqrt,itemSizesReference:"positions",autoRescale:!0,autoRescaleContent:"positions",enableNodeDrag:!1,getDraggedNodes:function(n){return[n]},dragPositionToAttributes:null,labelDensity:1,labelGridCellSize:100,labelRenderedSizeThreshold:6,labelPixelSnapping:!0,minCameraRatio:null,maxCameraRatio:null,enableCameraZooming:!0,enableCameraPanning:!0,enableCameraRotation:!0,enableCameraMouseRotation:!0,cameraPanBoundaries:null,gestureTarget:"graph",sharedGestureWheelMessage:"Use Ctrl + scroll to zoom the graph",sharedGestureAppleWheelMessage:"Use \u2318 + scroll to zoom the graph",sharedGestureTouchMessage:"Use two fingers to move the graph",allowInvalidContainer:!1,DEBUG_displayPickingLayer:!1,DEBUG_logShaders:!1,DEBUG_logRenderStats:!1,DEBUG_gpuTimerQueries:!1};function Un(a){if(typeof a.labelDensity!="number"||a.labelDensity<0)throw new Error("Settings: invalid `labelDensity`. Expecting a positive number.");var n=a.minCameraRatio,t=a.maxCameraRatio;if(typeof n=="number"&&typeof t=="number"&&t<n)throw new Error("Settings: invalid camera ratio boundaries. Expecting `maxCameraRatio` to be greater than `minCameraRatio`.")}function Ii(a){return Ri({},Wn,a)}var nu=new Set(["italic","oblique"]),au=new Set(["bold","bolder","lighter"]);function On(a){for(var n="normal",t="normal",e=a.trim(),r=e.split(/\s+/),i=0,o=0;o<r.length;o++){var s=r[o].toLowerCase();if(nu.has(s))t=s,i=o+1;else if(au.has(s)||/^\d{3}$/.test(s))n=s,i=o+1;else if(s==="normal")i=o+1;else break}var l=r.slice(i).join(" ");return{family:l||e,weight:n,style:t}}function Rt(a,n,t){var e=document.createElement(a);if(n)for(var r in n)e.style[r]=n[r];if(t)for(var i in t)e.setAttribute(i,t[i]);return e}function Yt(){return typeof window.devicePixelRatio<"u"?window.devicePixelRatio:1}var $i=$e(vt());function ru(a){var n=a.type===WebGL2RenderingContext.FLOAT&&!a.normalized,t=a.type===WebGL2RenderingContext.UNSIGNED_BYTE&&a.size===4&&a.normalized;if(!n&&!t)throw new Error('Attribute "'.concat(a.name,'" is invalid: only non-normalized FLOAT and normalized UNSIGNED_BYTE with size 4 are supported.'));return a.normalized?1:a.size}function Na(a){var n=0;return a.forEach(function(t){return n+=ru(t)}),n}function Vi(a,n,t){var e=a==="VERTEX"?n.VERTEX_SHADER:n.FRAGMENT_SHADER,r=n.createShader(e);if(r===null)throw new Error("loadShader: error while creating the shader");n.shaderSource(r,t),n.compileShader(r);var i=n.getShaderParameter(r,n.COMPILE_STATUS);if(!i){var o=n.getShaderInfoLog(r);throw n.deleteShader(r),new Error(`loadShader: error while compiling the shader:
`.concat(o,`
`).concat(t))}return r}function Ma(a,n){return Vi("VERTEX",a,n)}function Ba(a,n){return Vi("FRAGMENT",a,n)}function Wa(a,n){var t=a.createProgram();if(t===null)throw new Error("loadProgram: error while creating the program.");var e,r;for(e=0,r=n.length;e<r;e++)a.attachShader(t,n[e]);a.linkProgram(t);var i=a.getProgramParameter(t,a.LINK_STATUS);if(!i){var o=a.getProgramInfoLog(t);throw a.deleteProgram(t),new Error("loadProgram: error while linking the program: ".concat(o))}return t}function Gi(a){var n=a.gl,t=a.buffer,e=a.program,r=a.vertexShader,i=a.fragmentShader;n.deleteShader(r),n.deleteShader(i),n.deleteProgram(e),n.deleteBuffer(t)}function me(a){return a%1===0?a.toFixed(1):a.toString()}var qn=new Map,ji=new Map,iu=0;function Xi(a,n){var t,e=(t=a.variables)===null||t===void 0||(t=t[n.source||n.name.replace(/^a_/,"")])===null||t===void 0?void 0:t.default,r=typeof e=="number"?e:typeof n.defaultValue=="number"?n.defaultValue:0;return me(r)}function Kn(a,n,t){var e,r=arguments.length>3&&arguments[3]!==void 0?arguments[3]:function(o){return"v_".concat(o.name.replace(/^a_/,""))},i=[n,t].concat(J(a.uniforms.filter(function(o){return o.type==="float"}).map(function(o){var s;return me((s=o.value)!==null&&s!==void 0?s:0)})),J(((e=a.attributes)!==null&&e!==void 0?e:[]).map(r)));return"sdf_".concat(a.name,"(").concat(i.join(", "),")")}function ou(a){var n,t=a.name,e=a.uniforms.filter(function(o){return o.type==="float"&&o.value!==void 0&&o.value!==0}).map(function(o){return"".concat(o.name.replace("u_",""),"=").concat(o.value)}).sort(),r=((n=a.attributes)!==null&&n!==void 0?n:[]).map(function(o){var s;return"".concat(o.name,"@").concat((s=o.source)!==null&&s!==void 0?s:"","=").concat(Xi(a,o))}),i=[].concat(J(e),J(r));return i.length>0&&(t+="#"+i.join("#")),t}function su(a){var n=ou(a);if(!qn.has(n)){var t={},e=U(a.uniforms),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;i.type==="float"&&i.value!==void 0&&(t[i.name]=i.value)}}catch(o){e.e(o)}finally{e.f()}qn.set(n,{shape:a,uniformValues:t,slug:n}),ji.set(n,iu++)}return n}function tn(a){var n;return(n=ji.get(a))!==null&&n!==void 0?n:-1}function qi(a){var n=[],t=new Set,e=new Set,r=/mat2 rotate2D\(float angle\)\s*\{[^}]+\}/,i=U(a),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if(!e.has(s.name)){e.add(s.name);var l=s.glsl;r.test(l)&&(t.has("rotate2D")?l=l.replace(r,""):t.add("rotate2D")),n.push(l)}}}catch(u){i.e(u)}finally{i.f()}return n.join(`
`)}function lu(){return qi(Array.from(qn.values()).map(function(a){return a.shape}))}function uu(a){var n=Array.from(qn.entries());if(n.length===0)return`
float querySDF(int shapeId, vec2 uv, float size) {
  return length(uv) - size;
}
`;var t=n.map(function(e,r){var i=oe(e,2),o=i[0],s=i[1].shape,l=Kn(s,"uv","size",function(u){var d=u.name.replace(/^a_/,"");return a!=null&&a.has(d)?"g_".concat(d):Xi(s,u)});return"    case ".concat(r,": return ").concat(l,"; // ").concat(o)}).join(`
`);return`
float querySDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(t,`
    default: return length(uv) - size;
  }
}
`)}function Ua(a){return qi(a)}function nn(a){return J(new Map(a.flatMap(function(n){return n.uniforms}).map(function(n){return[n.name,n]})).values())}function du(a,n){if(a.length===0)return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = length(uv) - size;
  context.inradiusFactor = 1.0;
}
`;var t=function(h){return Kn(h,"uv","size")},e=function(h){var f,c;return(f=h.inradiusFactorGLSL)!==null&&f!==void 0?f:me((c=h.inradiusFactor)!==null&&c!==void 0?c:1)};if(a.length===1){var r=a[0];return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = `.concat(t(r),`;
  context.inradiusFactor = `).concat(e(r),`;
}
`)}var i=a.map(function(d,h){return"    case ".concat(h,": // ").concat(d.name,`
      context.sdf = `).concat(t(d),`;
      context.inradiusFactor = `).concat(e(d),`;
      break;`)}).join(`
`),o=a[0],s="",l="shapeId";if(n&&n.length>1){var u=n.map(function(d,h){return"    case ".concat(d,": return ").concat(h,"; // ").concat(a[h].name)}).join(`
`);s=`
int globalToLocalShapeId(int globalId) {
  switch (globalId) {
`.concat(u,`
    default: return 0;
  }
}
`),l="globalToLocalShapeId(shapeId)"}return"".concat(s,`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  switch (`).concat(l,`) {
`).concat(i,`
    default:
      context.sdf = `).concat(t(o),`;
      context.inradiusFactor = `).concat(e(o),`;
  }
}
`)}var Ct={right:0,left:1,above:2,below:3,over:4},kt=5,Oa=3,Ki=`
float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
float nodeRadiusGraphSpace = nodeSize * u_correctionRatio / u_sizeRatio * 2.0;
float nodeRadiusNDC = nodeRadiusGraphSpace * matrixScaleX;
float nodeRadiusPixels = nodeRadiusNDC * u_resolution.x / 2.0;
`,hu=`
mat2 rotate2D(float angle) {
  float c = cos(angle);
  float s = sin(angle);
  return mat2(c, -s, s, c);
}
`,Yi=`
vec2 getLabelDirection(float positionMode) {
  if (positionMode < 0.5) return vec2(1.0, 0.0);   // Right
  if (positionMode < 1.5) return vec2(-1.0, 0.0);  // Left
  if (positionMode < 2.5) return vec2(0.0, -1.0);  // Above
  if (positionMode < 3.5) return vec2(0.0, 1.0);   // Below
  return vec2(0.0);                                 // Over (centered)
}
`,cu=`
float sdfBox(vec2 p, vec2 halfSize) {
  vec2 d = abs(p) - halfSize;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}
`,fu=`
float sdfRotatedBox(vec2 p, vec2 halfSize, float angle) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfBox(rotatedP, halfSize);
}
`,gu=`
float sdfRoundedBox(vec2 p, vec2 halfSize, float radius) {
  vec2 d = abs(p) - halfSize + radius;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - radius;
}
`,pu=`
float sdfRoundedRotatedBox(vec2 p, vec2 halfSize, float angle, float radius) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfRoundedBox(rotatedP, halfSize, radius);
}
`,Zi=`
vec2 labelBoxCenter(float positionMode, float labelStart, vec2 halfSize, float textHalfY) {
  if (positionMode < 0.5) return vec2(labelStart + halfSize.x, 0.0);    // right
  if (positionMode < 1.5) return vec2(-(labelStart + halfSize.x), 0.0); // left
  if (positionMode < 2.5) return vec2(0.0, -(labelStart + textHalfY));  // above
  if (positionMode < 3.5) return vec2(0.0, labelStart + textHalfY);     // below
  return vec2(0.0);                                                     // over
}
`,Ha=2,He=`
vec4 readNodeData(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Ha,`;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),ot=`
vec4 readNodeFlags(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Ha,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),vu=`
vec4 readNodeColor(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Ha,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  vec4 texel = texelFetch(nodeDataTexture, coord, 0);
  float r = floor(texel.b / 65536.0);
  float g = floor(mod(texel.b, 65536.0) / 256.0);
  float b = mod(texel.b, 256.0);
  return vec4(r / 255.0, g / 255.0, b / 255.0, texel.a);
}
`),Pt=`
vec4 readFrameTexel(sampler2D frameTexture, int frameTextureWidth, int index) {
  ivec2 coord = ivec2(index % frameTextureWidth, index / frameTextureWidth);
  return texelFetch(frameTexture, coord, 0);
}
`;function mu(a,n){var t=function(o){return Kn(o,"uv","size")};if(a.length===1)return{code:Ni(t(a[0])),multiShape:!1};var e=a.map(function(i,o){return"    case ".concat(n?n[o]:o,": return ").concat(t(i),";")}).join(`
`),r=`
float queryShapeSDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(e,`
    default: return `).concat(t(a[0]),`;
  }
}
int g_shapeId;
`).concat(Ni("queryShapeSDF(g_shapeId, uv, size)"),`
`);return{code:r,multiShape:!0}}function Ni(a){return`
float findEdgeDistance(vec2 direction, float size) {
  float lo = 0.0, hi = 2.0;
  for (int i = 0; i < 8; i++) {
    float mid = (lo + hi) * 0.5;
    vec2 uv = direction * mid;
    if (`.concat(a,` < 0.0) lo = mid; else hi = mid;
  }
  return (lo + hi) * 0.5;
}
`)}function bu(a,n){for(;!{}.hasOwnProperty.call(a,n)&&(a=nt(a))!==null;);return a}function za(){return za=typeof Reflect<"u"&&Reflect.get?Reflect.get.bind():function(a,n,t){var e=bu(a,n);if(e){var r=Object.getOwnPropertyDescriptor(e,n);return r.get?r.get.call(arguments.length<3?a:t):r.value}},za.apply(null,arguments)}function ye(a,n,t,e){var r=za(nt(1&e?a.prototype:a),n,t);return 2&e&&typeof r=="function"?function(i){return r.apply(t,i)}:r}var yu=1024,xu=1.5,_u=4096,Qi=-1,Yn=(function(){function a(n,t){var e=arguments.length>2&&arguments[2]!==void 0?arguments[2]:yu;ne(this,a),E(this,"texture",null),E(this,"dirty",!1),E(this,"dirtyRangeStart",1/0),E(this,"dirtyRangeEnd",-1),E(this,"indexMap",new Map),E(this,"freeIndices",[]),E(this,"nextIndex",0),this.gl=n,this.TEXELS_PER_ITEM=t,this.capacity=this.roundUpToPowerOfTwo(e);var r=this.computeTextureDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.data=new Float32Array(this.textureWidth*this.textureHeight*4),this.createTexture()}return ae(a,[{key:"computeTextureDimensions",value:function(t){var e=t*this.TEXELS_PER_ITEM,r=Math.min(e,_u),i=Math.ceil(e/r);return{width:r,height:i}}},{key:"roundUpToPowerOfTwo",value:function(t){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,t))))}},{key:"createTexture",value:function(){var t=this.gl;this.texture=t.createTexture(),t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture),t.texImage2D(t.TEXTURE_2D,0,t.RGBA32F,this.textureWidth,this.textureHeight,0,t.RGBA,t.FLOAT,this.data),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_S,t.CLAMP_TO_EDGE),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_T,t.CLAMP_TO_EDGE),t.bindTexture(t.TEXTURE_2D,null)}},{key:"resize",value:function(t){if(!(t<=this.capacity)){var e=this.roundUpToPowerOfTwo(Math.ceil(t*xu)),r=this.gl,i=this.computeTextureDimensions(e),o=new Float32Array(i.width*i.height*4);o.set(this.data),this.texture&&r.deleteTexture(this.texture),this.data=o,this.capacity=e,this.textureWidth=i.width,this.textureHeight=i.height,this.createTexture(),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.nextIndex}}},{key:"allocate",value:function(t){var e=this.indexMap.get(t);if(e!==void 0)return e;var r;return this.freeIndices.length>0?r=this.freeIndices.pop():(r=this.nextIndex++,r>=this.capacity&&this.resize(r+1)),this.indexMap.set(t,r),r}},{key:"free",value:function(t){var e=this.indexMap.get(t);if(e!==void 0){this.indexMap.delete(t),this.freeIndices.push(e);for(var r=e*this.TEXELS_PER_ITEM*4,i=0;i<this.TEXELS_PER_ITEM*4;i++)this.data[r+i]=0;this.markDirty(e)}}},{key:"getIndex",value:function(t){var e;return(e=this.indexMap.get(t))!==null&&e!==void 0?e:-1}},{key:"has",value:function(t){return this.indexMap.has(t)}},{key:"markDirty",value:function(t){this.dirty=!0,this.dirtyRangeStart=Math.min(this.dirtyRangeStart,t),this.dirtyRangeEnd=Math.max(this.dirtyRangeEnd,t+1)}},{key:"upload",value:function(){if(!(!this.dirty||!this.texture)){var t=this.gl,e=this.textureWidth;t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture);var r=this.dirtyRangeStart*this.TEXELS_PER_ITEM,i=Math.min(this.dirtyRangeEnd*this.TEXELS_PER_ITEM,this.capacity*this.TEXELS_PER_ITEM);if(r<i)for(var o=Math.floor(r/e),s=Math.floor((i-1)/e),l=o;l<=s;l++){var u=l*e,d=Math.min(u+e,this.capacity*this.TEXELS_PER_ITEM),h=Math.max(r,u),f=Math.min(i,d);if(h<f){var c=h-u,g=f-h,b=this.data.subarray(h*4,f*4);t.texSubImage2D(t.TEXTURE_2D,0,c,l,g,1,t.RGBA,t.FLOAT,b)}}t.bindTexture(t.TEXTURE_2D,null),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}}},{key:"bind",value:function(t){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getCapacity",value:function(){return this.capacity}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"getTexelsPerItem",value:function(){return this.TEXELS_PER_ITEM}},{key:"getCount",value:function(){return this.indexMap.size}},{key:"getHighWaterMark",value:function(){return this.nextIndex}},{key:"isDirty",value:function(){return this.dirty}},{key:"clear",value:function(){this.indexMap.clear(),this.freeIndices=[],this.nextIndex=0,this.data.fill(0),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.capacity}},{key:"restore",value:function(){this.createTexture(),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}},{key:"kill",value:function(){this.texture&&(this.gl.deleteTexture(this.texture),this.texture=null),this.indexMap.clear(),this.freeIndices=[]}}])})();function Fe(a){var n={},t={},e=0,r=U(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=U(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");d in n||(n[d]=e,t[d]=u,e+=u.size)}}catch(h){s.e(h)}finally{s.f()}}}catch(h){r.e(h)}finally{r.f()}return{floatsPerItem:e,texelsPerItem:Math.max(1,Math.ceil(e/4)),offsets:n,specs:t}}var Zn=(function(a){function n(t,e,r){var i;return ne(this,n),i=fe(this,n,[t,e.texelsPerItem,r]),i.floatsPerItem=e.floatsPerItem,i}return ge(n,a),ae(n,[{key:"updateAllAttributes",value:function(e,r){var i=this.indexMap.get(e);i===void 0&&(i=this.allocate(e));for(var o=i*this.TEXELS_PER_ITEM*4,s=Math.min(r.length,this.floatsPerItem),l=0;l<s;l++)this.data[o+l]=r[l];this.markDirty(i)}},{key:"updateAllAttributesAtRow",value:function(e,r){e>=this.capacity&&this.resize(e+1);for(var i=e*this.TEXELS_PER_ITEM*4,o=Math.min(r.length,this.floatsPerItem),s=0;s<o;s++)this.data[i+s]=r[s];this.markDirty(e)}}])})(Yn);function Qn(a,n,t){for(var e=[],r=new Set,i=0;i<a.length;i++){var o=a[i],s=U(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");if(!r.has(d)){r.add(d);var h=n.offsets[d];if(h!==void 0){var f=t?.get(i);e.push({sourceKey:u.source||d,packedOffset:h,size:u.size,isColor:u.size===4&&!!u.normalized,defaultNum:typeof u.defaultValue=="number"?u.defaultValue:u.defaultValue===!0?1:0,defaultColor:typeof u.defaultValue=="string"?u.defaultValue:"",sourceIndex:i,hasLifecycleHook:!!(f!=null&&f.getAttributeData)})}}}}catch(c){s.e(c)}finally{s.f()}}return e}function Jn(a,n,t,e,r,i){t.fill(0);for(var o=0,s=a.length;o<s;o++){var l=a[o],u=l.packedOffset,d=void 0;if(l.hasLifecycleHook){var h=r.get(l.sourceIndex-i);d=h.getAttributeData(n,l.sourceKey),d===null&&(d=n[l.sourceKey])}else d=n[l.sourceKey];if(l.isColor){var f=typeof d=="string"?d:l.defaultColor||e,c=Xe(f),g=oe(c,4),b=g[0],m=g[1],p=g[2],v=g[3];t[u]=b/255,t[u+1]=m/255,t[u+2]=p/255,t[u+3]=v/255}else if(l.size===1)t[u]=typeof d=="number"?d:typeof d=="boolean"?d?1:0:l.defaultNum;else{var y=Array.isArray(d)?d:null;if(y)for(var T=0;T<l.size;T++){var _;t[u+T]=(_=y[T])!==null&&_!==void 0?_:0}}}}var Hn=["r","g","b","a"];function Lt(a,n){var t=a.offsets,e=a.specs,r=a.texelsPerItem,i=a.floatsPerItem,o=Object.keys(t);if(o.length===0||i===0)return{fetchCode:"",varyingAssignments:""};var s=n.varPrefix,l=n.baseTexelExpr,u=n.textureWidthUniform,d=n.textureSamplerUniform,h=n.outputPrefix,f=h===void 0?"v_":h,c=[];c.push("  int ".concat(s,"BaseTexel = ").concat(l,";")),c.push("");for(var g=0;g<r;g++)c.push("  ivec2 ".concat(s,"Coord").concat(g," = ivec2((").concat(s,"BaseTexel + ").concat(g,") % ").concat(u,", (").concat(s,"BaseTexel + ").concat(g,") / ").concat(u,");")),c.push("  vec4 ".concat(s,"Texel").concat(g," = texelFetch(").concat(d,", ").concat(s,"Coord").concat(g,", 0);"));c.push("");for(var b=[],m=function(){var T=v[p],_=e[T],S=t[T],D=Math.floor(S/4),w=S%4,k="".concat(s,"Texel").concat(D),L="".concat(s,"Texel").concat(D+1),N="".concat(s,"Fetched_").concat(T);if(_.size===1)c.push("  float ".concat(N," = ").concat(k,".").concat(Hn[w],";"));else if(_.size===4&&w===0)c.push("  vec4 ".concat(N," = ").concat(k,";"));else{var R=w+_.size;if(R<=4){var z="vec".concat(_.size),C=Hn.slice(w,R).join("");c.push("  ".concat(z," ").concat(N," = ").concat(k,".").concat(C,";"))}else{var I=_.size===4?"vec4":"vec".concat(_.size),B=Hn.slice(w).map(function(P){return"".concat(k,".").concat(P)}),F=Hn.slice(0,R-4).map(function(P){return"".concat(L,".").concat(P)}),x=[].concat(J(B),J(F)).join(", ");c.push("  ".concat(I," ").concat(N," = ").concat(I,"(").concat(x,");"))}}b.push("  ".concat(f).concat(T," = ").concat(N,";"))},p=0,v=o;p<v.length;p++)m();return{fetchCode:c.join(`
`),varyingAssignments:b.join(`
`)}}function $a(a,n,t){if(!(!n||t.type==="sampler2D"))switch(t.type){case"float":a.uniform1f(n,t.value);break;case"int":case"bool":a.uniform1i(n,t.value);break;case"vec2":a.uniform2fv(n,t.value);break;case"vec3":a.uniform3fv(n,t.value);break;case"vec4":a.uniform4fv(n,t.value);break;case"mat3":a.uniformMatrix3fv(n,!1,t.value);break;case"mat4":a.uniformMatrix4fv(n,!1,t.value);break}}var Tu=E(E(E(E(E(E(E(E({},WebGL2RenderingContext.BOOL,1),WebGL2RenderingContext.BYTE,1),WebGL2RenderingContext.UNSIGNED_BYTE,1),WebGL2RenderingContext.SHORT,2),WebGL2RenderingContext.UNSIGNED_SHORT,2),WebGL2RenderingContext.INT,4),WebGL2RenderingContext.UNSIGNED_INT,4),WebGL2RenderingContext.FLOAT,4);function $n(a){var n=a.match(/^(#version[^\n]*\n)/);return n?n[1]+`#define PICKING_MODE
`+a.slice(n[1].length):`#define PICKING_MODE
`+a}var st=(function(){function a(n,t,e){ne(this,a),E(this,"floats",new Float32Array),E(this,"ints",new Uint32Array),E(this,"constantArray",new Float32Array),E(this,"capacity",0),E(this,"verticesCount",0),E(this,"bufferGeneration",0),E(this,"uploadedGeneration",new Map),E(this,"constantBufferGeneration",0),E(this,"uploadedConstantGeneration",new Map),E(this,"renderOffset",0),E(this,"renderCount",-1),E(this,"debugStats",{drawCalls:0,verticesDrawn:0,bufferUploadBytes:0}),E(this,"shadersLogged",!1),E(this,"pickProgram",null);var r=this.getDefinition();if(this.VERTICES=r.VERTICES,this.VERTEX_SHADER_SOURCE=r.VERTEX_SHADER_SOURCE,this.FRAGMENT_SHADER_SOURCE=r.FRAGMENT_SHADER_SOURCE,this.UNIFORMS=r.UNIFORMS,this.ATTRIBUTES=r.ATTRIBUTES,this.METHOD=r.METHOD,this.CONSTANT_ATTRIBUTES="CONSTANT_ATTRIBUTES"in r?r.CONSTANT_ATTRIBUTES:[],this.CONSTANT_DATA="CONSTANT_DATA"in r?r.CONSTANT_DATA:[],this.isInstanced="CONSTANT_ATTRIBUTES"in r,this.ATTRIBUTES_ITEMS_COUNT=Na(this.ATTRIBUTES),this.STRIDE=this.VERTICES*this.ATTRIBUTES_ITEMS_COUNT,this.renderer=e,this.normalProgram=this.getProgramInfo("normal",n,r.VERTEX_SHADER_SOURCE,r.FRAGMENT_SHADER_SOURCE,null),this.pickProgram=this.getProgramInfo("pick",n,$n(r.VERTEX_SHADER_SOURCE),$n(r.FRAGMENT_SHADER_SOURCE),null),this.isInstanced){var i=Na(this.CONSTANT_ATTRIBUTES);if(this.CONSTANT_DATA.length!==this.VERTICES)throw new Error("Program: error while getting constant data (expected ".concat(this.VERTICES," items, received ").concat(this.CONSTANT_DATA.length," instead)"));this.constantArray=new Float32Array(this.CONSTANT_DATA.length*i);for(var o=0;o<this.CONSTANT_DATA.length;o++){var s=this.CONSTANT_DATA[o];if(s.length!==i)throw new Error("Program: error while getting constant data (one vector has ".concat(s.length," items instead of ").concat(i,")"));for(var l=0;l<s.length;l++)this.constantArray[o*i+l]=s[l]}this.STRIDE=this.ATTRIBUTES_ITEMS_COUNT}}return ae(a,[{key:"kill",value:function(){Gi(this.normalProgram),this.pickProgram&&Gi(this.pickProgram)}},{key:"resetDebugStats",value:function(){this.debugStats.drawCalls=0,this.debugStats.verticesDrawn=0,this.debugStats.bufferUploadBytes=0}},{key:"getProgramInfo",value:function(t,e,r,i,o){var s=e.createBuffer();if(s===null)throw new Error("Program: error while creating the WebGL buffer.");var l=Ma(e,r),u=Ba(e,i),d=Wa(e,[l,u]),h={};this.UNIFORMS.forEach(function(g){var b=e.getUniformLocation(d,g);b&&(h[g]=b)});var f={};this.ATTRIBUTES.forEach(function(g){f[g.name]=e.getAttribLocation(d,g.name)});var c;if(this.isInstanced&&(this.CONSTANT_ATTRIBUTES.forEach(function(g){f[g.name]=e.getAttribLocation(d,g.name)}),c=e.createBuffer(),c===null))throw new Error("Program: error while creating the WebGL constant buffer.");return{name:t,program:d,gl:e,frameBuffer:o,buffer:s,constantBuffer:c||{},uniformLocations:h,attributeLocations:f,isPicking:t==="pick",vertexShader:l,fragmentShader:u}}},{key:"bindProgram",value:function(t){var e=this,r=0,i=t.gl,o=t.buffer;if(this.isInstanced){if(i.bindBuffer(i.ARRAY_BUFFER,t.constantBuffer),r=0,this.CONSTANT_ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r,!1)}),this.uploadedConstantGeneration.get(t.constantBuffer)!==this.constantBufferGeneration){var l;i.bufferData(i.ARRAY_BUFFER,this.constantArray,i.STATIC_DRAW),this.uploadedConstantGeneration.set(t.constantBuffer,this.constantBufferGeneration),(l=this.renderer)!==null&&l!==void 0&&l.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.constantArray.byteLength)}if(i.bindBuffer(i.ARRAY_BUFFER,t.buffer),r=this.renderOffset*this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT,this.ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r,!0)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var u;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(u=this.renderer)!==null&&u!==void 0&&u.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}}else if(i.bindBuffer(i.ARRAY_BUFFER,o),r=0,this.ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var s;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(s=this.renderer)!==null&&s!==void 0&&s.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}i.bindBuffer(i.ARRAY_BUFFER,null)}},{key:"unbindProgram",value:function(t){var e=this;this.isInstanced?(this.CONSTANT_ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t,!1)}),this.ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t,!0)})):this.ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t)})}},{key:"bindAttribute",value:function(t,e,r,i){var o=Tu[t.type];if(typeof o!="number")throw new Error('Program.bind: yet unsupported attribute type "'.concat(t.type,'"'));var s=e.attributeLocations[t.name],l=e.gl;if(s!==-1){l.enableVertexAttribArray(s);var u=this.isInstanced?(i?this.ATTRIBUTES_ITEMS_COUNT:Na(this.CONSTANT_ATTRIBUTES))*Float32Array.BYTES_PER_ELEMENT:this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT;l.vertexAttribPointer(s,t.size,t.type,t.normalized||!1,u,r),this.isInstanced&&i&&l.vertexAttribDivisor(s,1)}return t.size*o}},{key:"unbindAttribute",value:function(t,e,r){var i=e.attributeLocations[t.name],o=e.gl;i!==-1&&(o.disableVertexAttribArray(i),this.isInstanced&&r&&o.vertexAttribDivisor(i,0))}},{key:"reallocate",value:function(t){t!==this.capacity&&(this.capacity=t,this.verticesCount=this.VERTICES*t,this.floats=new Float32Array(this.isInstanced?this.capacity*this.ATTRIBUTES_ITEMS_COUNT:this.verticesCount*this.ATTRIBUTES_ITEMS_COUNT),this.ints=new Uint32Array(this.floats.buffer),this.invalidateBuffers())}},{key:"invalidateBuffers",value:function(){this.bufferGeneration++,this.constantBufferGeneration++}},{key:"hasNothingToRender",value:function(){return this.verticesCount===0}},{key:"setTypedUniform",value:function(t,e){var r;$a(e.gl,(r=e.uniformLocations[t.name])!==null&&r!==void 0?r:null,t)}},{key:"renderProgram",value:function(t,e){var r=e.gl,i=e.program,o=e.isPicking;o?r.disable(r.BLEND):r.enable(r.BLEND),r.useProgram(i),this.setUniforms(t,e),this.drawWebGL(this.METHOD,e)}},{key:"render",value:function(t,e,r){var i;if(!this.shadersLogged&&(i=this.renderer)!==null&&i!==void 0&&i.getSetting("DEBUG_logShaders")&&(this.shadersLogged=!0,console.log("[sigma] DEBUG_logShaders: ".concat(this.constructor.name),{normal:{vertexShaderSource:this.VERTEX_SHADER_SOURCE,fragmentShaderSource:this.FRAGMENT_SHADER_SOURCE},pick:{vertexShaderSource:$n(this.VERTEX_SHADER_SOURCE),fragmentShaderSource:$n(this.FRAGMENT_SHADER_SOURCE)}})),!this.hasNothingToRender()){this.renderOffset=e??0,this.renderCount=r??-1;var o=this.normalProgram.gl;if(o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,t.width*t.pixelRatio,t.height*t.pixelRatio),this.bindProgram(this.normalProgram),this.renderProgram(t,this.normalProgram),this.unbindProgram(this.normalProgram),this.pickProgram&&t.pickingFrameBuffer){var s=Math.ceil(t.width*t.pixelRatio/t.downSizingRatio),l=Math.ceil(t.height*t.pixelRatio/t.downSizingRatio);o.bindFramebuffer(o.FRAMEBUFFER,t.pickingFrameBuffer),o.viewport(0,0,s,l),this.bindProgram(this.pickProgram),this.renderProgram(t,this.pickProgram),this.unbindProgram(this.pickProgram),o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,t.width*t.pixelRatio,t.height*t.pixelRatio)}}}},{key:"drawWebGL",value:function(t,e){var r,i=e.gl,o=this.renderCount>=0?this.renderCount:this.capacity;(r=this.renderer)!==null&&r!==void 0&&r.getSetting("DEBUG_logRenderStats")&&(this.debugStats.drawCalls++,this.debugStats.verticesDrawn+=o*this.VERTICES),this.isInstanced?i.drawArraysInstanced(t,0,this.VERTICES,o):i.drawArrays(t,this.renderOffset*this.VERTICES,o*this.VERTICES)}}])})(),Ze=2,an=7,Ji={below:0,above:1,left:2,right:3};function Su(){var a=`#version 300 es
precision highp float;

uniform mat3 u_matrix;
uniform vec2 u_resolution;
uniform float u_labelPixelSnapping;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
uniform float u_pixelRatio;
uniform float u_labelMargin;
uniform float u_zoomLabelSizeRatio;

uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_nodeFrameTexture;
uniform int u_nodeFrameTextureWidth;

// Atlas size is fixed at 2048\xD72048 \u2014 matches AttachmentManager.ATLAS_SIZE.
uniform sampler2D u_atlasTexture;
const vec2 u_atlasSize = vec2(2048.0);

// Per-instance
in float a_nodeIndex;           // node-data texture index
in vec4 a_atlasRect;            // x, y, width, height in atlas pixels
in vec2 a_attachmentSize;       // attachment dimensions (CSS px)
in float a_positionMode;        // label position: 0=right 1=left 2=above 3=below 4=over
in float a_attachmentPlacement; // 0=below 1=above 2=left 3=right (relative to label)
in float a_labelWidth;          // label width (CSS px)
in float a_labelHeight;         // label height: font line box (CSS px)
in float a_textHeight;          // actual glyph height (CSS px)
in float a_labelAngle;          // label rotation angle (radians)

// Per-vertex (constant)
in vec2 a_quadCorner;           // [-1,-1], [1,-1], [-1,1], [1,1]

out vec2 v_texCoord;

`.concat(He,`
`).concat(ot,`
`).concat(Pt,`
`).concat(Zi,`
`).concat(hu,`

void main() {
  int nodeIdx = int(a_nodeIndex);

  // Node data: (x, y, size, shapeId).
  vec4 nodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  vec2 nodePos = nodeData.xy;
  float nodeSize = nodeData.z;

  // Normalized edge distance from the shared frame texture (the frame-pass ran
  // the SDF search once; we just read the result).
  float edgeDist = readFrameTexel(u_nodeFrameTexture, u_nodeFrameTextureWidth, nodeIdx).r;

  // Per-node label rotation alignment: 0 = viewport, 1 = label turns with camera.
  float labelRotation = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx).g;

  vec3 nodeClip = u_matrix * vec3(nodePos, 1.0);

  // Node radius in physical pixels (matches label/background).
  float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
  float nodeRadiusGraphSpace = nodeSize * u_correctionRatio / u_sizeRatio * 2.0;
  float nodeRadiusPixels = nodeRadiusGraphSpace * matrixScaleX * u_resolution.x / 2.0;

  // CSS-px inputs -> physical px, scaled by the zoom-dependent label ratio, so
  // the attachment stays glued to the label box exactly as the label scales.
  float zoomScale = u_zoomLabelSizeRatio;
  vec2 labelHalf = vec2(a_labelWidth, a_labelHeight) * 0.5 * zoomScale * u_pixelRatio;
  vec2 attachHalf = a_attachmentSize * 0.5 * zoomScale * u_pixelRatio;
  float gap = `).concat(Ze.toFixed(1),` * zoomScale * u_pixelRatio;
  float labelMargin = u_labelMargin * zoomScale * u_pixelRatio;

  // Graph-aligned labels add the camera angle so the attachment orbits with the box.
  mat2 labelRotMat = rotate2D(a_labelAngle - labelRotation * u_cameraAngle);

  // Label box center relative to the node center (pre-rotation). The shape-aware
  // edge distance comes from the frame texture, so placement matches the label.
  vec2 boxCenter = vec2(0.0);
  if (a_positionMode < 4.0) {
    float labelStart = nodeRadiusPixels * edgeDist + labelMargin;
    float textHalf = a_textHeight * 0.5 * zoomScale * u_pixelRatio;
    boxCenter = labelBoxCenter(a_positionMode, labelStart, labelHalf, textHalf);
  }

  // Horizontal anchor of below/above attachments tracks the label position, so
  // the attachment hangs from the label edge nearest the node (or is centered
  // when the label itself is node-centered): right->left, left->right, else center.
  float anchorX = 0.0;
  if (a_positionMode < 0.5) anchorX = -labelHalf.x + attachHalf.x;       // right label
  else if (a_positionMode < 1.5) anchorX = labelHalf.x - attachHalf.x;   // left label

  // Attachment center relative to the box (pre-rotation, Y-down). This offset
  // depends only on the box, the gap and the attachment size \u2014 never the shape.
  vec2 attachCenter;
  if (a_attachmentPlacement < 0.5) {
    // below
    attachCenter = boxCenter + vec2(anchorX, labelHalf.y + gap + attachHalf.y);
  } else if (a_attachmentPlacement < 1.5) {
    // above
    attachCenter = boxCenter + vec2(anchorX, -(labelHalf.y + gap + attachHalf.y));
  } else if (a_attachmentPlacement < 2.5) {
    // left, top-aligned
    attachCenter = boxCenter + vec2(-(labelHalf.x + gap + attachHalf.x), -labelHalf.y + attachHalf.y);
  } else {
    // right, top-aligned
    attachCenter = boxCenter + vec2(labelHalf.x + gap + attachHalf.x, -labelHalf.y + attachHalf.y);
  }

  // Rotate the whole assembly by the label angle, around the node center.
  vec2 rotatedCenter = labelRotMat * attachCenter;
  vec2 cornerOffset = labelRotMat * (a_quadCorner * attachHalf);

  // Node center in screen px (Y-down).
  vec2 nodeScreen = vec2(
    (nodeClip.x + 1.0) * u_resolution.x,
    (1.0 - nodeClip.y) * u_resolution.y
  ) * 0.5;

  // Snap node center to the pixel grid so label/backdrop/attachment move as one.
  vec2 snapDelta = (round(nodeScreen) - nodeScreen) * u_labelPixelSnapping;

  // Snap the quad's top-left to integer pixels so atlas texels map 1:1.
  vec2 centerScreen = nodeScreen + rotatedCenter + snapDelta;
  vec2 topLeft = centerScreen - attachHalf;
  topLeft = mix(topLeft, round(topLeft), u_labelPixelSnapping);
  centerScreen = topLeft + attachHalf;

  vec2 vertexScreen = centerScreen + cornerOffset;
  gl_Position = vec4(
    vertexScreen.x * 2.0 / u_resolution.x - 1.0,
    1.0 - vertexScreen.y * 2.0 / u_resolution.y,
    0.0, 1.0
  );

  vec2 texOrigin = a_atlasRect.xy / u_atlasSize;
  vec2 texSize = a_atlasRect.zw / u_atlasSize;
  vec2 uv = (a_quadCorner + 1.0) / 2.0;
  v_texCoord = texOrigin + uv * texSize;
}
`);return a}var Eu=`#version 300 es
precision highp float;

uniform sampler2D u_atlasTexture;

in vec2 v_texCoord;

layout(location = 0) out vec4 fragColor;
#ifdef PICKING_MODE
layout(location = 1) out vec4 pickColor;
#endif

void main() {
  // Canvas textures are premultiplied; output directly for (ONE, 1-SRC_ALPHA) blending
  vec4 color = texture(u_atlasTexture, v_texCoord);
  if (color.a < 0.01) discard;
  fragColor = color;
#ifdef PICKING_MODE
  pickColor = vec4(0.0); // Attachments are not pickable
#endif
}
`;function Au(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:kt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=Su(),h=Eu,f=(function(c){function g(){var b;ne(this,g);for(var m=arguments.length,p=new Array(m),v=0;v<m;v++)p[v]=arguments[v];return b=fe(this,g,[].concat(p)),E(b,"totalCount",0),E(b,"bufferCapacity",0),b}return ge(g,c),ae(g,[{key:"getDefinition",value:function(){var m=WebGL2RenderingContext,p=m.FLOAT,v=m.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:h,METHOD:v,UNIFORMS:["u_matrix","u_resolution","u_labelPixelSnapping","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_atlasTexture"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:p},{name:"a_atlasRect",size:4,type:p},{name:"a_attachmentSize",size:2,type:p},{name:"a_positionMode",size:1,type:p},{name:"a_attachmentPlacement",size:1,type:p},{name:"a_labelWidth",size:1,type:p},{name:"a_labelHeight",size:1,type:p},{name:"a_textHeight",size:1,type:p},{name:"a_labelAngle",size:1,type:p}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:p}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processAttachment",value:function(m,p){var v=this.floats,y=m*this.STRIDE;v[y++]=p.nodeIndex,v[y++]=p.atlasX,v[y++]=p.atlasY,v[y++]=p.atlasW,v[y++]=p.atlasH,v[y++]=p.attachWidth,v[y++]=p.attachHeight,v[y++]=p.positionMode,v[y++]=p.attachmentPlacement,v[y++]=p.labelWidth,v[y++]=p.labelHeight,v[y++]=p.textHeight,v[y++]=p.labelAngle}},{key:"setUniforms",value:function(m,p){var v=p.gl,y=p.uniformLocations;v.uniformMatrix3fv(y.u_matrix,!1,m.matrix),v.uniform2f(y.u_resolution,m.width*m.pixelRatio,m.height*m.pixelRatio),v.uniform1f(y.u_labelPixelSnapping,m.labelPixelSnapping),v.uniform1f(y.u_sizeRatio,m.sizeRatio),v.uniform1f(y.u_correctionRatio,m.correctionRatio),v.uniform1f(y.u_cameraAngle,m.cameraAngle),v.uniform1f(y.u_pixelRatio,m.pixelRatio),v.uniform1f(y.u_labelMargin,g.labelMargin),v.uniform1f(y.u_zoomLabelSizeRatio,1/u(m.zoomRatio)),v.uniform1i(y.u_nodeDataTexture,m.nodeDataTextureUnit),v.uniform1i(y.u_nodeDataTextureWidth,m.nodeDataTextureWidth),v.uniform1i(y.u_nodeFrameTexture,m.nodeFrameTextureUnit),v.uniform1i(y.u_nodeFrameTextureWidth,m.nodeFrameTextureWidth),v.uniform1i(y.u_atlasTexture,an)}},{key:"reallocateAttachments",value:function(m){this.totalCount=m,m>this.bufferCapacity&&(this.bufferCapacity=Math.max(m,Math.ceil(this.bufferCapacity*1.5)||10),ye(g,"reallocate",this,3)([this.bufferCapacity]))}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(m,p){var v=p.gl;this.totalCount!==0&&v.drawArraysInstanced(m,0,this.VERTICES,this.totalCount)}}])})(st);return E(f,"labelMargin",l),new f(a,n,t)}var Ke=5;function ea(a,n){return[].concat(J(n),J(a.map(function(t){var e;return{attributes:(e=t.attributes)!==null&&e!==void 0?e:[]}})))}function Ft(a,n){var t=Fe(ea(a,n)),e=new Set(a.flatMap(function(u){var d;return(d=u.attributes)!==null&&d!==void 0?d:[]}).map(function(u){return u.name.replace(/^a_/,"")})),r={},i={},o=U(e),s;try{for(o.s();!(s=o.n()).done;){var l=s.value;r[l]=t.offsets[l],i[l]=t.specs[l]}}catch(u){o.e(u)}finally{o.f()}return H(H({},t),{},{offsets:r,specs:i})}function Du(a,n){var t=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pickingPadding","u_nodeDataTexture","u_layerAttributeTexture"]),e=new Set,r=a.flatMap(function(c){return c.uniforms}).filter(function(c){return t.has(c.name)||e.has(c.name)?!1:(e.add(c.name),!0)}).map(function(c){return"uniform ".concat(c.type," ").concat(c.name,";")}).join(`
`),i=n.flatMap(function(c){return c.uniforms}).filter(function(c){return t.has(c.name)||e.has(c.name)?!1:(e.add(c.name),!0)}).map(function(c){return"uniform ".concat(c.type," ").concat(c.name,";")}).join(`
`),o=ea(a,n),s=new Set,l=o.flatMap(function(c){return c.attributes}).filter(function(c){var g=c.name.replace(/^a_/,"");return s.has(g)?!1:(s.add(g),!0)}).map(function(c){var g=c.name.replace(/^a_/,""),b=c.size===1?"float":"vec".concat(c.size);return"out ".concat(b," v_").concat(g,";")}).join(`
`),u=Lt(Fe(o),{varPrefix:"layer",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),d=u.fetchCode,h=u.varyingAssignments,f=`#version 300 es

// Standard node attributes (per instance) - minimal buffer usage
in float a_nodeIndex;  // Index into node data texture AND layer attribute texture
in vec4 a_id;          // Node ID for picking
in float a_opacity;    // Node opacity, applied once to the final fragment

// Constant attributes (per vertex, same for all instances)
in vec2 a_quadCorner;  // (-1,-1), (1,-1), (1,1), (-1,1) for quad corners

// Standard uniforms
uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
#ifdef PICKING_MODE
uniform float u_pickingPadding;
#endif
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;

// Layer attribute texture uniforms
uniform sampler2D u_layerAttributeTexture;
uniform int u_layerAttributeTextureWidth;
uniform int u_layerAttributeTexelsPerNode;

`.concat(r,`
`).concat(i,`

// Standard varyings
out vec2 v_uv;                    // Normalized coordinates [-1, 1]
out vec4 v_id;
out float v_opacity;
out float v_antialiasingWidth;    // Width for antialiasing in UV space
out float v_pixelSize;            // Node size in pixels (for pixel-mode borders)
out float v_pixelToUV;            // Conversion factor: multiply by this to convert screen pixels to UV units
out float v_shapeId;              // Shape ID for multi-shape programs

// Layer varyings
`).concat(l,`

// Node-data fetch helpers (geometry texel + rotation-flags texel)
`).concat(He,`
`).concat(ot,`

void main() {
  // Fetch node geometry: vec4(x, y, size, shapeId).
  int nodeIdx = int(a_nodeIndex);

  // Hidden nodes are flagged with a negative row by NodeProgram.process(). Push
  // the whole quad outside the clip volume: every vertex lands at the same
  // out-of-range position, so the primitive is fully clipped and rasterizes
  // nothing \u2014 neither to the frame buffer nor to the picking buffer.
  if (nodeIdx < 0) {
    gl_Position = vec4(2.0, 0.0, 0.0, 1.0);
    v_id = vec4(0.0);
    return;
  }

  vec4 nodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  vec2 a_position = nodeData.xy;
  float a_size = nodeData.z;
  v_shapeId = nodeData.w;  // Pass shape ID to fragment shader

  // Per-node rotation alignment: 0 = viewport (screen-upright), 1 = graph.
  float nodeRotation = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx).r;

`).concat(d,`

  // Calculate the actual size in pixels
  float size = a_size * u_correctionRatio / u_sizeRatio * 2.0;

  // In PICKING_MODE, inflate the quad by nodePickingPadding pixels on each side
  #ifdef PICKING_MODE
    float paddedSize = size + u_pickingPadding * u_correctionRatio;
    vec2 offset = a_quadCorner * paddedSize;
  #else
    vec2 offset = a_quadCorner * size;
  #endif
  // Counter-rotate the quad offset so viewport-aligned nodes stay upright as the
  // camera turns. Graph-aligned nodes (nodeRotation=1) skip it and turn with the
  // camera. The angle scales by (1 - nodeRotation) so both fall out of one path.
  {
    float ca = u_cameraAngle * (1.0 - nodeRotation);
    float c = cos(ca);
    float s = sin(ca);
    offset = mat2(c, s, -s, c) * offset;
  }
  vec2 position = a_position + offset;

  gl_Position = vec4(
    (u_matrix * vec3(position, 1)).xy,
    0,
    1
  );

  // In PICKING_MODE, UV is scaled beyond [-1, 1] to match the inflated quad
  #ifdef PICKING_MODE
    v_uv = a_quadCorner * (paddedSize / size);
  #else
    v_uv = a_quadCorner;
  #endif

  // Pass ID to fragment shader
  v_id = a_id;
  v_opacity = a_opacity;

  // Pass pixel size for layers that need pixel-mode calculations
  // Multiply by 2 because 'size' is half-width (offset from center), not full diameter
  v_pixelSize = size * 2.0;

  // Conversion factor from screen pixels to UV units
  // Same derivation as v_antialiasingWidth which represents ~1 pixel in UV space
  // P pixels in UV space = P * u_correctionRatio / size
  v_pixelToUV = u_correctionRatio / size;

  // We use an antialiasing width of 1px (so v_pixelToUV)
  v_antialiasingWidth = v_pixelToUV;

  // Pass layer attributes to fragment shader (fetched from texture)
`).concat(h,`
}
`);return f}function wu(a,n,t){var e=arguments.length>3&&arguments[3]!==void 0?arguments[3]:!0,r=n.map(function(g,b){var m="layer_".concat(g.name),p=[].concat(J(g.attributes.map(function(v){return"v_".concat(v.name.replace(/^a_/,""))})),J(g.uniforms.map(function(v){return v.name}))).join(", ");return"  // Layer ".concat(b+1,": ").concat(g.name,`
  color = blendOver(color, `).concat(m,"(").concat(p,"));")}).join(`

`),i=new Set(["u_correctionRatio"]),o=new Set,s=a.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),l=n.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),u=new Set,d=ea(a,n).flatMap(function(g){return g.attributes}).filter(function(g){var b=g.name.replace(/^a_/,"");return u.has(b)?!1:(u.add(b),!0)}).map(function(g){var b=g.name.replace(/^a_/,""),m=g.size===1?"float":"vec".concat(g.size);return"in ".concat(m," v_").concat(b,";")}).join(`
`),h=Ua(a),f=du(a,t),c=`#version 300 es
precision highp float;

// Standard varyings
in vec2 v_uv;
in vec4 v_id;
in float v_opacity;
in float v_antialiasingWidth;
in float v_pixelSize;
in float v_pixelToUV;
in float v_shapeId;  // Shape ID for multi-shape programs

// Standard uniforms (needed for some layer calculations like pixel-mode borders)
uniform float u_correctionRatio;
#ifdef PICKING_MODE
uniform float u_pickingPadding;
#endif

// Shape uniforms
`.concat(s,`

// Layer uniforms
`).concat(l,`

// Layer varyings
`).concat(d,`

// Fragment output (single target - picking handled via separate pass)
out vec4 fragColor;

// LayerContext struct - provides rendering context to all layers
struct LayerContext {
  float sdf;             // Signed distance from shape boundary (negative inside)
  vec2 uv;               // UV coordinates [-1, 1], center at (0,0)
  float shapeSize;       // Effective shape size (~diameter) in UV space (1.0 - aaWidth)
  float shapeHalfSize;   // Effective shape half size (~radius) in UV space
  float pixelSize;       // Node full size (~diameter) in screen pixels
  float aaWidth;         // Anti-aliasing width for smooth transitions
  float correctionRatio; // Scaling factor for consistent rendering across zoom levels
  float pixelToUV;       // Conversion factor: multiply screen pixels by this to get UV units
  float inradiusFactor;  // Ratio of inradius to circumradius (shape depth factor)
};

LayerContext context;  // Global instance, populated before layer calls

// Alpha "over" compositing for layer blending
vec4 blendOver(vec4 bg, vec4 fg) {
  float a = fg.a;
  return vec4(mix(bg.rgb, fg.rgb, a), bg.a + a * (1.0 - bg.a));
}

// SDF shape functions
`).concat(h,`

// Shape selector function (sets context.sdf and context.inradiusFactor)
`).concat(f,`

// Layer functions
`).concat(n.map(function(g){return g.glsl}).join(`

`),`

void main() {
  // 1. Setup LayerContext (available to all layer functions)
  context.shapeSize = 1.0 - v_antialiasingWidth;
  context.shapeHalfSize = context.shapeSize * 0.5;
  context.pixelSize = v_pixelSize;
  context.uv = v_uv;
  context.aaWidth = v_antialiasingWidth;
  context.correctionRatio = u_correctionRatio;
  context.pixelToUV = v_pixelToUV;

  // Query shape SDF based on shapeId (sets context.sdf and context.inradiusFactor)
  queryNodeSDF(int(v_shapeId), v_uv, context.shapeSize);

  // 2. Early discard for pixels fully outside the shape (with AA margin)
  // In PICKING_MODE, allow extra fragments up to the picking padding distance
  #ifdef PICKING_MODE
    if (context.sdf > u_pickingPadding * v_pixelToUV + context.aaWidth) discard;
  #else
    if (context.sdf > context.aaWidth) discard;
  #endif

  // 3. Apply layers sequentially with "over" compositing
  vec4 color = vec4(0.0);

`).concat(r,`

  #ifdef PICKING_MODE
    // Picking pass: output node ID for pixels within the picking area.
    if (context.sdf > u_pickingPadding * v_pixelToUV) discard;
    fragColor = v_id;
  #else
`).concat(e?`    // Visual pass: apply antialiasing at shape boundary, node opacity once
    // smoothstep provides smooth transition from opaque to transparent
    float alpha = smoothstep(context.aaWidth, -context.aaWidth, context.sdf) * v_opacity;`:`    // Visual pass: hard-edged (no anti-aliasing gradient) shape boundary, node opacity applied once
    float alpha = (context.sdf < 0.0 ? 1.0 : 0.0) * v_opacity;`,`
    // Mix with transparent to fade both color AND alpha together (avoids bright halo)
    fragColor = mix(vec4(0.0), color, alpha);
  #endif
}
`);return c}function Ru(a,n){var t=new Set;return t.add("u_matrix"),t.add("u_sizeRatio"),t.add("u_correctionRatio"),t.add("u_cameraAngle"),t.add("u_pickingPadding"),t.add("u_nodeDataTexture"),t.add("u_nodeDataTextureWidth"),t.add("u_layerAttributeTexture"),t.add("u_layerAttributeTextureWidth"),t.add("u_layerAttributeTexelsPerNode"),a.forEach(function(e){e.uniforms.forEach(function(r){return t.add(r.name)})}),n.forEach(function(e){e.uniforms.forEach(function(r){return t.add(r.name)})}),Array.from(t)}function Lu(a){var n=WebGL2RenderingContext,t=n.UNSIGNED_BYTE,e=n.FLOAT;return[{name:"a_nodeIndex",size:1,type:e},{name:"a_id",size:4,type:t,normalized:!0},{name:"a_opacity",size:1,type:e}]}function zi(a){var n=a.shapes,t=a.layers,e=a.shapeGlobalIds,r=a.antialias,i=r===void 0?!0:r;return{vertexShader:Du(n,t),fragmentShader:wu(n,t,e,i),uniforms:Ru(n,t),attributes:Lu()}}function eo(a,n,t){var e=Ft(a,n),r=e.specs;return Object.keys(r).map(function(i){return"flat ".concat(t," ").concat(r[i].size===1?"float":"vec".concat(r[i].size)," v_").concat(i,";")}).join(`
`)}function Cu(a){var n,t,e=a.shapes,r=a.layers,i=a.shapeGlobalIds,o=Lt(Ft(e,r),{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),s=e.length===1?"float inradiusFactor = ".concat(me((n=e[0].inradiusFactor)!==null&&n!==void 0?n:1),";"):"float inradiusFactor = ".concat(me((t=e[0].inradiusFactor)!==null&&t!==void 0?t:1),`;
  switch (int(shapeId)) {
`).concat(e.map(function(u,d){var h,f=i?i[d]:d;return"    case ".concat(f,": inradiusFactor = ").concat(me((h=u.inradiusFactor)!==null&&h!==void 0?h:1),"; break;")}).join(`
`),`
    default: break;
  }`),l=`#version 300 es

in float a_nodeIndex;
in float a_labelWidth;
in float a_labelHeight;
in float a_textHeight;
in float a_positionMode;
in float a_labelAngle;
in vec4 a_backdropColor;
in vec4 a_backdropShadowColor;
in float a_backdropShadowBlur;
in float a_backdropPadding;
in vec4 a_backdropBorderColor;
in vec4 a_backdropExtra; // [borderWidth, cornerRadius, labelPadding, area]
in vec2 a_labelBoxOffset;
in vec2 a_quadCorner;

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
uniform vec2 u_resolution;
uniform float u_pixelRatio;
uniform float u_labelMargin;
uniform float u_zoomLabelSizeRatio;
uniform float u_labelPixelSnapping;
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_nodeFrameTexture;
uniform int u_nodeFrameTextureWidth;
uniform sampler2D u_layerAttributeTexture;
uniform int u_layerAttributeTextureWidth;
uniform int u_layerAttributeTexelsPerNode;

out vec2 v_uv;
out vec2 v_nodeCenter;
out float v_nodeRadius;
out vec2 v_labelCenter;
out vec2 v_labelHalfSize;
out float v_aaWidth;
out float v_shapeId;
out float v_nodeRotation;
out float v_labelAngle;
out vec4 v_backdropColor;
out vec4 v_backdropShadowColor;
out float v_backdropShadowBlur;
out float v_backdropPadding;
out vec4 v_backdropBorderColor;
out float v_backdropBorderWidth;
out float v_backdropCornerRadius;
out float v_backdropArea;
`.concat(eo(e,r,"out"),`

`).concat(He,`
`).concat(ot,`
`).concat(Pt,`

void main() {
  int nodeIdx = int(a_nodeIndex);
`).concat(o.fetchCode,`
`).concat(o.varyingAssignments,`

  // Node data: (x, y, size, shapeId). shapeId (global) is forwarded to the
  // fragment shader, which keeps the shape SDF for the outline.
  vec4 nodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  vec2 nodePosition = nodeData.xy;
  float nodeSize = nodeData.z;
  float shapeId = nodeData.w;

  // Per-node rotation alignment (0 = viewport, 1 = graph). nodeRotation drives
  // the fragment's shape outline; labelRotation turns the label box with camera.
  vec4 nodeFlags = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  float nodeRotation = nodeFlags.r;
  float labelRotation = nodeFlags.g;
  v_nodeRotation = nodeRotation;

  `).concat(Ki,`
  // CSS pixel attributes are multiplied by u_pixelRatio to match nodeRadiusPixels,
  // which is already in physical pixels (nodeRadiusNDC * u_resolution.x / 2.0).
  float padding = a_backdropPadding * u_pixelRatio;
  float shadowBlur = a_backdropShadowBlur * u_pixelRatio;
  // Unpack a_backdropExtra: [borderWidth, cornerRadius, labelPadding, area]
  float borderWidth = a_backdropExtra.x * u_pixelRatio;
  float cornerRadius = a_backdropExtra.y * u_pixelRatio;
  float labelPad = a_backdropExtra.z * u_pixelRatio;
  float backdropArea = a_backdropExtra.w;
  // Use 2x shadowBlur so the Gaussian fully decays before the quad edge
  float totalExpansion = shadowBlur * 2.0 + borderWidth;
  float enlargedRadius = nodeRadiusPixels + padding;

  // Circumscribed radius for the quad bounds. Non-circular shapes reach past
  // their inradius out to their circumradius (= enlargedRadius / inradiusFactor),
  // in any direction once rotated. The axis-aligned quad must contain that, or
  // the fill/shadow gets clipped (square corners, triangle tip, etc.).
  `).concat(s,`
  float boundRadius = enlargedRadius / inradiusFactor;

  // Apply zoom-dependent label size scaling
  float zoomScale = u_zoomLabelSizeRatio;
  float labelW = a_labelWidth * zoomScale * u_pixelRatio;
  float labelH = a_labelHeight * zoomScale * u_pixelRatio;
  float labelMargin = u_labelMargin * zoomScale * u_pixelRatio;

  // Only apply labelPad when a label is actually present
  float effectiveLabelPad = labelW > 0.0 ? labelPad : 0.0;
  vec2 labelHalfSize = vec2(labelW * 0.5 + effectiveLabelPad, labelH * 0.5 + effectiveLabelPad);
  vec2 labelOffset = vec2(0.0);

  // Effective label angle: intrinsic angle plus the camera angle when the label
  // is graph-aligned, so the box orbits the node in lockstep with the frame-pass.
  float labelAngle = a_labelAngle - labelRotation * u_cameraAngle;
  float la_c = cos(labelAngle);
  float la_s = sin(labelAngle);
  mat2 labelRotMat = mat2(la_c, -la_s, la_s, la_c);

  vec3 nodeClip = u_matrix * vec3(nodePosition, 1.0);
  vec2 snapDelta = vec2(0.0);

  if (labelW > 0.0) {
    // labelW > 0.0 means this node's label is displayed, so the frame-pass wrote
    // its edge distance this frame.
    float edgeDistPixels = nodeRadiusPixels * readFrameTexel(u_nodeFrameTexture, u_nodeFrameTextureWidth, nodeIdx).r;
    // labelMargin matches the label shader's margin (gap from node edge to text)
    float labelStart = edgeDistPixels + labelMargin;
    // Above/below center on the actual glyph height, not the font line box, so
    // the box stays aligned with the rendered text (matches labelBoxCenter()).
    float textHalf = a_textHeight * zoomScale * u_pixelRatio * 0.5;

    // Snap node center to pixel grid so label/backdrop/attachment move as a unit
    vec2 nodeScreen = vec2(
      (nodeClip.x + 1.0) * u_resolution.x,
      (1.0 - nodeClip.y) * u_resolution.y
    ) * 0.5;
    snapDelta = (round(nodeScreen) - nodeScreen) * u_labelPixelSnapping;

    if (a_positionMode < 0.5) {
      // Right: box spans from node center to text end + padding
      float boxRightEdge = labelStart + labelW + labelPad;
      labelOffset = vec2(boxRightEdge * 0.5, 0.0);
      labelHalfSize.x = boxRightEdge * 0.5;
    } else if (a_positionMode < 1.5) {
      // Left: mirror of right
      float boxLeftEdge = labelStart + labelW + labelPad;
      labelOffset = vec2(-boxLeftEdge * 0.5, 0.0);
      labelHalfSize.x = boxLeftEdge * 0.5;
    } else if (a_positionMode < 2.5) {
      // Above: text bottom at labelStart, centered horizontally
      labelOffset = vec2(0.0, -(labelStart + textHalf));
    } else if (a_positionMode < 3.5) {
      // Below: text top at labelStart, centered horizontally
      labelOffset = vec2(0.0, labelStart + textHalf);
    }
    // over (>=4): labelOffset stays (0,0) \u2014 text centered on the node.

    // The attachment-cover shift is in label space (like the attachment itself),
    // so rotate it together with the position offset \u2014 otherwise the box drifts
    // off the attachment as the label angle grows.
    labelOffset = labelRotMat * (labelOffset + a_labelBoxOffset * zoomScale * u_pixelRatio);
  }

  // For node-only mode, zero out label dimensions
  if (backdropArea > 0.5 && backdropArea < 1.5) {
    labelHalfSize = vec2(0.0);
    labelOffset = vec2(0.0);
  }

  vec2 minBound, maxBound;

  bool hasLabelBounds = labelW > 0.0 && (backdropArea > 1.5 || backdropArea < 0.5);

  if (hasLabelBounds) {
    // Union with label bounds (area=both) or label-only bounds (area=label)
    vec2 labelMin, labelMax;

    if (a_labelAngle != 0.0) {
      vec2 corner1 = labelOffset + labelRotMat * vec2(-labelHalfSize.x, -labelHalfSize.y);
      vec2 corner2 = labelOffset + labelRotMat * vec2(labelHalfSize.x, -labelHalfSize.y);
      vec2 corner3 = labelOffset + labelRotMat * vec2(labelHalfSize.x, labelHalfSize.y);
      vec2 corner4 = labelOffset + labelRotMat * vec2(-labelHalfSize.x, labelHalfSize.y);
      labelMin = min(min(corner1, corner2), min(corner3, corner4));
      labelMax = max(max(corner1, corner2), max(corner3, corner4));
    } else {
      labelMin = labelOffset - labelHalfSize;
      labelMax = labelOffset + labelHalfSize;
    }

    if (backdropArea > 1.5) {
      // Label-only: bounds from label rect only
      minBound = labelMin - totalExpansion;
      maxBound = labelMax + totalExpansion;
    } else {
      // Both: union of node + label
      minBound = min(-vec2(boundRadius), labelMin) - totalExpansion;
      maxBound = max(vec2(boundRadius), labelMax) + totalExpansion;
    }
  } else {
    // Node-only or no visible label
    float totalRadius = boundRadius + totalExpansion;
    minBound = -vec2(totalRadius);
    maxBound = vec2(totalRadius);
  }

  vec2 quadSize = maxBound - minBound;
  vec2 quadCenter = (minBound + maxBound) * 0.5;

  vec2 localPos = quadCenter + a_quadCorner * quadSize * 0.5 + snapDelta;
  vec2 ndcOffset = localPos * 2.0 / u_resolution;
  ndcOffset.y = -ndcOffset.y;

  gl_Position = vec4(nodeClip.xy + ndcOffset, 0.0, 1.0);

  v_uv = localPos;
  v_nodeCenter = vec2(0.0);
  v_nodeRadius = nodeRadiusPixels;
  v_labelCenter = labelOffset;
  v_labelHalfSize = labelHalfSize;
  v_aaWidth = 1.0;
  v_shapeId = shapeId;
  v_labelAngle = labelAngle;
  v_backdropColor = a_backdropColor;
  v_backdropShadowColor = a_backdropShadowColor;
  v_backdropShadowBlur = a_backdropShadowBlur;
  v_backdropPadding = a_backdropPadding;
  v_backdropBorderColor = a_backdropBorderColor;
  v_backdropBorderWidth = borderWidth;
  v_backdropCornerRadius = cornerRadius;
  v_backdropArea = backdropArea;
}
`);return l}function ku(a){var n=a.shapes,t=a.layers,e=a.shapeGlobalIds,r=Ua(n),i=nn(n).map(function(f){return"uniform ".concat(f.type," ").concat(f.name,";")}).join(`
`),o=function(c){return Kn(c,"nodeUV","1.0")},s;if(n.length===1)s="float nodeSdfNormalized = ".concat(o(n[0]),";");else{var l=n.map(function(f,c){var g=e?e[c]:c;return"    case ".concat(g,": nodeSdfNormalized = ").concat(o(f),"; break;")}).join(`
`),u=o(n[0]);s=`float nodeSdfNormalized;
  int shapeId = int(v_shapeId);
  switch (shapeId) {
`.concat(l,`
    default: nodeSdfNormalized = `).concat(u,`;
  }`)}var d=`float effectiveRadius = max(enlargedRadius - cornerRadius, 0.01);
  float ca = u_cameraAngle * v_nodeRotation;
  float ca_c = cos(ca);
  float ca_s = sin(ca);
  vec2 rotatedScreenUV = mat2(ca_c, -ca_s, ca_s, ca_c) * screenUV;
  vec2 nodeUV = vec2(rotatedScreenUV.x, -rotatedScreenUV.y) / effectiveRadius;`,h=`#version 300 es
precision highp float;

in vec2 v_uv;
in vec2 v_nodeCenter;
in float v_nodeRadius;
in vec2 v_labelCenter;
in vec2 v_labelHalfSize;
in float v_aaWidth;
in float v_shapeId;
in float v_nodeRotation;
in float v_labelAngle;
in vec4 v_backdropColor;
in vec4 v_backdropShadowColor;
in float v_backdropShadowBlur;
in float v_backdropPadding;
in vec4 v_backdropBorderColor;
in float v_backdropBorderWidth;
in float v_backdropCornerRadius;
in float v_backdropArea;
`.concat(eo(n,t,"in"),`

uniform float u_cameraAngle;
`).concat(i,`

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec4 fragPicking;

`).concat(r,`
`).concat(cu,`
`).concat(fu,`
`).concat(gu,`
`).concat(pu,`

void main() {
  vec4 backdropColor = v_backdropColor;
  vec4 shadowColor = v_backdropShadowColor;
  float shadowBlur = v_backdropShadowBlur;
  float padding = v_backdropPadding;
  vec4 borderColor = v_backdropBorderColor;
  float borderWidth = v_backdropBorderWidth;
  float cornerRadius = v_backdropCornerRadius;

  float enlargedRadius = v_nodeRadius + padding;
  vec2 screenUV = v_uv - v_nodeCenter;
  `).concat(d,`

  // Query the correct shape SDF based on shapeId
  `).concat(s,`
  float nodeSdfPixels = nodeSdfNormalized * effectiveRadius - cornerRadius;

  // Label SDF with optional corner radius and rotation
  float labelSdfPixels;
  if (v_labelHalfSize.x > 0.0) {
    vec2 labelP = v_uv - v_labelCenter;
    labelSdfPixels = sdfRoundedRotatedBox(labelP, v_labelHalfSize, v_labelAngle, cornerRadius);
  } else {
    labelSdfPixels = 10000.0;
  }

  // Select area: 0=both, 1=node, 2=label
  float combinedSdf;
  if (v_backdropArea > 1.5) {
    combinedSdf = labelSdfPixels;
  } else if (v_backdropArea > 0.5) {
    combinedSdf = nodeSdfPixels;
  } else {
    combinedSdf = min(nodeSdfPixels, labelSdfPixels);
  }

  // Fill + border composite
  float outerEdge = smoothstep(v_aaWidth, -v_aaWidth, combinedSdf);
  vec4 background;
  if (borderWidth > 0.5) {
    float innerEdge = smoothstep(v_aaWidth, -v_aaWidth, combinedSdf + borderWidth);
    float fillAlpha = innerEdge * backdropColor.a;
    vec4 fill = vec4(backdropColor.rgb * fillAlpha, fillAlpha);
    float borderAlpha = (outerEdge - innerEdge) * borderColor.a;
    vec4 border = vec4(borderColor.rgb * borderAlpha, borderAlpha);
    background = fill + border * (1.0 - fill.a);
  } else {
    float fillAlpha = outerEdge * backdropColor.a;
    background = vec4(backdropColor.rgb * fillAlpha, fillAlpha);
  }

  // Gaussian-like shadow falloff (mimics canvas shadowBlur)
  vec4 shadow = vec4(0.0);
  float sigma = shadowBlur / 2.5;
  if (sigma > 0.001) {
    float shadowDist = max(0.0, combinedSdf);
    float shadowAlpha = exp(-(shadowDist * shadowDist) / (2.0 * sigma * sigma)) * shadowColor.a;
    shadow = vec4(shadowColor.rgb * shadowAlpha, shadowAlpha);
  }

  fragColor = background + shadow * (1.0 - background.a);
  fragPicking = vec4(0.0);
}
`);return h}function Pu(a){var n=["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"],t=U(nn(a)),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.includes(r.name)||n.push(r.name)}}catch(i){t.e(i)}finally{t.f()}return n}function Fu(a){return{vertexShader:Cu(a),fragmentShader:ku(a),uniforms:Pu(a.shapes)}}function Iu(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=e.shapes,u=e.layers,d=e.getAttributeTexture,h=e.shapeGlobalIds;if(l.length===0)throw new Error("createBackdropProgram: at least one shape must be provided in 'shapes'");var f=(r=s.margin)!==null&&r!==void 0?r:5,c=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},g={shapes:l,layers:u,shapeGlobalIds:h},b=Fu(g),m=(function(p){function v(){var y;ne(this,v);for(var T=arguments.length,_=new Array(T),S=0;S<T;S++)_[S]=arguments[S];return y=fe(this,v,[].concat(_)),E(y,"totalBackdropCount",0),E(y,"bufferCapacity",0),y}return ge(v,p),ae(v,[{key:"getDefinition",value:function(){var T=WebGL2RenderingContext,_=T.FLOAT,S=T.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:b.vertexShader,FRAGMENT_SHADER_SOURCE:b.fragmentShader,METHOD:S,UNIFORMS:b.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:_},{name:"a_labelWidth",size:1,type:_},{name:"a_labelHeight",size:1,type:_},{name:"a_textHeight",size:1,type:_},{name:"a_positionMode",size:1,type:_},{name:"a_labelAngle",size:1,type:_},{name:"a_backdropColor",size:4,type:_},{name:"a_backdropShadowColor",size:4,type:_},{name:"a_backdropShadowBlur",size:1,type:_},{name:"a_backdropPadding",size:1,type:_},{name:"a_backdropBorderColor",size:4,type:_},{name:"a_backdropExtra",size:4,type:_},{name:"a_labelBoxOffset",size:2,type:_}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:_}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processBackdrop",value:function(T,_){var S=this.floats,D=this.STRIDE,w=T*D;S[w++]=_.nodeIndex,S[w++]=_.labelWidth,S[w++]=_.labelHeight,S[w++]=_.textHeight,S[w++]=Ct[_.position],S[w++]=_.labelAngle,S[w++]=_.backdropColor[0],S[w++]=_.backdropColor[1],S[w++]=_.backdropColor[2],S[w++]=_.backdropColor[3],S[w++]=_.backdropShadowColor[0],S[w++]=_.backdropShadowColor[1],S[w++]=_.backdropShadowColor[2],S[w++]=_.backdropShadowColor[3],S[w++]=_.backdropShadowBlur,S[w++]=_.backdropPadding,S[w++]=_.backdropBorderColor[0],S[w++]=_.backdropBorderColor[1],S[w++]=_.backdropBorderColor[2],S[w++]=_.backdropBorderColor[3],S[w++]=_.backdropBorderWidth,S[w++]=_.backdropCornerRadius,S[w++]=_.backdropLabelPadding,S[w++]=_.backdropArea,S[w++]=_.labelBoxOffset[0],S[w++]=_.labelBoxOffset[1]}},{key:"setUniforms",value:function(T,_){var S=_.gl,D=_.uniformLocations;S.uniformMatrix3fv(D.u_matrix,!1,T.matrix),S.uniform1f(D.u_sizeRatio,T.sizeRatio),S.uniform1f(D.u_correctionRatio,T.correctionRatio),S.uniform1f(D.u_cameraAngle,T.cameraAngle),S.uniform2f(D.u_resolution,T.width*T.pixelRatio,T.height*T.pixelRatio),S.uniform1f(D.u_pixelRatio,T.pixelRatio),S.uniform1f(D.u_labelMargin,v.labelMargin),S.uniform1f(D.u_zoomLabelSizeRatio,1/c(T.zoomRatio)),S.uniform1f(D.u_labelPixelSnapping,T.labelPixelSnapping),S.uniform1i(D.u_nodeDataTexture,T.nodeDataTextureUnit),S.uniform1i(D.u_nodeDataTextureWidth,T.nodeDataTextureWidth),S.uniform1i(D.u_nodeFrameTexture,T.nodeFrameTextureUnit),S.uniform1i(D.u_nodeFrameTextureWidth,T.nodeFrameTextureWidth);var w=d();D.u_layerAttributeTexture&&w&&(w.bind(Ke),S.uniform1i(D.u_layerAttributeTexture,Ke),S.uniform1i(D.u_layerAttributeTextureWidth,w.getTextureWidth()),S.uniform1i(D.u_layerAttributeTexelsPerNode,w.getTexelsPerItem()));var k=U(nn(l)),L;try{for(k.s();!(L=k.n()).done;){var N=L.value;this.setTypedUniform(N,_)}}catch(R){k.e(R)}finally{k.f()}}},{key:"hasNothingToRender",value:function(){return this.totalBackdropCount===0}},{key:"drawWebGL",value:function(T,_){var S=_.gl;this.totalBackdropCount!==0&&(this.isInstanced?S.drawArraysInstanced(T,0,this.VERTICES,this.totalBackdropCount):S.drawArrays(T,0,this.totalBackdropCount*this.VERTICES))}},{key:"reallocate",value:function(T){this.totalBackdropCount=T,T>this.bufferCapacity&&(this.bufferCapacity=Math.max(T,Math.ceil(this.bufferCapacity*1.5)||10),ye(v,"reallocate",this,3)([this.bufferCapacity]))}}])})(st);return E(m,"labelMargin",f),new m(a,n,t)}var to=(function(a){function n(){var t;ne(this,n);for(var e=arguments.length,r=new Array(e),i=0;i<e;i++)r[i]=arguments[i];return t=fe(this,n,[].concat(r)),E(t,"totalCharacterCount",0),E(t,"bufferCapacity",0),t}return ge(n,a),ae(n,[{key:"processLabel",value:function(e,r,i){if(i.hidden||!i.text)return 0;for(var o=i.text,s=o.length,l=0;l<s;l++){var u=o[l];this.processCharacter(r+l,i,u,l)}return s}},{key:"hasNothingToRender",value:function(){return this.totalCharacterCount===0}},{key:"drawWebGL",value:function(e,r){var i=r.gl;this.totalCharacterCount!==0&&(this.isInstanced?i.drawArraysInstanced(e,0,this.VERTICES,this.totalCharacterCount):i.drawArrays(e,0,this.totalCharacterCount*this.VERTICES))}},{key:"reallocate",value:function(e){this.totalCharacterCount=e,e>this.bufferCapacity&&(this.bufferCapacity=Math.max(e,Math.ceil(this.bufferCapacity*1.5)||1e3),ye(n,"reallocate",this,3)([this.bufferCapacity]))}}])})(st);function Gu(){var a=`#version 300 es

in float a_nodeIndex;
in vec4 a_id;
in vec4 a_color;
in float a_labelWidth;
in float a_labelHeight;
in float a_textHeight;
in float a_positionMode;
in float a_labelAngle;
in float a_padding;
in vec2 a_quadCorner;

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
uniform vec2 u_resolution;
uniform float u_pixelRatio;
uniform float u_labelMargin;
uniform float u_zoomLabelSizeRatio;
uniform float u_labelPixelSnapping;
uniform float u_pickingPadding;
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_nodeFrameTexture;
uniform int u_nodeFrameTextureWidth;

out vec4 v_id;
out vec4 v_color;

`.concat(He,`
`).concat(ot,`
`).concat(Pt,`
`).concat(Zi,`

void main() {
  int nodeIdx = int(a_nodeIndex);

  // Node data: (x, y, size, shapeId).
  vec4 nodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  vec2 nodePosition = nodeData.xy;
  float nodeSize = nodeData.z;

  // Shape-aware edge distance, read once from the shared frame texture.
  float edgeDist = readFrameTexel(u_nodeFrameTexture, u_nodeFrameTextureWidth, nodeIdx).r;

  // Per-node label rotation alignment: 0 = viewport, 1 = label turns with camera.
  float labelRotation = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx).g;

  `).concat(Ki,`

  float zoomScale = u_zoomLabelSizeRatio;
  float labelW = a_labelWidth * zoomScale * u_pixelRatio;
  float labelH = a_labelHeight * zoomScale * u_pixelRatio;
  float labelMargin = u_labelMargin * zoomScale * u_pixelRatio;
#ifdef PICKING_MODE
  float padding = u_pickingPadding * u_pixelRatio;
#else
  float padding = a_padding * u_pixelRatio;
#endif

  if (labelW <= 0.0) {
    gl_Position = vec4(2.0, 0.0, 0.0, 1.0);
    v_id = vec4(0.0);
    v_color = vec4(0.0);
    return;
  }

  vec2 labelHalfSize = vec2(labelW * 0.5 + padding, labelH * 0.5 + padding);
  vec2 labelOffset = vec2(0.0);

  // Graph-aligned labels add the camera angle so the rect orbits with the text.
  float labelAngle = a_labelAngle - labelRotation * u_cameraAngle;
  float la_c = cos(labelAngle);
  float la_s = sin(labelAngle);
  mat2 labelRotMat = mat2(la_c, -la_s, la_s, la_c);

  vec3 nodeClip = u_matrix * vec3(nodePosition, 1.0);
  vec2 nodeScreen = vec2(
    (nodeClip.x + 1.0) * u_resolution.x,
    (1.0 - nodeClip.y) * u_resolution.y
  ) * 0.5;
  vec2 snapDelta = (round(nodeScreen) - nodeScreen) * u_labelPixelSnapping;

  if (a_positionMode < 4.0) {
    float labelStart = nodeRadiusPixels * edgeDist + labelMargin;
    float textHalf = a_textHeight * zoomScale * u_pixelRatio * 0.5;
    // Box center uses the text half-size; the padding expands the quad below.
    labelOffset = labelRotMat * labelBoxCenter(a_positionMode, labelStart, vec2(labelW * 0.5, labelH * 0.5), textHalf);
  }

  // Rotate the rect with the label so it stays aligned with the (rotated) text.
  vec2 localPos = labelOffset + labelRotMat * (a_quadCorner * labelHalfSize);
  vec2 ndcOffset = (localPos + snapDelta) * 2.0 / u_resolution;
  ndcOffset.y = -ndcOffset.y;

  gl_Position = vec4(nodeClip.xy + ndcOffset, 0.0, 1.0);
  v_id = a_id;
  v_color = a_color;
}
`);return a}var Nu=`#version 300 es
precision highp float;

in vec4 v_id;
in vec4 v_color;

out vec4 fragColor;

void main() {
  #ifdef PICKING_MODE
    fragColor = v_id;
  #else
    if (v_color.a <= 0.0) discard;
    // v_color is non-premultiplied RGBA (0-1); convert to premultiplied for blending
    fragColor = vec4(v_color.rgb * v_color.a, v_color.a);
  #endif
}
`;function zu(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:kt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=Gu(),h=(function(f){function c(){var g;ne(this,c);for(var b=arguments.length,m=new Array(b),p=0;p<b;p++)m[p]=arguments[p];return g=fe(this,c,[].concat(m)),E(g,"totalCount",0),E(g,"bufferCapacity",0),g}return ge(c,f),ae(c,[{key:"getDefinition",value:function(){var b=WebGL2RenderingContext,m=b.FLOAT,p=b.UNSIGNED_BYTE,v=b.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:Nu,METHOD:v,UNIFORMS:["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:m},{name:"a_id",size:4,type:p,normalized:!0},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_labelWidth",size:1,type:m},{name:"a_labelHeight",size:1,type:m},{name:"a_textHeight",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_labelAngle",size:1,type:m},{name:"a_padding",size:1,type:m}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:m}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processLabelBackground",value:function(b,m){var p=this.floats,v=this.ints,y=b*this.STRIDE;p[y++]=m.nodeIndex,v[y++]=m.id,p[y++]=m.color,p[y++]=m.labelWidth,p[y++]=m.labelHeight,p[y++]=m.textHeight,p[y++]=m.positionMode,p[y++]=m.labelAngle,p[y++]=m.padding}},{key:"setUniforms",value:function(b,m){var p=m.gl,v=m.uniformLocations;p.uniformMatrix3fv(v.u_matrix,!1,b.matrix),p.uniform1f(v.u_sizeRatio,b.sizeRatio),p.uniform1f(v.u_correctionRatio,b.correctionRatio),p.uniform1f(v.u_cameraAngle,b.cameraAngle),p.uniform2f(v.u_resolution,b.width*b.pixelRatio,b.height*b.pixelRatio),p.uniform1f(v.u_pixelRatio,b.pixelRatio),p.uniform1f(v.u_labelMargin,c.labelMargin),p.uniform1f(v.u_zoomLabelSizeRatio,1/u(b.zoomRatio)),p.uniform1f(v.u_labelPixelSnapping,b.labelPixelSnapping),p.uniform1f(v.u_pickingPadding,b.labelPickingPadding),p.uniform1i(v.u_nodeDataTexture,b.nodeDataTextureUnit),p.uniform1i(v.u_nodeDataTextureWidth,b.nodeDataTextureWidth),p.uniform1i(v.u_nodeFrameTexture,b.nodeFrameTextureUnit),p.uniform1i(v.u_nodeFrameTextureWidth,b.nodeFrameTextureWidth)}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(b,m){var p=m.gl;this.totalCount!==0&&p.drawArraysInstanced(p.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(b){this.totalCount=b,b>this.bufferCapacity&&(this.bufferCapacity=Math.max(b,Math.ceil(this.bufferCapacity*1.5)||10),ye(c,"reallocate",this,3)([this.bufferCapacity]))}}])})(st);return E(h,"labelMargin",l),new h(a,n,t)}var Ne={fontSize:64,buffer:8,radius:24,cutoff:.25,maxTextureSize:2048,debounceTimeout:100},Zt=2,Jt=1e20;function Mu(a,n,t,e,r){var i=a+r*4,o=document.createElement("canvas");o.width=i,o.height=i;var s=o.getContext("2d",{willReadFrequently:!0});return s.font="".concat(e," ").concat(t," ").concat(a,"px ").concat(n),s.textBaseline="alphabetic",s.textAlign="left",s.fillStyle="black",{ctx:s,canvasSize:i,gridOuter:new Float64Array(i*i),gridInner:new Float64Array(i*i),f:new Float64Array(i),z:new Float64Array(i+1),v:new Uint16Array(i)}}function Bu(a,n,t,e,r){var i=a.ctx,o=a.canvasSize,s=i.measureText(n),l=s.width,u=s.actualBoundingBoxAscent,d=s.actualBoundingBoxDescent,h=s.actualBoundingBoxLeft,f=s.actualBoundingBoxRight,c=Math.ceil(u),g=Math.ceil(h),b=Math.max(0,Math.min(o-t,Math.ceil(h)+Math.ceil(f))),m=Math.min(o-t,c+Math.ceil(d)),p=b+2*t,v=m+2*t,y=Math.max(p*v,0),T=new Uint8ClampedArray(y),_={data:T,width:p,height:v,glyphWidth:b,glyphHeight:m,glyphTop:c,glyphLeft:g,glyphAdvance:l};if(b===0||m===0)return _;var S=a.gridInner,D=a.gridOuter;i.clearRect(t,t,b,m),i.fillText(n,t+g,t+c);var w=i.getImageData(t,t,b,m);D.fill(Jt,0,y),S.fill(0,0,y);for(var k=0;k<m;k++)for(var L=0;L<b;L++){var N=w.data[4*(k*b+L)+3]/255;if(N!==0){var R=(k+t)*p+L+t;if(N===1)D[R]=0,S[R]=Jt;else{var z=.5-N;D[R]=z>0?z*z:0,S[R]=z<0?z*z:0}}}Mi(D,0,0,p,v,p,a.f,a.v,a.z),Mi(S,t,t,b,m,p,a.f,a.v,a.z);for(var C=0;C<y;C++){var I=Math.sqrt(D[C])-Math.sqrt(S[C]);T[C]=Math.round(255-255*(I/e+r))}return _}function Mi(a,n,t,e,r,i,o,s,l){for(var u=n;u<n+e;u++)Bi(a,t*i+u,i,r,o,s,l);for(var d=t;d<t+r;d++)Bi(a,d*i+n,1,e,o,s,l)}function Bi(a,n,t,e,r,i,o){i[0]=0,o[0]=-Jt,o[1]=Jt,r[0]=a[n];for(var s=1,l=0,u=0;s<e;s++){r[s]=a[n+s*t];var d=s*s;do{var h=i[l];u=(r[s]-r[h]+d-h*h)/(s-h)/2}while(u<=o[l]&&--l>-1);l++,i[l]=s,o[l]=u,o[l+1]=Jt}for(var f=0,c=0;f<e;f++){for(;o[c+1]<f;)c++;var g=i[c],b=f-g;a[n+f*t]=r[g]+b*b}}var it=(function(a){function n(){var t,e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{};return ne(this,n),t=fe(this,n),E(t,"fonts",new Map),E(t,"textures",[]),E(t,"cursor",{x:0,y:0,rowHeight:0,atlasIndex:0}),E(t,"pendingGlyphs",[]),E(t,"debounceTimer",null),t.options=H(H({},Ne),e),t.canvas=document.createElement("canvas"),t.canvas.width=t.options.maxTextureSize,t.canvas.height=t.options.maxTextureSize,t.ctx=t.canvas.getContext("2d",{willReadFrequently:!0}),t.measureCanvas=document.createElement("canvas"),t.measureCtx=t.measureCanvas.getContext("2d"),t.textures.push(t.ctx.getImageData(0,0,1,1)),t}return ge(n,a),ae(n,[{key:"getFontKey",value:function(e){return"".concat(e.family,"-").concat(e.weight,"-").concat(e.style)}},{key:"registerFont",value:function(e){var r=this.getFontKey(e);if(this.fonts.has(r))return r;var i=Mu(this.options.fontSize,e.family,e.weight,e.style,this.options.buffer);return this.fonts.set(r,{descriptor:e,generator:i,glyphs:new Map}),r}},{key:"ensureGlyphs",value:function(e,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered. Call registerFont() first.'));var o=!1,s=U(e),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.codePointAt(0);d!==void 0&&(i.glyphs.has(d)||(this.pendingGlyphs.push({fontKey:r,charCode:d}),o=!0))}}catch(h){s.e(h)}finally{s.f()}o&&this.scheduleTextureGeneration()}},{key:"measureText",value:function(e,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered.'));var o=i.descriptor,s=o.family,l=o.weight,u=o.style;return this.measureCtx.font="".concat(u," ").concat(l," ").concat(this.options.fontSize,"px ").concat(s),this.measureCtx.measureText(e).width}},{key:"getGlyph",value:function(e,r){var i=this.fonts.get(r);if(i)return i.glyphs.get(e)}},{key:"getTextures",value:function(){return this.textures}},{key:"getFontCount",value:function(){return this.fonts.size}},{key:"getGlyphCount",value:function(){var e=0,r=U(this.fonts.values()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;e+=o.glyphs.size}}catch(s){r.e(s)}finally{r.f()}return e}},{key:"hasPendingGlyphs",value:function(){return this.pendingGlyphs.length>0}},{key:"flush",value:function(){this.debounceTimer&&(clearTimeout(this.debounceTimer),this.debounceTimer=null),this.generateTextures()}},{key:"destroy",value:function(){this.debounceTimer&&clearTimeout(this.debounceTimer),this.fonts.clear(),this.textures=[],this.pendingGlyphs=[],this.removeAllListeners()}},{key:"scheduleTextureGeneration",value:function(){var e=this;this.debounceTimer===null&&(this.options.debounceTimeout===null?this.generateTextures():this.debounceTimer=setTimeout(function(){e.debounceTimer=null,e.generateTextures()},this.options.debounceTimeout))}},{key:"generateTextures",value:function(){if(this.pendingGlyphs.length!==0){var e=this.options,r=e.maxTextureSize,i=e.buffer,o=e.radius,s=e.cutoff,l=U(this.pendingGlyphs),u;try{for(l.s();!(u=l.n()).done;){var d=u.value,h=d.fontKey,f=d.charCode,c=this.fonts.get(h);if(!(!c||c.glyphs.has(f))){var g=String.fromCodePoint(f),b=Bu(c.generator,g,i,o,s),m=b.width,p=b.height;this.cursor.x+m+Zt>r&&(this.cursor.x=0,this.cursor.y+=this.cursor.rowHeight+Zt,this.cursor.rowHeight=0),this.cursor.y+p+Zt>r&&(this.finalizeCurrentTexture(),this.cursor={x:0,y:0,rowHeight:0,atlasIndex:this.cursor.atlasIndex+1},this.ctx.clearRect(0,0,r,r));for(var v=b.data,y=new Uint8ClampedArray(m*p*4),T=0;T<v.length;T++){var _=T*4;y[_]=255,y[_+1]=255,y[_+2]=255,y[_+3]=v[T]}var S=new ImageData(y,m,p);this.ctx.putImageData(S,this.cursor.x,this.cursor.y);var D={charCode:f,width:b.glyphWidth,height:b.glyphHeight,bearingX:-b.glyphLeft-i,bearingY:b.glyphTop+i,advance:b.glyphAdvance,atlasX:this.cursor.x,atlasY:this.cursor.y,atlasWidth:m,atlasHeight:p,atlasIndex:this.cursor.atlasIndex};c.glyphs.set(f,D),this.cursor.x+=m+Zt,this.cursor.rowHeight=Math.max(this.cursor.rowHeight,p)}}}catch(w){l.e(w)}finally{l.f()}this.finalizeCurrentTexture(),this.pendingGlyphs=[],this.emit(n.ATLAS_UPDATED_EVENT,{textures:this.textures,glyphCount:this.getGlyphCount()})}}},{key:"finalizeCurrentTexture",value:function(){var e=this.options.maxTextureSize,r=this.cursor.y>0||this.cursor.rowHeight>0,i=Math.min(e,Math.max(this.cursor.x,r?e:1)),o=Math.min(e,this.cursor.y+this.cursor.rowHeight+Zt),s=this.ctx.getImageData(0,0,i,o);this.cursor.atlasIndex>=this.textures.length?this.textures.push(s):this.textures[this.cursor.atlasIndex]=s}}])})($i.EventEmitter);E(it,"ATLAS_UPDATED_EVENT","atlasUpdated");var Wu=Ne.fontSize;function Uu(){var a=`  // -------------------------------------------------------------------------
  // Step 3: Calculate position offset from the shared edge distance
  // -------------------------------------------------------------------------
  vec2 positionOffset = vec2(0.0);

  if (a_positionMode < 4.0) {
    vec2 screenDir = getLabelDirection(a_positionMode);
    float boundaryDistPixels = nodeRadiusPixels * edgeDist;
    positionOffset = screenDir * (boundaryDistPixels + margin);
  }`,n=`#version 300 es

// ============================================================================
// Attributes
// ============================================================================

// Per-character (instanced)
in float a_nodeIndex;        // Index into node data texture
in vec2 a_charOffset;        // Character offset from label origin (pixels)
in vec2 a_charSize;          // Character dimensions (pixels)
in vec4 a_texCoords;         // Atlas coords: (x, y, width, height) in pixels
in vec4 a_color;             // Text color (RGBA)
in float a_margin;           // Gap between node edge and label (pixels)
in float a_positionMode;     // Position: 0=right, 1=left, 2=above, 3=below, 4=over
in float a_labelWidth;       // Total label width (pixels)
in float a_labelHeight;      // Label height (pixels)
in float a_verticalCenter;   // Vertical center offset from baseline (pixels)
in float a_textHeight;       // Actual text height: maxAscent + maxDescent (pixels)
in float a_labelAngle;       // Label rotation angle (radians)

// Per-vertex (constant quad corners)
in vec2 a_quadCorner;        // Quad corner: [-1,-1], [1,-1], [-1,1], [1,1]

// ============================================================================
// Uniforms
// ============================================================================

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
uniform vec2 u_resolution;
uniform vec2 u_atlasSize;
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_nodeFrameTexture;
uniform int u_nodeFrameTextureWidth;
uniform float u_zoomLabelSizeRatio;
uniform float u_labelPixelSnapping;
uniform float u_pixelRatio;

// ============================================================================
// Varyings
// ============================================================================

out vec2 v_texCoord;
out vec4 v_color;
out float v_fontScale;

// ============================================================================
// Constants
// ============================================================================

const float bias = 255.0 / 254.0;
const float ATLAS_FONT_SIZE = `.concat(me(Wu),`;

// ============================================================================
// Helper Functions
// ============================================================================

`).concat(He,`
`).concat(ot,`
`).concat(Pt,`
`).concat(Yi,`

// ============================================================================
// Main
// ============================================================================

void main() {
  // -------------------------------------------------------------------------
  // Step 0: Fetch node data + shared edge distance from textures
  // -------------------------------------------------------------------------
  // Node-data texture format: vec4(x, y, size, shapeId)
  // 2D texture layout: texCoord = (index % width, index / width)
  int nodeIdx = int(a_nodeIndex);
  vec4 nodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  vec2 a_anchorPosition = nodeData.xy;
  float a_nodeSize = nodeData.z;

  // Normalized edge distance from the shared frame texture (the frame-pass ran
  // the SDF search once; the label just reads the result).
  float edgeDist = readFrameTexel(u_nodeFrameTexture, u_nodeFrameTextureWidth, nodeIdx).r;

  // Per-node label rotation alignment: 0 = viewport, 1 = label turns with camera.
  float labelRotation = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx).g;

  // Apply zoom-dependent label size scaling
  // Positional values are in CSS pixels; multiply by u_pixelRatio to convert to
  // physical pixels, which is what the NDC conversion (/ u_resolution) expects.
  float zoomScale = u_zoomLabelSizeRatio;
  float margin = a_margin * zoomScale * u_pixelRatio;
  vec2 charOffset = a_charOffset * zoomScale * u_pixelRatio;
  vec2 charSize = a_charSize * zoomScale * u_pixelRatio;
  float labelWidth = a_labelWidth * zoomScale * u_pixelRatio;
  float labelHeight = a_labelHeight * zoomScale;

  // Font scale: ratio of CSS label size to the base atlas font size.
  // Divided by u_pixelRatio because the atlas is generated at ATLAS_FONT_SIZE * pixelRatio,
  // which cancels out the u_pixelRatio in the fragment shader's gamma formula and keeps
  // the anti-aliasing band width consistent across pixel densities.
  v_fontScale = a_labelHeight * zoomScale / (ATLAS_FONT_SIZE * u_pixelRatio);

  // -------------------------------------------------------------------------
  // Step 1: Transform node position to clip space
  // -------------------------------------------------------------------------
  vec3 anchorClip = u_matrix * vec3(a_anchorPosition, 1.0);

  // -------------------------------------------------------------------------
  // Step 2: Convert node size to screen pixels
  // -------------------------------------------------------------------------
  float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
  float nodeRadiusGraphSpace = a_nodeSize * u_correctionRatio / u_sizeRatio * 2.0;
  float nodeRadiusNDC = nodeRadiusGraphSpace * matrixScaleX;
  float nodeRadiusPixels = nodeRadiusNDC * u_resolution.x / 2.0;

`).concat(a,`

  // -------------------------------------------------------------------------
  // Step 4: Calculate final vertex position
  // -------------------------------------------------------------------------
  vec2 cornerOffset = (a_quadCorner + 1.0) * 0.5;
  vec2 charPixelPos = positionOffset + charOffset + cornerOffset * charSize;

  // Apply text alignment based on position mode
  float verticalCenter = a_verticalCenter * zoomScale * u_pixelRatio;
  float textHeight = a_textHeight * zoomScale * u_pixelRatio;
  float baselineToDescent = textHeight / 2.0 - verticalCenter;
  float baselineToAscent = textHeight / 2.0 + verticalCenter;

  if (a_positionMode < 0.5) {
    // Right: vertically center
    charPixelPos.y += verticalCenter;
  } else if (a_positionMode < 1.5) {
    // Left: right-align and vertically center
    charPixelPos.x -= labelWidth;
    charPixelPos.y += verticalCenter;
  } else if (a_positionMode < 2.5) {
    // Above: center horizontally, bottom of text at anchor
    charPixelPos.x -= labelWidth * 0.5;
    charPixelPos.y -= baselineToDescent;
  } else if (a_positionMode < 3.5) {
    // Below: center horizontally, top of text at anchor
    charPixelPos.x -= labelWidth * 0.5;
    charPixelPos.y += baselineToAscent;
  } else {
    // Over: center both
    charPixelPos.x -= labelWidth * 0.5;
    charPixelPos.y += verticalCenter;
  }

  // Apply label angle rotation. Graph-aligned labels add the camera angle so the
  // whole label orbits the node in lockstep with the frame-pass edge distance.
  float labelAngle = a_labelAngle - labelRotation * u_cameraAngle;
  float la_c = cos(labelAngle);
  float la_s = sin(labelAngle);
  mat2 labelRotMat = mat2(la_c, -la_s, la_s, la_c);
  charPixelPos = labelRotMat * charPixelPos;

  // Snap node center to pixel grid so label/backdrop/attachment move as a unit
  vec2 nodeScreen = vec2(
    (anchorClip.x + 1.0) * u_resolution.x,
    (1.0 - anchorClip.y) * u_resolution.y
  ) * 0.5;
  charPixelPos += (round(nodeScreen) - nodeScreen) * u_labelPixelSnapping;

  // Convert to NDC (flip Y: screen Y-down -> clip Y-up)
  vec2 ndcOffset = vec2(charPixelPos.x, -charPixelPos.y) * 2.0 / u_resolution;
  gl_Position = vec4(anchorClip.xy + ndcOffset, 0.0, 1.0);

  // -------------------------------------------------------------------------
  // Step 5: Texture coordinates
  // -------------------------------------------------------------------------
  v_texCoord = (a_texCoords.xy + cornerOffset * a_texCoords.zw) / u_atlasSize;

  // -------------------------------------------------------------------------
  // Step 6: Pass color
  // -------------------------------------------------------------------------
  v_color = a_color;
  v_color.a *= bias;
}
`);return n}function Ou(){var a=`#version 300 es
precision highp float;

in vec2 v_texCoord;
in vec4 v_color;
in float v_fontScale;

uniform sampler2D u_atlas;
uniform float u_gamma;
uniform float u_sdfBuffer;
uniform float u_pixelRatio;

// Fragment output (single target - picking handled via separate pass)
out vec4 fragColor;

void main() {
  #ifdef PICKING_MODE
    // Labels are not pickable - discard all fragments in picking mode
    fragColor = vec4(0.0);
    discard;
  #else
    // Sample SDF value from atlas (high = inside glyph, low = outside)
    float sdfValue = texture(u_atlas, v_texCoord).a;

    // Edge threshold: 1.0 - cutoff = 0.75 for default cutoff=0.25
    // This is where the glyph edge is located in the SDF
    float edgeThreshold = 1.0 - u_sdfBuffer;

    // Gamma controls the anti-aliasing band width.
    // Scale inversely with font scale so small labels get a wider AA band
    // (smoother) and large labels get a tighter band (sharper).
    float gamma = u_gamma / (u_pixelRatio * v_fontScale);

    // Pure gamma-based anti-aliasing using smoothstep
    // The AA band extends from (threshold - gamma) to (threshold + gamma)
    float alpha = smoothstep(edgeThreshold - gamma, edgeThreshold + gamma, sdfValue);

    // Premultiplied alpha output for correct blending
    float finalAlpha = v_color.a * alpha;
    fragColor = vec4(v_color.rgb * finalAlpha, finalAlpha);
  #endif
}
`;return a}function Hu(){return["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_pixelRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_zoomLabelSizeRatio","u_labelPixelSnapping"]}function $u(){return{vertexShader:Uu(),fragmentShader:Ou(),uniforms:Hu()}}function Vu(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:kt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=$u(),h=(function(f){function c(g,b,m){var p,v,y,T;if(ne(this,c),T=fe(this,c,[g,b,m]),E(T,"atlasTexture",null),E(T,"atlasNeedsUpdate",!1),E(T,"labelGlyphCache",new Map),T.atlasFontSize=Ne.fontSize*Yt(),T.atlasManager=new it({fontSize:T.atlasFontSize}),T.gamma=.025,T.sdfBuffer=Ne.cutoff,T.atlasTexture=g.createTexture(),!T.atlasTexture)throw new Error("NodeLabelProgram: failed to create atlas texture");g.bindTexture(g.TEXTURE_2D,T.atlasTexture),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_S,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_T,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MIN_FILTER,g.LINEAR),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MAG_FILTER,g.LINEAR),g.bindTexture(g.TEXTURE_2D,null),T.atlasManager.on(it.ATLAS_UPDATED_EVENT,function(){T.atlasNeedsUpdate=!0});var _={family:((p=s.font)===null||p===void 0?void 0:p.family)||"sans-serif",weight:((v=s.font)===null||v===void 0?void 0:v.weight)||"normal",style:((y=s.font)===null||y===void 0?void 0:y.style)||"normal"};return T.defaultFontKey=T.atlasManager.registerFont(_),T}return ge(c,f),ae(c,[{key:"getDefinition",value:function(){var b=WebGL2RenderingContext,m=b.FLOAT,p=b.UNSIGNED_BYTE,v=b.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d.vertexShader,FRAGMENT_SHADER_SOURCE:d.fragmentShader,METHOD:v,UNIFORMS:d.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:m},{name:"a_charOffset",size:2,type:m},{name:"a_charSize",size:2,type:m},{name:"a_texCoords",size:4,type:m},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_margin",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_labelWidth",size:1,type:m},{name:"a_labelHeight",size:1,type:m},{name:"a_verticalCenter",size:1,type:m},{name:"a_textHeight",size:1,type:m},{name:"a_labelAngle",size:1,type:m}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:m}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(b,m){if(m.hidden||!m.text){this.labelGlyphCache.delete(b);return}var p=m.text,v=m.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(p,v);var y=[],T=[],_=0,S=0,D=0,w=U(p),k;try{for(w.s();!(k=w.n()).done;){var L=k.value,N=L.codePointAt(0);if(N===void 0){y.push(void 0),T.push(_);continue}var R=this.atlasManager.getGlyph(N,v);y.push(R),T.push(_),R&&(_+=R.advance,S=Math.max(S,R.bearingY),D=Math.max(D,R.atlasHeight-R.bearingY))}}catch(z){w.e(z)}finally{w.f()}this.labelGlyphCache.set(b,{glyphs:y,xOffsets:T,totalWidth:_,totalHeight:S+D,verticalCenterOffset:(S-D)/2})}},{key:"processCharacter",value:function(b,m,p,v){var y=this.floats,T=this.STRIDE,_=b*T,S=this.labelGlyphCache.get(m.parentKey);if(!S||!S.glyphs[v]){for(var D=0;D<T;D++)y[_+D]=0;return}var w=S.glyphs[v],k=S.xOffsets[v],L=m.size/this.atlasFontSize,N=ke(m.color),R=_;y[R++]=m.nodeIndex,y[R++]=(k+w.bearingX)*L,y[R++]=-w.bearingY*L,y[R++]=w.atlasWidth*L,y[R++]=w.atlasHeight*L,y[R++]=w.atlasX,y[R++]=w.atlasY,y[R++]=w.atlasWidth,y[R++]=w.atlasHeight,y[R++]=N,y[R++]=m.margin,y[R++]=Ct[m.position],y[R++]=S.totalWidth*L,y[R++]=m.size,y[R++]=S.verticalCenterOffset*L,y[R++]=S.totalHeight*L,y[R++]=m.labelAngle}},{key:"processLabel",value:function(b,m,p){return this.prepareLabelGlyphs(b,p),ye(c,"processLabel",this,3)([b,m,p])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var b=this.normalProgram.gl,m=this.atlasManager.getTextures();if(m.length!==0){var p=m[0];b.bindTexture(b.TEXTURE_2D,this.atlasTexture),b.texImage2D(b.TEXTURE_2D,0,b.RGBA,p.width,p.height,0,b.RGBA,b.UNSIGNED_BYTE,p.data),b.bindTexture(b.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(b,m){var p=m.gl,v=m.uniformLocations;p.uniformMatrix3fv(v.u_matrix,!1,b.matrix),p.uniform1f(v.u_sizeRatio,b.sizeRatio),p.uniform1f(v.u_correctionRatio,b.correctionRatio),p.uniform1f(v.u_cameraAngle,b.cameraAngle),p.uniform2f(v.u_resolution,b.width*b.pixelRatio,b.height*b.pixelRatio);var y=this.atlasManager.getTextures();y.length>0?p.uniform2f(v.u_atlasSize,y[0].width,y[0].height):p.uniform2f(v.u_atlasSize,1,1),p.activeTexture(p.TEXTURE0),p.bindTexture(p.TEXTURE_2D,this.atlasTexture),p.uniform1i(v.u_atlas,0),v.u_nodeDataTexture!==void 0&&p.uniform1i(v.u_nodeDataTexture,b.nodeDataTextureUnit),v.u_nodeDataTextureWidth!==void 0&&p.uniform1i(v.u_nodeDataTextureWidth,b.nodeDataTextureWidth),p.uniform1i(v.u_nodeFrameTexture,b.nodeFrameTextureUnit),p.uniform1i(v.u_nodeFrameTextureWidth,b.nodeFrameTextureWidth),p.uniform1f(v.u_gamma,this.gamma),p.uniform1f(v.u_sdfBuffer,this.sdfBuffer),p.uniform1f(v.u_pixelRatio,b.pixelRatio),p.uniform1f(v.u_zoomLabelSizeRatio,1/c.zoomToLabelSizeRatioFunction(b.zoomRatio)),p.uniform1f(v.u_labelPixelSnapping,b.labelPixelSnapping)}},{key:"renderProgram",value:function(b,m){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),ye(c,"renderProgram",this,3)([b,m])}},{key:"registerFont",value:function(b){var m=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",p=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:b,weight:m,style:p})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"measureLabel",value:function(b,m,p){var v=p||this.defaultFontKey;this.atlasManager.ensureGlyphs(b,v),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var y=0,T=0,_=0,S=U(b),D;try{for(S.s();!(D=S.n()).done;){var w=D.value,k=w.codePointAt(0);if(k!==void 0){var L=this.atlasManager.getGlyph(k,v);L&&(y+=L.advance,T=Math.max(T,L.bearingY),_=Math.max(_,L.atlasHeight-L.bearingY))}}}catch(R){S.e(R)}finally{S.f()}var N=m/this.atlasFontSize;return{width:y*N,height:m,textHeight:(T+_)*N}}},{key:"ensureGlyphsReady",value:function(b,m){var p=m||this.defaultFontKey,v=U(b),y;try{for(v.s();!(y=v.n()).done;){var T=y.value;this.atlasManager.ensureGlyphs(T,p)}}catch(_){v.e(_)}finally{v.f()}this.atlasManager.flush()}},{key:"kill",value:function(){var b=this.normalProgram.gl;this.atlasTexture&&(b.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),ye(c,"kill",this,3)([])}}])})(to);return E(h,"labelMargin",l),E(h,"zoomToLabelSizeRatioFunction",u),new h(a,n,t)}var ju=`#version 300 es
precision highp float;

in float v_edgeDist;

// R32F target: only the .r channel is stored.
out vec4 fragColor;

void main() {
  fragColor = vec4(v_edgeDist, 0.0, 0.0, 0.0);
}
`;function Xu(a,n,t){var e=Ua(a),r=nn(a).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),i=mu(a,t),o=i.code,s=i.multiShape,l=Ft(a,n),u=Object.keys(l.specs).map(function(g){return"float v_".concat(g,";")}).join(`
`),d=Lt(l,{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),h=d.fetchCode,f=d.varyingAssignments,c=`#version 300 es
precision highp float;

uniform float u_cameraAngle;
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform float u_frameTextureWidth;
uniform float u_frameTextureHeight;
uniform sampler2D u_layerAttributeTexture;
uniform int u_layerAttributeTextureWidth;
uniform int u_layerAttributeTexelsPerNode;
`.concat(r,`

in float a_nodeIndex;     // node-data texture index (also the target frame texel)
in float a_positionMode;  // 0=right 1=left 2=above 3=below 4=over
in float a_labelAngle;    // intrinsic label angle (radians)

out float v_edgeDist;

// Shape attributes (plain globals: this pass is vertex-only)
`).concat(u,`

`).concat(He,`
`).concat(ot,`
`).concat(e,`
`).concat(o,`
`).concat(Yi,`

void main() {
  int nodeIdx = int(a_nodeIndex);
`).concat(h,`
`).concat(f,`
  `).concat(s?`// Multi-shape: the shape id lives in the node-data texture's .w channel:
  g_shapeId = int(readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx).w);`:"// Single-shape mode, shape id not needed",`

  // Per-node rotation alignment (0 = viewport, 1 = graph).
  vec4 nodeFlags = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, nodeIdx);
  float nodeRotation = nodeFlags.r;
  float labelRotation = nodeFlags.g;

  // Effective angle: intrinsic (style-given) plus the camera angle when the label
  // is graph-aligned. The companions place the box at this same angle.
  float effectiveAngle = a_labelAngle - labelRotation * u_cameraAngle;

  // The "over" mode (4) sits on the node center, so it has no edge distance.
  float edgeDist = 0.0;
  if (a_positionMode < 4.0) {
    vec2 screenDir = getLabelDirection(a_positionMode);
    // Rotate by the effective label angle. Inlined rather than via rotate2D()
    // because the shape SDFs (getShapeGLSLForShapes) already define that helper
    // for multi-shape programs, and redefining it would be a GLSL error.
    float ea_c = cos(effectiveAngle);
    float ea_s = sin(effectiveAngle);
    vec2 rotatedScreenDir = mat2(ea_c, -ea_s, ea_s, ea_c) * screenDir;
    // Screen (Y-down) -> SDF (Y-up).
    vec2 sdfDir = vec2(rotatedScreenDir.x, -rotatedScreenDir.y);
    // Counter-rotate into the shape's local frame for graph-aligned nodes, so the
    // boundary is queried against the shape's actual on-screen orientation.
    float nodeCa = -u_cameraAngle * nodeRotation;
    float nc = cos(nodeCa), ns = sin(nodeCa);
    sdfDir = mat2(nc, -ns, ns, nc) * sdfDir;
    edgeDist = findEdgeDistance(sdfDir, 1.0);
  }
  v_edgeDist = edgeDist;

  // Scatter to this node's texel center in the frame texture.
  float x = mod(a_nodeIndex, u_frameTextureWidth);
  float y = floor(a_nodeIndex / u_frameTextureWidth);
  vec2 ndc = (vec2(x, y) + 0.5) / vec2(u_frameTextureWidth, u_frameTextureHeight) * 2.0 - 1.0;
  gl_Position = vec4(ndc, 0.0, 1.0);
  gl_PointSize = 1.0;
}
`);return c}var no=(function(){function a(n,t){ne(this,a),E(this,"uniformLocations",{});var e=t.shapes,r=t.layers,i=t.shapeGlobalIds;if(e.length===0)throw new Error("NodeLabelFramePass: at least one shape must be provided");this.gl=n,this.shapeUniforms=nn(e),this.hasAttributeData=Object.keys(Ft(e,r).offsets).length>0,this.vertexShader=Ma(n,Xu(e,r,i)),this.fragmentShader=Ba(n,ju),this.program=Wa(n,[this.vertexShader,this.fragmentShader]);var o=["u_cameraAngle","u_nodeDataTexture","u_nodeDataTextureWidth","u_frameTextureWidth","u_frameTextureHeight","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"].concat(J(this.shapeUniforms.map(function(p){return p.name}))),s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;this.uniformLocations[u]=n.getUniformLocation(this.program,u)}}catch(p){s.e(p)}finally{s.f()}var d=a.FLOATS_PER_POINT*4;this.vao=n.createVertexArray(),this.buffer=n.createBuffer(),n.bindVertexArray(this.vao),n.bindBuffer(n.ARRAY_BUFFER,this.buffer);var h=U([["a_nodeIndex",0],["a_positionMode",4],["a_labelAngle",8]]),f;try{for(h.s();!(f=h.n()).done;){var c=oe(f.value,2),g=c[0],b=c[1],m=n.getAttribLocation(this.program,g);m>=0&&(n.enableVertexAttribArray(m),n.vertexAttribPointer(m,1,n.FLOAT,!1,d,b))}}catch(p){h.e(p)}finally{h.f()}n.bindVertexArray(null)}return ae(a,[{key:"run",value:function(t,e,r,i,o){if(e!==0){var s=this.gl;s.useProgram(this.program),s.bindVertexArray(this.vao),s.bindBuffer(s.ARRAY_BUFFER,this.buffer),s.bufferData(s.ARRAY_BUFFER,t.subarray(0,e*a.FLOATS_PER_POINT),s.DYNAMIC_DRAW);var l=this.uniformLocations;s.uniform1f(l.u_cameraAngle,i.cameraAngle),s.uniform1i(l.u_nodeDataTexture,i.nodeDataTextureUnit),s.uniform1i(l.u_nodeDataTextureWidth,i.nodeDataTextureWidth),s.uniform1f(l.u_frameTextureWidth,r.getTextureWidth()),s.uniform1f(l.u_frameTextureHeight,r.getTextureHeight()),this.hasAttributeData&&o&&(o.bind(Ke),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,Ke),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=U(this.shapeUniforms),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;$a(s,this.uniformLocations[h.name],h)}}catch(f){u.e(f)}finally{u.f()}r.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,e),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var t=this.gl;t.deleteProgram(this.program),t.deleteShader(this.vertexShader),t.deleteShader(this.fragmentShader),t.deleteBuffer(this.buffer),t.deleteVertexArray(this.vao)}}])})();E(no,"FLOATS_PER_POINT",3);function ao(a,n,t,e,r){var i,o=e.label,s=o===void 0?{}:o,l=e.shapes;if(l.length===0)throw new Error("createNodeProgram: at least one shape must be provided in 'shapes'");var u={},d=[],h;l.forEach(function(D,w){var k=su(D);w===0&&(h=k),u[D.name]=w,d[w]=tn(k)});var f=J(e.layers),c=zi({shapes:l,layers:f,shapeGlobalIds:l.length>1?d:void 0,antialias:r}),g=Vu(a,null,t,{label:s}),b=Iu(a,null,t,{shapes:l,layers:f,getAttributeTexture:function(){return S.getAttributeTexture()},label:s,shapeGlobalIds:l.length>1?d:void 0}),m=zu(a,n,t,{label:s}),p=e.labelAttachments&&Object.keys(e.labelAttachments).length>0?Au(a,null,t,{label:s}):null,v=new no(a,{shapes:l,layers:f,shapeGlobalIds:l.length>1?d:void 0}),y=ea(l,f),T=Fe(y),_=(i=(function(D){function w(k,L,N){var R;ne(this,w),R=fe(this,w,[k,L,N]),E(R,"layerLifecycles",new Map),E(R,"layersNeedingRegeneration",new Set),E(R,"attrDescriptors",[]),R._pickingBuffer=L;var z=_.layerTextures.get(k);z||(z=new Zn(k,T),_.layerTextures.set(k,z)),R.layerAttributeTexture=z;var C=_.textureRefCounts.get(k)||0;_.textureRefCounts.set(k,C+1),R.packedAttributeData=new Float32Array(T.floatsPerItem),f.forEach(function(B,F){if(B.lifecycle){var x={gl:k,renderer:{refresh:function(){return N.refresh()}},getUniformLocation:function(G){return k.getUniformLocation(R.normalProgram.program,G)},requestShaderRegeneration:function(){R.layersNeedingRegeneration.add(F)},requestRefresh:function(){N.refresh()}},P=B.lifecycle(x);R.layerLifecycles.set(F,P)}}),R.layerLifecycles.forEach(function(B){var F;(F=B.init)===null||F===void 0||F.call(B)});var I=new Map;return R.layerLifecycles.forEach(function(B,F){B.getAttributeData&&I.set(F,B)}),R.attrDescriptors=Qn(y,T,I),R}return ge(w,D),ae(w,[{key:"getDefinition",value:function(){var L=WebGL2RenderingContext,N=L.FLOAT,R=L.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:c.vertexShader,FRAGMENT_SHADER_SOURCE:c.fragmentShader,METHOD:R,UNIFORMS:c.uniforms,ATTRIBUTES:c.attributes,CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:N}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"maybeRegenerateShaders",value:function(){var L=this;if(this.layersNeedingRegeneration.size!==0){f=f.map(function(F,x){if(L.layersNeedingRegeneration.has(x)){var P=L.layerLifecycles.get(x);if(P!=null&&P.regenerate){var W=P.regenerate();return H(H({},W),{},{lifecycle:F.lifecycle})}}return F}),this.layersNeedingRegeneration.clear(),c=zi({shapes:l,layers:f,shapeGlobalIds:l.length>1?d:void 0,antialias:this.renderer.getSetting("antialiasNodes")});var N=this.normalProgram.gl,R=this.normalProgram,z=R.program,C=R.buffer,I=R.vertexShader,B=R.fragmentShader;N.deleteProgram(z),N.deleteBuffer(C),N.deleteShader(I),N.deleteShader(B),this.normalProgram=this.getProgramInfo("normal",N,c.vertexShader,c.fragmentShader,this._pickingBuffer)}}},{key:"allocateNode",value:function(L){this.layerAttributeTexture.allocate(L)}},{key:"freeNode",value:function(L){this.layerAttributeTexture.free(L)}},{key:"getAttributeTexture",value:function(){return this.layerAttributeTexture}},{key:"uploadLayerTexture",value:function(){this.layerAttributeTexture.upload()}},{key:"process",value:function(L,N,R,z,C){var I=N*this.STRIDE;if(R.visibility==="hidden"){for(var B=I+this.STRIDE;I<B;I++)this.floats[I]=0;this.floats[N*this.STRIDE]=Qi;return}this.processVisibleItem(St(L),I,R,z,C)}},{key:"processVisibleItem",value:function(L,N,R,z,C){var I,B=this.floats,F=this.ints;if(B[N++]=z,F[N++]=L,B[N++]=(I=R.opacity)!==null&&I!==void 0?I:1,T.floatsPerItem!==0){var x=this.packedAttributeData;Jn(this.attrDescriptors,R,x,R.color,this.layerLifecycles,0),this.layerAttributeTexture.updateAllAttributes(C,x)}}},{key:"setUniforms",value:function(L,N){var R=this,z=N.gl,C=N.uniformLocations;C.u_matrix&&z.uniformMatrix3fv(C.u_matrix,!1,L.matrix),C.u_sizeRatio&&z.uniform1f(C.u_sizeRatio,L.sizeRatio),C.u_correctionRatio&&z.uniform1f(C.u_correctionRatio,L.correctionRatio),C.u_pickingPadding&&z.uniform1f(C.u_pickingPadding,L.nodePickingPadding),C.u_cameraAngle&&z.uniform1f(C.u_cameraAngle,L.cameraAngle),C.u_nodeDataTexture&&z.uniform1i(C.u_nodeDataTexture,L.nodeDataTextureUnit),C.u_nodeDataTextureWidth&&z.uniform1i(C.u_nodeDataTextureWidth,L.nodeDataTextureWidth),C.u_layerAttributeTexture&&(this.layerAttributeTexture.bind(Ke),z.uniform1i(C.u_layerAttributeTexture,Ke)),C.u_layerAttributeTextureWidth&&z.uniform1i(C.u_layerAttributeTextureWidth,this.layerAttributeTexture.getTextureWidth()),C.u_layerAttributeTexelsPerNode&&z.uniform1i(C.u_layerAttributeTexelsPerNode,this.layerAttributeTexture.getTexelsPerItem()),l.forEach(function(I){I.uniforms.forEach(function(B){R.setTypedUniform(B,N)})}),f.forEach(function(I){I.uniforms.forEach(function(B){R.setTypedUniform(B,N)})})}},{key:"render",value:function(L,N,R){this.maybeRegenerateShaders(),ye(w,"render",this,3)([L,N,R])}},{key:"renderProgram",value:function(L,N){var R=N.gl,z=N.program;R.useProgram(z),N===this.normalProgram&&this.layerLifecycles.forEach(function(C){var I;(I=C.beforeRender)===null||I===void 0||I.call(C)}),ye(w,"renderProgram",this,3)([L,N])}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(R){var z;(z=R.kill)===null||z===void 0||z.call(R)}),this.layerLifecycles.clear();var L=this.normalProgram.gl,N=(_.textureRefCounts.get(L)||1)-1;N<=0?(this.layerAttributeTexture.kill(),_.layerTextures.delete(L),_.textureRefCounts.delete(L)):_.textureRefCounts.set(L,N),ye(w,"kill",this,3)([])}}])})(st),E(i,"layerTextures",new WeakMap),E(i,"textureRefCounts",new WeakMap),i),S=new _(a,null,t);return{nodeProgram:S,labelProgram:g,backdropProgram:b,labelBackgroundProgram:m,attachmentProgram:p,framePass:v,shapeSlug:h,shapeNameToIndex:l.length>1?u:void 0,shapeGlobalIds:l.length>1?d:void 0}}var Ye=6;function rn(a){var n=a.offsets,t=a.specs,e=a.floatsPerItem,r=Object.keys(n);if(r.length===0||e===0)return{uniformDeclarations:"",uniformNames:[],vertexVaryingDeclarations:"",fragmentVaryingDeclarations:"",fetchCode:"",varyingAssignments:""};var i=`
uniform sampler2D u_edgeAttributeTexture;
uniform int u_edgeAttributeTextureWidth;
uniform int u_edgeAttributeTexelsPerEdge;`,o=["u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"],s=r.map(function(c){var g=t[c].size===1?"float":"vec".concat(t[c].size);return"".concat(g," v_").concat(c,";")}),l=s.map(function(c){return"out ".concat(c)}).join(`
`),u=s.map(function(c){return"in ".concat(c)}).join(`
`),d=Lt(a,{varPrefix:"attr",baseTexelExpr:"edgeIdx * u_edgeAttributeTexelsPerEdge",textureWidthUniform:"u_edgeAttributeTextureWidth",textureSamplerUniform:"u_edgeAttributeTexture"}),h=d.fetchCode,f=d.varyingAssignments;return{uniformDeclarations:i,uniformNames:o,vertexVaryingDeclarations:l,fragmentVaryingDeclarations:u,fetchCode:h,varyingAssignments:f}}function qu(a){return`
float findSourceClampT_`.concat(a,`(vec2 source, float sourceSize, int sourceShapeId, float sourceRotateAlign, vec2 target, float margin) {
  float lo = 0.0, hi = 0.5;
  float nodeExtent = sourceSize * u_correctionRatio / u_sizeRatio * 2.0;
  float effectiveSize = 1.0 - u_correctionRatio / nodeExtent;

  // Counter-rotate the query point so viewport-aligned nodes (rotateAlign=0) are
  // clamped against their on-screen orientation; graph-aligned nodes skip it.
  float ca = u_cameraAngle * (1.0 - sourceRotateAlign);
  float rc = cos(ca), rs = sin(ca);
  mat2 rot = mat2(rc, -rs, rs, rc);

  for (int i = 0; i < 12; i++) {
    float mid = (lo + hi) * 0.5;
    vec2 pos = path_`).concat(a,`_position(mid, source, target);
    vec2 localPos = rot * ((pos - source) / nodeExtent);
    float sdf = querySDF(sourceShapeId, localPos, effectiveSize);
    if (sdf < 0.0) lo = mid;
    else hi = mid;
  }

  float pathLen = path_`).concat(a,`_length(source, target);
  float marginT = (margin * u_correctionRatio / u_sizeRatio) / pathLen;
  return (lo + hi) * 0.5 + marginT;
}
`)}function Ku(a){return`
float findTargetClampT_`.concat(a,`(vec2 source, vec2 target, float targetSize, int targetShapeId, float targetRotateAlign, float margin) {
  float lo = 0.5, hi = 1.0;
  float nodeExtent = targetSize * u_correctionRatio / u_sizeRatio * 2.0;
  float effectiveSize = 1.0 - u_correctionRatio / nodeExtent;

  // See findSourceClampT_ for the rotation rationale.
  float ca = u_cameraAngle * (1.0 - targetRotateAlign);
  float rc = cos(ca), rs = sin(ca);
  mat2 rot = mat2(rc, -rs, rs, rc);

  for (int i = 0; i < 12; i++) {
    float mid = (lo + hi) * 0.5;
    vec2 pos = path_`).concat(a,`_position(mid, source, target);
    vec2 localPos = rot * ((pos - target) / nodeExtent);
    float sdf = querySDF(targetShapeId, localPos, effectiveSize);
    if (sdf < 0.0) hi = mid;
    else lo = mid;
  }

  float pathLen = path_`).concat(a,`_length(source, target);
  float marginT = (margin * u_correctionRatio / u_sizeRatio) / pathLen;
  return (lo + hi) * 0.5 - marginT;
}
`)}function ro(a){return`
// Auto-generated numerical tangent (from position via finite differences)
vec2 path_`.concat(a,`_tangent(float t, vec2 source, vec2 target) {
  float epsilon = 0.001;
  float t1 = max(0.0, t - epsilon);
  float t2 = min(1.0, t + epsilon);
  vec2 p1 = path_`).concat(a,`_position(t1, source, target);
  vec2 p2 = path_`).concat(a,`_position(t2, source, target);
  return normalize(p2 - p1);
}

// Auto-generated normal (perpendicular to tangent)
vec2 path_`).concat(a,`_normal(float t, vec2 source, vec2 target) {
  vec2 tangent = path_`).concat(a,`_tangent(t, source, target);
  return vec2(-tangent.y, tangent.x);
}
`)}function Yu(a){return`
// Auto-generated path length (samples position 16 times)
float path_`.concat(a,`_length(vec2 source, vec2 target) {
  float len = 0.0;
  vec2 prev = path_`).concat(a,`_position(0.0, source, target);
  for (int i = 1; i <= 16; i++) {
    float t = float(i) / 16.0;
    vec2 curr = path_`).concat(a,`_position(t, source, target);
    len += length(curr - prev);
    prev = curr;
  }
  return len;
}
`)}function Zu(a){return`
// Auto-generated closest_t (coarse sample + ternary search)
float path_`.concat(a,`_closest_t(vec2 p, vec2 source, vec2 target) {
  // Coarse search: find best among 10 samples
  float bestT = 0.0;
  float bestDist = 1e10;
  for (int i = 0; i <= 10; i++) {
    float t = float(i) / 10.0;
    vec2 pos = path_`).concat(a,`_position(t, source, target);
    float d = length(p - pos);
    if (d < bestDist) {
      bestDist = d;
      bestT = t;
    }
  }

  // Refine with ternary search
  float lo = max(0.0, bestT - 0.1);
  float hi = min(1.0, bestT + 0.1);
  for (int i = 0; i < 10; i++) {
    float mid1 = lo + (hi - lo) / 3.0;
    float mid2 = hi - (hi - lo) / 3.0;
    float d1 = length(p - path_`).concat(a,`_position(mid1, source, target));
    float d2 = length(p - path_`).concat(a,`_position(mid2, source, target));
    if (d1 < d2) {
      hi = mid2;
    } else {
      lo = mid1;
    }
  }
  return (lo + hi) * 0.5;
}
`)}function Qu(a){return`
// Auto-generated signed distance (via closest_t + normal)
float path_`.concat(a,`_distance(vec2 p, vec2 source, vec2 target) {
  float closestT = path_`).concat(a,`_closest_t(p, source, target);
  vec2 closest = path_`).concat(a,`_position(closestT, source, target);
  vec2 diff = p - closest;
  float dist = length(diff);
  if (dist < 0.0001) return 0.0;

  // Get normal at closest point
  vec2 normal = path_`).concat(a,`_normal(closestT, source, target);
  return dist * sign(dot(diff, normal));
}
`)}function Ju(a){return`
// Auto-generated t_at_distance (binary search)
float path_`.concat(a,`_t_at_distance(float targetDist, vec2 source, vec2 target) {
  if (targetDist <= 0.0) return 0.0;

  float totalLen = path_`).concat(a,`_length(source, target);
  if (targetDist >= totalLen) return 1.0;

  // Binary search for t
  float lo = 0.0, hi = 1.0;
  for (int i = 0; i < 12; i++) {
    float mid = (lo + hi) * 0.5;

    // Compute arc length from 0 to mid
    float arcLen = 0.0;
    vec2 prev = path_`).concat(a,`_position(0.0, source, target);
    for (int j = 1; j <= 8; j++) {
      float t = mid * float(j) / 8.0;
      vec2 curr = path_`).concat(a,`_position(t, source, target);
      arcLen += length(curr - prev);
      prev = curr;
    }

    if (arcLen < targetDist) {
      lo = mid;
    } else {
      hi = mid;
    }
  }
  return (lo + hi) * 0.5;
}
`)}function Vn(a,n){var t=new RegExp("\\b(float|vec[234]|void|int|bool)\\s+".concat(n,"\\s*\\("));return t.test(a)}function io(a,n){var t=[];return Vn(n,"path_".concat(a,"_length"))||t.push(Yu(a)),Vn(n,"path_".concat(a,"_closest_t"))||t.push(Zu(a)),Vn(n,"path_".concat(a,"_distance"))||t.push(Qu(a)),Vn(n,"path_".concat(a,"_t_at_distance"))||t.push(Ju(a)),t.length>0?`
// ============================================================================
// Auto-generated fallback functions (path only provided position)
// ============================================================================
`.concat(t.join(`
`)):""}function oo(a){var n,t,e,r=a.paths,i=a.layers,o=(n=a.extremities)!==null&&n!==void 0?n:[];if(r.length===0)throw new Error("At least one path is required in 'paths'");if(i.length===0)throw new Error("At least one layer is required in 'layers'");var s=(t=a.defaultHead)!==null&&t!==void 0?t:"none",l=(e=a.defaultTail)!==null&&e!==void 0?e:"none";return{paths:r,extremities:o,layers:i,path:r[0],layer:i[0],defaultHead:s,defaultTail:l}}var so=WebGL2RenderingContext,en=so.FLOAT,Wi=so.UNSIGNED_BYTE;function ed(a,n,t){var e=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"]),r=new Set(e);return a.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),n.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),t.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),Array.from(r)}function Va(a){var n=new Set,t=[],e=U(a),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;n.has(i.name)||(n.add(i.name),t.push("// Path: ".concat(i.name)),t.push(i.glsl),t.push(ro(i.name)),t.push(io(i.name,i.glsl)))}}catch(o){e.e(o)}finally{e.f()}return t.join(`

`)}function td(a){var n=[],t=U(a),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.push("// Extremity: ".concat(r.name)),n.push(r.glsl)}}catch(i){t.e(i)}finally{t.f()}return n.join(`

`)}var nd=[{queryName:"queryPathPosition",pathFunc:"position",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathTangent",pathFunc:"tangent",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathNormal",pathFunc:"normal",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathLength",pathFunc:"length",returnType:"float",params:"vec2 source, vec2 target",args:"source, target"},{queryName:"queryPathClosestT",pathFunc:"closest_t",returnType:"float",params:"vec2 p, vec2 source, vec2 target",args:"p, source, target"}];function ad(a,n){var t=n.queryName,e=n.pathFunc,r=n.returnType,i=n.params,o=n.args;if(a.length===1)return"".concat(r," ").concat(t,"(int pathId, ").concat(i,`) {
  return path_`).concat(a[0].name,"_").concat(e,"(").concat(o,`);
}`);var s=a.map(function(l,u){return"    case ".concat(u,": return path_").concat(l.name,"_").concat(e,"(").concat(o,");")}).join(`
`);return"".concat(r," ").concat(t,"(int pathId, ").concat(i,`) {
  switch (pathId) {
`).concat(s,`
    default: return path_`).concat(a[0].name,"_").concat(e,"(").concat(o,`);
  }
}`)}function ja(a){return nd.map(function(n){return ad(a,n)}).join(`

`)}function rd(a){if(a.length===1)return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  return extremity_`.concat(a[0].name,`(uv, lengthRatio, widthRatio);
}`);var n=a.map(function(t,e){return"    case ".concat(e,": return extremity_").concat(t.name,"(uv, lengthRatio, widthRatio);")}).join(`
`);return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  switch (extremityId) {
`.concat(n,`
    default: return extremity_`).concat(a[0].name,`(uv, lengthRatio, widthRatio);
  }
}`)}function id(a){var n=[],t=U(a),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.push(qu(r.name)),n.push(Ku(r.name))}}catch(s){t.e(s)}finally{t.f()}if(a.length===1)n.push(`
float queryFindSourceClampT(int pathId, vec2 source, float sourceSize, int sourceShapeId, float sourceRotateAlign, vec2 target, float margin) {
  return findSourceClampT_`.concat(a[0].name,`(source, sourceSize, sourceShapeId, sourceRotateAlign, target, margin);
}

float queryFindTargetClampT(int pathId, vec2 source, vec2 target, float targetSize, int targetShapeId, float targetRotateAlign, float margin) {
  return findTargetClampT_`).concat(a[0].name,`(source, target, targetSize, targetShapeId, targetRotateAlign, margin);
}`));else{var i=a.map(function(s,l){return"    case ".concat(l,": return findSourceClampT_").concat(s.name,"(source, sourceSize, sourceShapeId, sourceRotateAlign, target, margin);")}).join(`
`),o=a.map(function(s,l){return"    case ".concat(l,": return findTargetClampT_").concat(s.name,"(source, target, targetSize, targetShapeId, targetRotateAlign, margin);")}).join(`
`);n.push(`
float queryFindSourceClampT(int pathId, vec2 source, float sourceSize, int sourceShapeId, float sourceRotateAlign, vec2 target, float margin) {
  switch (pathId) {
`.concat(i,`
    default: return findSourceClampT_`).concat(a[0].name,`(source, sourceSize, sourceShapeId, sourceRotateAlign, target, margin);
  }
}

float queryFindTargetClampT(int pathId, vec2 source, vec2 target, float targetSize, int targetShapeId, float targetRotateAlign, float margin) {
  switch (pathId) {
`).concat(o,`
    default: return findTargetClampT_`).concat(a[0].name,`(source, target, targetSize, targetShapeId, targetRotateAlign, margin);
  }
}`))}return n.join(`

`)}function od(a,n,t,e,r){var i=Fe([].concat(J(a),J(t))),o=rn(i),s=Ft(e,r),l=Object.keys(s.offsets),u=function(D){var w=s.specs[D].size;return w===1?"float":"vec".concat(w)},d=l.map(function(S){return"".concat(u(S)," v_source_").concat(S,`;
`).concat(u(S)," v_target_").concat(S,";")}).join(`
`),h=Lt(s,{varPrefix:"srcNodeAttr",baseTexelExpr:"srcIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_source_"}),f=Lt(s,{varPrefix:"tgtNodeAttr",baseTexelExpr:"tgtIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_target_"}),c=l.map(function(S){return"".concat(u(S)," g_").concat(S,";")}).join(`
`),g=l.map(function(S){return"  g_".concat(S," = v_source_").concat(S,";")}).join(`
`),b=l.map(function(S){return"  g_".concat(S," = v_target_").concat(S,";")}).join(`
`),m=new Set,p=[],v=function(D){m.has(D.name)||(m.add(D.name),p.push("uniform ".concat(D.type," ").concat(D.name,";")))};a.forEach(function(S){return S.uniforms.forEach(v)}),n.forEach(function(S){return S.uniforms.forEach(v)});var y=n.map(function(S){return me(S.widthFactor)}).join(", "),T=Math.max.apply(Math,J(a.map(function(S){return S.minBodyLengthRatio||0}))),_=`#version 300 es

// Node and edge data textures
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_edgeDataTexture;
uniform int u_edgeDataTextureWidth;

// Edge attribute texture (for path-specific attributes like curvature)
`.concat(o.uniformDeclarations,`

// Node attribute texture (for node shape attributes)
uniform sampler2D u_layerAttributeTexture;
uniform int u_layerAttributeTextureWidth;
uniform int u_layerAttributeTexelsPerNode;

// Render params needed for clamping
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
uniform float u_minEdgeThickness;

// Edge-frame texture dimensions, for scattering each point to its texel
uniform float u_frameTextureWidth;
uniform float u_frameTextureHeight;

// Custom path/extremity uniforms
`).concat(p.join(`
`),`

// Path attribute varyings \u2014 assigned so path functions can read them as globals
`).concat(o.vertexVaryingDeclarations,`

// Node size varyings \u2014 read by path functions like loop
out float v_sourceNodeSize;
out float v_targetNodeSize;

// Node shape attributes per endpoint, and the ones querySDF reads
`).concat(d,`
`).concat(c,`

// Scattered output written to the edge-frame texel: [tStart, tEnd, straightenFactor, pathLength]
out vec4 v_clamp;

// Extremity width factor array (needed for extremityScale computation)
const float EXTREMITY_WIDTH_FACTORS[`).concat(n.length,"] = float[](").concat(y,`);

// Node-data fetch helpers (geometry texel + rotation-flags texel)
`).concat(He,`
`).concat(ot,`

// Shape SDFs for node boundary clamping
`).concat(lu(),`
`).concat(uu(new Set(l)),`

// Path functions
`).concat(Va(a),`
`).concat(ja(a),`
`).concat(id(a),`

void main() {
  // One point per edge-data row; the row index is the edge-frame texel target.
  int edgeIdx = gl_VertexID;
  int texel0Idx = edgeIdx * 2;
  int texel1Idx = edgeIdx * 2 + 1;
  ivec2 edgeTexCoord0 = ivec2(texel0Idx % u_edgeDataTextureWidth, texel0Idx / u_edgeDataTextureWidth);
  ivec2 edgeTexCoord1 = ivec2(texel1Idx % u_edgeDataTextureWidth, texel1Idx / u_edgeDataTextureWidth);
  vec4 edgeData0 = texelFetch(u_edgeDataTexture, edgeTexCoord0, 0);
  vec4 edgeData1 = texelFetch(u_edgeDataTexture, edgeTexCoord1, 0);

  int srcIdx = int(edgeData0.x);
  int tgtIdx = int(edgeData0.y);
  float a_thickness = edgeData0.z;
  float a_headLengthRatio = edgeData1.x;
  float a_tailLengthRatio = edgeData1.y;
  int pathId = int(edgeData1.z);
  int extremityPacked = int(edgeData1.w);
  int headId = extremityPacked >> 4;
  int tailId = extremityPacked & 15;

  // Fetch path/layer attributes and assign to path-function globals
`).concat(o.fetchCode,`
`).concat(o.varyingAssignments,`

  // Fetch node shape attributes for both endpoints
`).concat(h.fetchCode,`
`).concat(h.varyingAssignments,`
`).concat(f.fetchCode,`
`).concat(f.varyingAssignments,`

  // Fetch node geometry (texel 0) and rotation flags (texel 1).
  vec4 srcNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx);
  vec4 tgtNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx);

  vec2 a_source = srcNodeData.xy;
  vec2 a_target = tgtNodeData.xy;
  float a_sourceSize = srcNodeData.z;
  float a_targetSize = tgtNodeData.z;
  float a_sourceShapeId = srcNodeData.w;
  float a_targetShapeId = tgtNodeData.w;
  // Per-node rotation alignment (0 = viewport, 1 = graph), for boundary clamping.
  float a_sourceRotateAlign = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx).r;
  float a_targetRotateAlign = readNodeFlags(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx).r;

  v_sourceNodeSize = a_sourceSize;
  v_targetNodeSize = a_targetSize;

  float headLengthRatio = a_headLengthRatio;
  float tailLengthRatio = a_tailLengthRatio;
  float headWidthFactor = EXTREMITY_WIDTH_FACTORS[headId];
  float tailWidthFactor = EXTREMITY_WIDTH_FACTORS[tailId];
  float minBodyLengthRatio = `).concat(me(T),`;

  // Thickness in WebGL units (needed to compute clamping margin and extremity lengths)
  float pixelsThickness = max(a_thickness, u_minEdgeThickness * u_sizeRatio);
  float webGLThickness = pixelsThickness * u_correctionRatio / u_sizeRatio;

  // SDF clamping: find where the edge body meets the node boundaries. Always
  // searched (ungated) so labels get a true boundary clamp; the body re-applies
  // its extremity gating in-shader.
`).concat(g,`
  float tStart = queryFindSourceClampT(pathId, a_source, a_sourceSize, int(a_sourceShapeId), a_sourceRotateAlign, a_target, 0.0);
`).concat(b,`
  float tEnd = queryFindTargetClampT(pathId, a_source, a_target, a_targetSize, int(a_targetShapeId), a_targetRotateAlign, 0.0);

  // straightenFactor (frame .z) is consumed only by the body, which runs to the
  // node center when an extremity is absent. Derive the straightening math from
  // these gated clamps \u2014 not the ungated boundary ones above \u2014 so it matches the
  // geometry it drives. The ungated tStart/tEnd remain what labels read.
  float bodyTStart = tailLengthRatio > 0.0 ? tStart : 0.0;
  float bodyTEnd = headLengthRatio > 0.0 ? tEnd : 1.0;

  // Path length and zone boundaries (needed for straightening check)
  float pathLength = queryPathLength(pathId, a_source, a_target);
  float visibleLength = pathLength * (bodyTEnd - bodyTStart);

  float headLength = headLengthRatio * webGLThickness;
  float tailLength = tailLengthRatio * webGLThickness;
  float minBodyLength = minBodyLengthRatio * webGLThickness;

  float totalNeededLength = headLength + tailLength + minBodyLength;
  float extremityScale = 1.0;
  if (totalNeededLength > visibleLength && totalNeededLength > 0.0001) {
    extremityScale = visibleLength / totalNeededLength;
    headLength *= extremityScale;
    tailLength *= extremityScale;
  }

  float headLengthT = pathLength > 0.0001 ? headLength / pathLength : 0.0;
  float tailLengthT = pathLength > 0.0001 ? tailLength / pathLength : 0.0;

  float tTailEnd = bodyTStart + tailLengthT;
  float tHeadStart = bodyTEnd - headLengthT;
  if (tTailEnd > tHeadStart) {
    float mid = (bodyTStart + bodyTEnd) * 0.5;
    tTailEnd = mid;
    tHeadStart = mid;
  }

  // Straighten factor: blend toward straight line when path twists in extremity zones
  float straightenFactor = 0.0;
  {
    float maxDeviation = 0.0;
    if (tailLengthT > 0.0001) {
      vec2 tailTang = queryPathTangent(pathId, tTailEnd, a_source, a_target);
      vec2 tailChord = queryPathPosition(pathId, bodyTStart, a_source, a_target)
                     - queryPathPosition(pathId, tTailEnd, a_source, a_target);
      float tailChordLen = length(tailChord);
      if (tailChordLen > 0.0001) {
        maxDeviation = max(maxDeviation, 1.0 - dot(-tailTang, tailChord / tailChordLen));
      }
    }
    if (headLengthT > 0.0001) {
      vec2 headTang = queryPathTangent(pathId, tHeadStart, a_source, a_target);
      vec2 headChord = queryPathPosition(pathId, bodyTEnd, a_source, a_target)
                     - queryPathPosition(pathId, tHeadStart, a_source, a_target);
      float headChordLen = length(headChord);
      if (headChordLen > 0.0001) {
        maxDeviation = max(maxDeviation, 1.0 - dot(headTang, headChord / headChordLen));
      }
    }
    straightenFactor = smoothstep(0.035, 0.5, maxDeviation);
  }

  // When straightening, blend the ungated tStart/tEnd (the label-facing clamps)
  // toward straight-line clamp positions.
  if (straightenFactor > 0.001) {
    if (tailLengthRatio > 0.0) {
`).concat(g,`
      float srcExtent = a_sourceSize * u_correctionRatio / u_sizeRatio * 2.0;
      float srcEffective = 1.0 - u_correctionRatio / srcExtent;
      float srcCa = u_cameraAngle * (1.0 - a_sourceRotateAlign);
      mat2 srcRot = mat2(cos(srcCa), -sin(srcCa), sin(srcCa), cos(srcCa));
      float lo = 0.0, hi = 0.5;
      for (int i = 0; i < 12; i++) {
        float mid = (lo + hi) * 0.5;
        vec2 pos = mix(a_source, a_target, mid);
        vec2 localPos = srcRot * ((pos - a_source) / srcExtent);
        float sdf = querySDF(int(a_sourceShapeId), localPos, srcEffective);
        if (sdf < 0.0) lo = mid; else hi = mid;
      }
      tStart = mix(tStart, (lo + hi) * 0.5, straightenFactor);
    }
    if (headLengthRatio > 0.0) {
`).concat(b,`
      float tgtExtent = a_targetSize * u_correctionRatio / u_sizeRatio * 2.0;
      float tgtEffective = 1.0 - u_correctionRatio / tgtExtent;
      float tgtCa = u_cameraAngle * (1.0 - a_targetRotateAlign);
      mat2 tgtRot = mat2(cos(tgtCa), -sin(tgtCa), sin(tgtCa), cos(tgtCa));
      float lo = 0.5, hi = 1.0;
      for (int i = 0; i < 12; i++) {
        float mid = (lo + hi) * 0.5;
        vec2 pos = mix(a_source, a_target, mid);
        vec2 localPos = tgtRot * ((pos - a_target) / tgtExtent);
        float sdf = querySDF(int(a_targetShapeId), localPos, tgtEffective);
        if (sdf < 0.0) hi = mid; else lo = mid;
      }
      tEnd = mix(tEnd, (lo + hi) * 0.5, straightenFactor);
    }
  }

  v_clamp = vec4(tStart, tEnd, straightenFactor, pathLength);

  // Scatter this point to its edge's texel center in the frame texture.
  float x = mod(float(edgeIdx), u_frameTextureWidth);
  float y = floor(float(edgeIdx) / u_frameTextureWidth);
  vec2 ndc = (vec2(x, y) + 0.5) / vec2(u_frameTextureWidth, u_frameTextureHeight) * 2.0 - 1.0;
  gl_Position = vec4(ndc, 0.0, 1.0);
  gl_PointSize = 1.0;
}
`);return _}var jn=0,Ui=1,Xn=2;function sd(a,n,t){var e=[];t&&(e.push([jn,0,-1],[jn,0,1]),e.push([jn,1,-1],[jn,1,1]));for(var r=0;r<=a;r++){var i=r/a;e.push([Ui,i,-1],[Ui,i,1])}return n&&(e.push([Xn,0,-1],[Xn,0,1]),e.push([Xn,1,-1],[Xn,1,1])),{data:e,attributes:[{name:"a_zone",size:1,type:en},{name:"a_zoneT",size:1,type:en},{name:"a_side",size:1,type:en}],verticesPerEdge:e.length}}function ld(a,n,t,e){var r=Fe([].concat(J(a),J(t))),i=rn(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture"]),s=new Set,l=[],u=function(p){!o.has(p.name)&&!s.has(p.name)&&(s.add(p.name),l.push("uniform ".concat(p.type," ").concat(p.name,";")))};a.forEach(function(m){return m.uniforms.forEach(u)}),n.forEach(function(m){return m.uniforms.forEach(u)}),t.forEach(function(m){return m.uniforms.forEach(u)});var d=e.map(function(m){var p=m.size===1?"float":"vec".concat(m.size);return"in ".concat(p," ").concat(m.name,";")}).join(`
`),h=n.map(function(m){return me(m.widthFactor)}).join(", "),f=Math.max.apply(Math,J(a.map(function(m){return m.minBodyLengthRatio||0}))),c=t.some(function(m){return m.needsNodeColors}),g=a.some(function(m){return m.needsNodeSize}),b=`#version 300 es

// Constant attributes (per vertex)
`.concat(d,`

// Per-edge attributes
// Edge data (source/target indices, thickness, extremity ratios, path/extremity IDs)
// is fetched from edge data texture via edge index
in float a_edgeIndex;   // Index into edge data texture
in vec4 a_color;        // Edge color
in vec4 a_id;           // Edge ID for picking
in float a_opacity;     // Edge opacity

// Standard uniforms
uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_zoomRatio;
uniform float u_pixelRatio;
uniform float u_cameraAngle;
uniform float u_minEdgeThickness;
#ifdef PICKING_MODE
uniform float u_pickingPadding;
#endif
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_edgeDataTexture;
uniform int u_edgeDataTextureWidth;
uniform sampler2D u_edgeFrameTexture;
uniform int u_edgeFrameTextureWidth;

// Edge path attribute texture uniforms
`).concat(i.uniformDeclarations,`

// Custom uniforms
`).concat(l.join(`
`),`

// Standard varyings
out vec4 v_color;
out float v_opacity;
out vec4 v_id;
out float v_thickness;       // Edge body thickness (in consistent units)
out float v_t;
out float v_tStart;
out float v_tEnd;
out float v_side;
out float v_antialiasingWidth;  // Anti-aliasing width (normalized: u_correctionRatio / thickness)
out vec2 v_source;
out vec2 v_target;
out float v_edgeLength;
`).concat(g?`
out float v_sourceNodeSize;  // Source node size (mirrored in labels/generator.ts as plain float)
out float v_targetNodeSize;  // Target node size (mirrored in labels/generator.ts as plain float)`:"",`
`).concat(c?`
out vec4 v_sourceColor;
out vec4 v_targetColor;`:"",`
// Zone varyings
out float v_zone;            // 0=tail, 1=body, 2=head
out float v_zoneT;           // Position within zone [0,1]
out float v_headLengthRatio; // Head length as ratio of thickness
out float v_tailLengthRatio; // Tail length as ratio of thickness
out float v_headWidthRatio;  // Head width factor
out float v_tailWidthRatio;  // Tail width factor

// Multi-path/extremity varyings
flat out int v_pathId;
flat out int v_headId;
flat out int v_tailId;

// Path/layer attribute varyings (fetched from edge attribute texture)
`).concat(i.vertexVaryingDeclarations,`

const float bias = 255.0 / 254.0;

// Width factor array for extremities (shared pool for head/tail)
const float EXTREMITY_WIDTH_FACTORS[`).concat(n.length,"] = float[](").concat(h,`);

// All path functions
`).concat(Va(a),`

// Path selector functions
`).concat(ja(a),`

// Node-data fetch helper (geometry texel of the two-texel node stride)
`).concat(He,`
`).concat(c?vu:"",`
// Per-edge clamp from the frame-pass (tStart, tEnd, straightenFactor, pathLength)
`).concat(Pt,`

void main() {
  // Fetch edge data from edge texture (2 texels per edge)
  // Texel 0: sourceNodeIndex, targetNodeIndex, thickness, reserved
  // Texel 1: headLengthRatio, tailLengthRatio, pathId, (headId << 4) | tailId
  int edgeIdx = int(a_edgeIndex);

  // Hidden edges are flagged with a negative row by EdgeProgram.process(). Push
  // the whole primitive outside the clip volume: every vertex lands at the same
  // out-of-range position, so it is fully clipped and rasterizes nothing \u2014
  // neither to the frame buffer nor to the picking buffer.
  if (edgeIdx < 0) {
    gl_Position = vec4(2.0, 0.0, 0.0, 1.0);
    v_color = vec4(0.0);
    v_id = vec4(0.0);
    return;
  }

  int texel0Idx = edgeIdx * 2;
  int texel1Idx = edgeIdx * 2 + 1;
  ivec2 edgeTexCoord0 = ivec2(texel0Idx % u_edgeDataTextureWidth, texel0Idx / u_edgeDataTextureWidth);
  ivec2 edgeTexCoord1 = ivec2(texel1Idx % u_edgeDataTextureWidth, texel1Idx / u_edgeDataTextureWidth);
  vec4 edgeData0 = texelFetch(u_edgeDataTexture, edgeTexCoord0, 0);
  vec4 edgeData1 = texelFetch(u_edgeDataTexture, edgeTexCoord1, 0);

  // Unpack edge data
  int srcIdx = int(edgeData0.x);
  int tgtIdx = int(edgeData0.y);
  float a_thickness = edgeData0.z;
  // edgeData0.w is now reserved (curvature moved to path attribute texture)
  float a_headLengthRatio = edgeData1.x;
  float a_tailLengthRatio = edgeData1.y;
  int pathId = int(edgeData1.z);
  int extremityPacked = int(edgeData1.w);
  int headId = extremityPacked >> 4;
  int tailId = extremityPacked & 15;

  // Fetch path/layer attributes from edge attribute texture
`).concat(i.fetchCode,`

  // Assign path/layer attribute varyings
`).concat(i.varyingAssignments,`

  // Fetch source and target node geometry (texel 0 of the two-texel node stride)
  vec4 srcNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx);
  vec4 tgtNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx);

  vec2 a_source = srcNodeData.xy;
  vec2 a_target = tgtNodeData.xy;
`).concat(g?`
  // Assign node size varyings early (path functions like loops need them during clamping)
  v_sourceNodeSize = srcNodeData.z;
  v_targetNodeSize = tgtNodeData.z;`:"",`
`).concat(c?`
  v_sourceColor = readNodeColor(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx);
  v_targetColor = readNodeColor(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx);`:"",`

  // Convert thickness to WebGL units
  float minThickness = u_minEdgeThickness;
  float pixelsThickness = max(a_thickness, minThickness * u_sizeRatio);
  float webGLThickness = pixelsThickness * u_correctionRatio / u_sizeRatio;

  // Extremity parameters from ID lookups (shared pool)
  float headLengthRatio = a_headLengthRatio;
  float tailLengthRatio = a_tailLengthRatio;
  float headWidthFactor = EXTREMITY_WIDTH_FACTORS[headId];
  float tailWidthFactor = EXTREMITY_WIDTH_FACTORS[tailId];
  float minBodyLengthRatio = `).concat(me(f),`;

  // Per-edge values from the frame-pass texture (read by edge index).
  // tStart/tEnd are the ungated boundary clamps: re-apply the body's extremity
  // gating here (no extremity \u2192 body runs to the node center, 0/1).
  vec4 frameClamp = readFrameTexel(u_edgeFrameTexture, u_edgeFrameTextureWidth, edgeIdx);
  float tStart           = a_tailLengthRatio > 0.0 ? frameClamp.x : 0.0;
  float tEnd             = a_headLengthRatio > 0.0 ? frameClamp.y : 1.0;
  float straightenFactor = frameClamp.z;
  float pathLength       = frameClamp.w;

  // Anti-aliasing width (~1 pixel, normalized by thickness)
  float antialiasingWidth = u_correctionRatio / webGLThickness;

  float visibleLength = pathLength * (tEnd - tStart);

  // Compute extremity lengths in world units
  float headLength = headLengthRatio * webGLThickness;
  float tailLength = tailLengthRatio * webGLThickness;
  float minBodyLength = minBodyLengthRatio * webGLThickness;

  // Handle short edges: scale down extremities if needed
  float totalNeededLength = headLength + tailLength + minBodyLength;
  float extremityScale = 1.0;
  if (totalNeededLength > visibleLength && totalNeededLength > 0.0001) {
    extremityScale = visibleLength / totalNeededLength;
    headLength *= extremityScale;
    tailLength *= extremityScale;
  }

  // Convert lengths to t-values
  float headLengthT = pathLength > 0.0001 ? headLength / pathLength : 0.0;
  float tailLengthT = pathLength > 0.0001 ? tailLength / pathLength : 0.0;

  // Zone boundaries in t-space
  float tTailEnd = tStart + tailLengthT;
  float tHeadStart = tEnd - headLengthT;

  // Ensure body has non-negative length
  if (tTailEnd > tHeadStart) {
    float mid = (tStart + tEnd) * 0.5;
    tTailEnd = mid;
    tHeadStart = mid;
  }

  // Convert to webGL units for geometry expansion
  float aaWidthWebGL = antialiasingWidth * webGLThickness;

  // Extra geometry width for picking padding (0 in visual mode)
  #ifdef PICKING_MODE
    float pickingPaddingWebGL = u_pickingPadding * u_correctionRatio;
  #else
    float pickingPaddingWebGL = 0.0;
  #endif

  // Straight-line direction and normal (for blending when straightenFactor > 0)
  vec2 straightDir = length(a_target - a_source) > 0.0001
    ? normalize(a_target - a_source) : vec2(1.0, 0.0);
  vec2 straightNormal = vec2(-straightDir.y, straightDir.x);

  // Zone-based vertex processing using path selectors
  vec2 position;
  vec2 normal;
  float t;
  float zone = a_zone;
  float zoneT = a_zoneT;
  float side = a_side;

  // Scaled extremity width factors (geometry must be at least as wide as body)
  float scaledTailWidth = max(tailWidthFactor * extremityScale, 1.0);
  float scaledHeadWidth = max(headWidthFactor * extremityScale, 1.0);

  if (zone < 0.5) {
    // TAIL ZONE: rectangular quad with scaled width
    vec2 tang = queryPathTangent(pathId, tTailEnd, a_source, a_target);
    normal = vec2(-tang.y, tang.x);
    vec2 centerPos = mix(queryPathPosition(pathId, tStart, a_source, a_target),
                         queryPathPosition(pathId, tTailEnd, a_source, a_target), zoneT);
    float halfWidth = webGLThickness * scaledTailWidth * 0.5 + aaWidthWebGL + pickingPaddingWebGL;
    position = centerPos + normal * side * halfWidth;
    t = mix(tStart, tTailEnd, zoneT);

  } else if (zone < 1.5) {
    // BODY ZONE: follows path curvature with width = 1.0
    t = mix(tTailEnd, tHeadStart, zoneT);
    normal = queryPathNormal(pathId, t, a_source, a_target);
    float halfWidth = webGLThickness * 0.5 + aaWidthWebGL + pickingPaddingWebGL;
    position = queryPathPosition(pathId, t, a_source, a_target) + normal * side * halfWidth;

  } else {
    // HEAD ZONE: rectangular quad with scaled width
    vec2 tang = queryPathTangent(pathId, tHeadStart, a_source, a_target);
    normal = vec2(-tang.y, tang.x);
    vec2 centerPos = mix(queryPathPosition(pathId, tHeadStart, a_source, a_target),
                         queryPathPosition(pathId, tEnd, a_source, a_target), zoneT);
    float halfWidth = webGLThickness * scaledHeadWidth * 0.5 + aaWidthWebGL + pickingPaddingWebGL;
    position = centerPos + normal * side * halfWidth;
    t = mix(tHeadStart, tEnd, zoneT);
  }

  // Blend toward straight line based on path twist in extremity zones
  if (straightenFactor > 0.001) {
    float zoneWidth = zone < 0.5 ? webGLThickness * scaledTailWidth * 0.5 + aaWidthWebGL + pickingPaddingWebGL :
                      zone < 1.5 ? webGLThickness * 0.5 + aaWidthWebGL + pickingPaddingWebGL :
                      webGLThickness * scaledHeadWidth * 0.5 + aaWidthWebGL + pickingPaddingWebGL;
    vec2 straightPos = mix(a_source, a_target, t) + straightNormal * side * zoneWidth;
    position = mix(position, straightPos, straightenFactor);
  }

  gl_Position = vec4((u_matrix * vec3(position, 1.0)).xy, 0.0, 1.0);

  // Pass varyings to fragment shader
  v_color = a_color;
  v_color.a *= bias;
  v_opacity = a_opacity;
  v_id = a_id;
  v_thickness = webGLThickness;
  v_t = t;
  v_tStart = tStart;
  v_tEnd = tEnd;
  v_side = side;
  v_antialiasingWidth = antialiasingWidth;
  v_source = a_source;
  v_target = a_target;
  v_edgeLength = pathLength;

  // Zone varyings
  v_zone = zone;
  v_zoneT = zoneT;
  v_headLengthRatio = headLengthRatio * extremityScale;
  v_tailLengthRatio = tailLengthRatio * extremityScale;
  // Scale extremity width proportionally with length when crushed
  v_headWidthRatio = headWidthFactor * extremityScale;
  v_tailWidthRatio = tailWidthFactor * extremityScale;

  // Multi-path varyings
  v_pathId = pathId;
  v_headId = headId;
  v_tailId = tailId;
}
`);return b}function ud(a,n,t,e){var r=Fe([].concat(J(a),J(t))),i=rn(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding"]),s=new Set,l=[],u=function(v){!o.has(v.name)&&!s.has(v.name)&&(s.add(v.name),l.push("uniform ".concat(v.type," ").concat(v.name,";")))};a.forEach(function(p){return p.uniforms.forEach(u)}),n.forEach(function(p){return p.uniforms.forEach(u)}),t.forEach(function(p){return p.uniforms.forEach(u)});var d=n.map(function(p){var v;return me((v=p.baseRatio)!==null&&v!==void 0?v:.5)}).join(", "),h=function(v){var y,T;return n.length>1?"EXTREMITY_BASE_RATIOS[".concat(v,"]"):me((y=(T=n[0])===null||T===void 0?void 0:T.baseRatio)!==null&&y!==void 0?y:.5)},f=n.some(function(p){return Ue(p.length)?!0:p.length>0}),c=t.some(function(p){return p.needsNodeColors}),g=a.some(function(p){return p.needsNodeSize}),b=t.map(function(p,v){return"  // Layer ".concat(v+1,": ").concat(p.name,`
  color = blendOver(color, layer_`).concat(p.name,"(context));")}).join(`

`),m=`#version 300 es
precision highp float;

// Standard varyings
in vec4 v_color;
in float v_opacity;
in vec4 v_id;
in float v_thickness;       // Edge body thickness
in float v_t;
in float v_tStart;
in float v_tEnd;
in float v_side;
in float v_antialiasingWidth;  // Anti-aliasing width (normalized: u_correctionRatio / thickness)
in vec2 v_source;
in vec2 v_target;
in float v_edgeLength;
`.concat(g?`
in float v_sourceNodeSize;   // Source node size (mirrored in labels/generator.ts as plain float)
in float v_targetNodeSize;   // Target node size (mirrored in labels/generator.ts as plain float)`:"",`
`).concat(c?`
in vec4 v_sourceColor;
in vec4 v_targetColor;`:"",`
// Zone varyings
in float v_zone;            // 0=tail, 1=body, 2=head
in float v_zoneT;           // Position within zone [0,1]
in float v_headLengthRatio; // Head length as ratio of thickness (scaled for short edges)
in float v_tailLengthRatio; // Tail length as ratio of thickness (scaled for short edges)
in float v_headWidthRatio;  // Head width factor
in float v_tailWidthRatio;  // Tail width factor

// Multi-path/extremity varyings
flat in int v_pathId;
flat in int v_headId;
flat in int v_tailId;

// Path/layer attribute varyings (from vertex shader texture fetch)
`).concat(i.fragmentVaryingDeclarations,`

// Standard uniforms (needed by some path types)
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_cameraAngle;
#ifdef PICKING_MODE
uniform float u_pickingPadding;
#endif

// Custom uniforms
`).concat(l.join(`
`),`

// Fragment output (single target - picking handled via separate pass)
out vec4 fragColor;

`).concat(n.length>1?`// Base ratio array for extremities (shared pool for head/tail)
const float EXTREMITY_BASE_RATIOS[`.concat(n.length,"] = float[](").concat(d,");"):"",`

// EdgeContext struct
struct EdgeContext {
  float t;                   // Position along path [0, 1]
  float sdf;                 // Signed distance from centerline
  vec2 position;             // World position
  vec2 tangent;              // Path tangent
  vec2 normal;               // Path normal
  float thickness;           // Edge thickness
  float aaWidth;             // Anti-aliasing width
  float edgeLength;          // Total path length
  float tStart;              // Clamped start t
  float tEnd;                // Clamped end t
  float distanceFromSource;  // Arc distance from source
  float distanceToTarget;    // Arc distance to target
};

EdgeContext context;

// Alpha "over" compositing for layer blending
vec4 blendOver(vec4 bg, vec4 fg) {
  float a = fg.a;
  return vec4(mix(bg.rgb, fg.rgb, a), bg.a + a * (1.0 - bg.a));
}

// All path functions
`).concat(Va(a),`

// Path selector functions
`).concat(ja(a),`

// All extremity functions
`).concat(td(n),`

// Extremity SDF selector (shared pool for head/tail)
`).concat(rd(n),`

// Layer functions
`).concat(t.map(function(p){return p.glsl}).join(`

`),`

// Helper: Compute arc length using path selector
float computeArcLengthMulti(int pathId, float t0, float t1, vec2 source, vec2 target, int samples) {
  float arcLen = 0.0;
  vec2 prev = queryPathPosition(pathId, t0, source, target);
  for (int i = 1; i <= samples; i++) {
    float t = t0 + (t1 - t0) * float(i) / float(samples);
    vec2 curr = queryPathPosition(pathId, t, source, target);
    arcLen += length(curr - prev);
    prev = curr;
  }
  return arcLen;
}

void main() {
  // Compute normalized t within visible edge (0 = start, 1 = end)
  float tNorm = (v_t - v_tStart) / max(v_tEnd - v_tStart, 0.0001);

  // Edge body half-thickness
  float halfThickness = v_thickness * 0.5;

  // Convert normalized AA width to webGL units (~1 pixel)
  float aaWidthWebGL = v_antialiasingWidth * v_thickness;

  // Distance from centerline based on v_side interpolation
  float zoneWidthFactor = v_zone < 0.5 ? v_tailWidthRatio :
                          v_zone < 1.5 ? 1.0 :
                          v_headWidthRatio;
  // In PICKING_MODE, inflate halfGeometryWidth to match the inflated vertex geometry
  #ifdef PICKING_MODE
    float halfGeometryWidth = halfThickness * zoneWidthFactor + aaWidthWebGL + u_pickingPadding * aaWidthWebGL;
  #else
    float halfGeometryWidth = halfThickness * zoneWidthFactor + aaWidthWebGL;
  #endif
  float distFromCenter = abs(v_side) * halfGeometryWidth;

  // Populate EdgeContext (for layer functions)
  context.t = tNorm;
  context.sdf = distFromCenter - halfThickness;
  context.position = queryPathPosition(v_pathId, v_t, v_source, v_target);
  context.tangent = queryPathTangent(v_pathId, v_t, v_source, v_target);
  context.normal = vec2(-context.tangent.y, context.tangent.x);
  context.thickness = v_thickness;
  context.aaWidth = aaWidthWebGL;
  context.edgeLength = v_edgeLength;
  context.tStart = v_tStart;
  context.tEnd = v_tEnd;

  // Compute arc distances
  float visibleLength = v_edgeLength * (v_tEnd - v_tStart);
  float pathT = v_t;
  float pathTNorm = tNorm;
`).concat(a.every(function(p){return p.linearParameterization})?`  // All paths have linear parameterization: t maps directly to arc distance
  context.distanceFromSource = pathTNorm * visibleLength;
  context.distanceToTarget = (1.0 - pathTNorm) * visibleLength;`:a.every(function(p){return!p.linearParameterization})?`  // No paths have linear parameterization: use numerical integration
  context.distanceFromSource = computeArcLengthMulti(v_pathId, v_tStart, pathT, v_source, v_target, 16);
  context.distanceToTarget = computeArcLengthMulti(v_pathId, pathT, v_tEnd, v_source, v_target, 16);`:`  // Mixed parameterization: use analytical for linear paths, numerical for others
  if (`.concat(a.filter(function(p){return p.linearParameterization}).map(function(p){return"v_pathId == ".concat(a.indexOf(p))}).join(" || "),`) {
    context.distanceFromSource = pathTNorm * visibleLength;
    context.distanceToTarget = (1.0 - pathTNorm) * visibleLength;
  } else {
    context.distanceFromSource = computeArcLengthMulti(v_pathId, v_tStart, pathT, v_source, v_target, 16);
    context.distanceToTarget = computeArcLengthMulti(v_pathId, pathT, v_tEnd, v_source, v_target, 16);
  }`),`

  // Compute SDF based on zone using extremity selector (shared pool)
  float bodySDF = distFromCenter - halfThickness;
  float finalSDF;

`).concat(f?`  // Base ratios, from baseRatioLookup
  float headBaseRatio = `.concat(h("v_headId"),`;
  float tailBaseRatio = `).concat(h("v_tailId"),`;

  if (v_zone < 0.5) {
    // TAIL ZONE: v_zoneT goes 0 (tip) to 1 (base)
    vec2 uv = vec2((1.0 - v_zoneT) * v_tailLengthRatio, v_side * v_tailWidthRatio * 0.5);
    float tailSDF = queryExtremitySDF(v_tailId, uv, v_tailLengthRatio, v_tailWidthRatio) * v_thickness;

    // Apply union only near base (v_zoneT > 1 - baseRatio)
    if (v_zoneT > 1.0 - tailBaseRatio) {
      finalSDF = min(tailSDF, bodySDF);
    } else {
      finalSDF = tailSDF;
    }
  } else if (v_zone < 1.5) {
    // BODY ZONE: distance from centerline
    finalSDF = bodySDF;
  } else {
    // HEAD ZONE: v_zoneT goes 0 (base) to 1 (tip)
    vec2 uv = vec2(v_zoneT * v_headLengthRatio, v_side * v_headWidthRatio * 0.5);
    float headSDF = queryExtremitySDF(v_headId, uv, v_headLengthRatio, v_headWidthRatio) * v_thickness;

    // Apply union only near base (v_zoneT < baseRatio)
    if (v_zoneT < headBaseRatio) {
      finalSDF = min(headSDF, bodySDF);
    } else {
      finalSDF = headSDF;
    }
  }`):`  // No extremity draws tail/head geometry, so every vertex is body zone.
  finalSDF = bodySDF;`,`

  #ifdef PICKING_MODE
    // Picking pass: output edge ID for pixels within the picking area
    if (finalSDF > u_pickingPadding * aaWidthWebGL) discard;
    fragColor = v_id;
  #else
`).concat(e?`    // Visual pass: anti-aliased edge with layers, edge opacity applied once
    float alpha = smoothstep(aaWidthWebGL, -aaWidthWebGL, finalSDF) * v_opacity;`:`    // Visual pass: hard-edged (no anti-aliasing gradient) edge with layers, edge opacity applied once
    float alpha = (finalSDF < 0.0 ? 1.0 : 0.0) * v_opacity;`,`
    if (alpha < 0.01) discard;

    // Apply layers sequentially with "over" compositing
    vec4 color = vec4(0.0);

`).concat(b,`

    // Mix with transparent to fade both color AND alpha (pre-multiplied alpha for correct blending)
    fragColor = mix(vec4(0.0), color, alpha);
  #endif
}
`);return m}function dd(a,n,t){var e=Ue(n.length)?!0:n.length>0,r=Ue(t.length)?!0:t.length>0;if(a.generateConstantData){var i=a.generateConstantData();return{data:i.data,attributes:i.attributes,verticesPerEdge:i.verticesPerEdge}}return sd(a.segments,e,r)}function Oi(a){var n,t=oo(a),e=t.paths,r=t.extremities,i=t.layers,o=(n=a.antialias)!==null&&n!==void 0?n:!0,s=new Map,l=new Map,u=new Map,d=U(e),h;try{for(d.s();!(h=d.n()).done;){var f=h.value,c=U(r),g;try{for(c.s();!(g=c.n()).done;){var b=g.value,m=U(r),p;try{for(m.s();!(p=m.n()).done;){var v=p.value,y="".concat(f.name,":").concat(b.name,":").concat(v.name),T=dd(f,b,v);s.set(y,T.verticesPerEdge),l.set(y,T.data);var _=U(T.attributes),S;try{for(_.s();!(S=_.n()).done;){var D=S.value;u.has(D.name)||u.set(D.name,D)}}catch(V){_.e(V)}finally{_.f()}}}catch(V){m.e(V)}finally{m.f()}}}catch(V){c.e(V)}finally{c.f()}}}catch(V){d.e(V)}finally{d.f()}var w=Array.from(u.values()),k={};w.forEach(function(V,Y){k[V.name]=Y});var L=0,N="",R=U(s),z;try{for(R.s();!(z=R.n()).done;){var C=oe(z.value,2),I=C[0],B=C[1];B>L&&(L=B,N=I)}}catch(V){R.e(V)}finally{R.f()}var F=l.get(N)||[],x=N.split(":"),P=oe(x,1),W=P[0],G=e.find(function(V){return V.name===W}),A=[];G!=null&&G.generateConstantData?A=G.generateConstantData().attributes:A=[{name:"a_zone"},{name:"a_zoneT"},{name:"a_side"}];var q=F.map(function(V){var Y=new Array(w.length).fill(0);return A.forEach(function(pe,se){var Z=k[pe.name];Z!==void 0&&se<V.length&&(Y[Z]=V[se])}),Y});return{vertexShader:ld(e,r,i,w),fragmentShader:ud(e,r,i,o),uniforms:ed(e,r,i),attributes:hd(),verticesPerEdge:L,constantData:q,constantAttributes:w,vertexCountsPerCombination:s,constantDataPerCombination:l}}function hd(a,n,t){return[{name:"a_edgeIndex",size:1,type:en},{name:"a_color",size:4,type:Wi,normalized:!0},{name:"a_id",size:4,type:Wi,normalized:!0},{name:"a_opacity",size:1,type:en}]}var cd=`#version 300 es
precision highp float;

in vec4 v_clamp;

// RGBA32F target: stores (tStart, tEnd, straightenFactor, pathLength).
out vec4 fragColor;

void main() {
  fragColor = v_clamp;
}
`,fd=(function(){function a(n,t){ne(this,a),E(this,"uniformLocations",{});var e=t.paths,r=t.extremities,i=t.layers,o=t.nodeShapes,s=t.nodeLayers;this.gl=n,this.hasAttributeData=Fe([].concat(J(e),J(i))).floatsPerItem>0,this.hasNodeAttributeData=Object.keys(Ft(o,s).offsets).length>0,this.vertexShader=Ma(n,od(e,r,i,o,s)),this.fragmentShader=Ba(n,cd),this.program=Wa(n,[this.vertexShader,this.fragmentShader]);var l=new Set;this.customUniforms=[];for(var u=0,d=[].concat(J(e),J(r));u<d.length;u++){var h=d[u],f=U(h.uniforms),c;try{for(f.s();!(c=f.n()).done;){var g=c.value;l.has(g.name)||(l.add(g.name),this.customUniforms.push(g))}}catch(y){f.e(y)}finally{f.f()}}var b=["u_sizeRatio","u_correctionRatio","u_cameraAngle","u_minEdgeThickness","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode","u_frameTextureWidth","u_frameTextureHeight"].concat(J(this.customUniforms.map(function(y){return y.name}))),m=U(b),p;try{for(m.s();!(p=m.n()).done;){var v=p.value;this.uniformLocations[v]=n.getUniformLocation(this.program,v)}}catch(y){m.e(y)}finally{m.f()}this.vao=n.createVertexArray()}return ae(a,[{key:"run",value:function(t,e,r,i,o){if(r!==0){var s=this.gl,l=this.uniformLocations;s.useProgram(this.program),s.bindVertexArray(this.vao),l.u_sizeRatio&&s.uniform1f(l.u_sizeRatio,t.sizeRatio),l.u_correctionRatio&&s.uniform1f(l.u_correctionRatio,t.correctionRatio),l.u_cameraAngle&&s.uniform1f(l.u_cameraAngle,t.cameraAngle),l.u_minEdgeThickness&&s.uniform1f(l.u_minEdgeThickness,t.minEdgeThickness),l.u_nodeDataTexture&&s.uniform1i(l.u_nodeDataTexture,t.nodeDataTextureUnit),l.u_nodeDataTextureWidth&&s.uniform1i(l.u_nodeDataTextureWidth,t.nodeDataTextureWidth),l.u_edgeDataTexture&&s.uniform1i(l.u_edgeDataTexture,t.edgeDataTextureUnit),l.u_edgeDataTextureWidth&&s.uniform1i(l.u_edgeDataTextureWidth,t.edgeDataTextureWidth),l.u_frameTextureWidth&&s.uniform1f(l.u_frameTextureWidth,e.getTextureWidth()),l.u_frameTextureHeight&&s.uniform1f(l.u_frameTextureHeight,e.getTextureHeight()),this.hasAttributeData&&i&&(i.bind(Ye),l.u_edgeAttributeTexture&&s.uniform1i(l.u_edgeAttributeTexture,Ye),l.u_edgeAttributeTextureWidth&&s.uniform1i(l.u_edgeAttributeTextureWidth,i.getTextureWidth()),l.u_edgeAttributeTexelsPerEdge&&s.uniform1i(l.u_edgeAttributeTexelsPerEdge,i.getTexelsPerItem())),this.hasNodeAttributeData&&o&&(o.bind(Ke),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,Ke),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=U(this.customUniforms),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;$a(s,this.uniformLocations[h.name],h)}}catch(f){u.e(f)}finally{u.f()}e.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,r),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var t=this.gl;t.deleteProgram(this.program),t.deleteShader(this.vertexShader),t.deleteShader(this.fragmentShader),t.deleteVertexArray(this.vao)}}])})();function Qt(a,n,t,e,r,i){if(a.length===1)return"".concat(e," ").concat(n,"(int pathId, ").concat(r,`) {
  return path_`).concat(a[0].name,"_").concat(t,"(").concat(i,`);
}`);var o=a.map(function(s,l){return"    case ".concat(l,": return path_").concat(s.name,"_").concat(t,"(").concat(i,");")}).join(`
`);return"".concat(e," ").concat(n,"(int pathId, ").concat(r,`) {
  switch (pathId) {
`).concat(o,`
    default: return path_`).concat(a[0].name,"_").concat(t,"(").concat(i,`);
  }
}`)}var gd=`
vec3 computeEdgeLabelBodyBounds(
  float tStart, float tEnd, float pathLength,
  float webGLThickness, float headLengthRatio, float tailLengthRatio
) {
  float visibleLength = pathLength * (tEnd - tStart);

  float headLength = headLengthRatio * webGLThickness;
  float tailLength = tailLengthRatio * webGLThickness;
  float totalNeededLength = headLength + tailLength;
  if (totalNeededLength > visibleLength && totalNeededLength > 0.0001) {
    float extremityScale = visibleLength / totalNeededLength;
    headLength *= extremityScale;
    tailLength *= extremityScale;
  }

  float bodyStartDist = tStart * pathLength + tailLength;
  float bodyEndDist = tEnd * pathLength - headLength;
  return vec3(bodyStartDist, bodyEndDist, max(bodyEndDist - bodyStartDist, 0.0));
}
`;function pd(a,n){var t=me(a),e=me(n);return`
float computeEdgeLabelAlpha(float bodyLength, float textWidthWebGL) {
  float ratio = textWidthWebGL > 0.0001 ? min(bodyLength / textWidthWebGL, 1.0) : 1.0;
  if (ratio < `.concat(t,`) return 0.0;
  if (ratio < `).concat(e,") return (ratio - ").concat(t,") / (").concat(e," - ").concat(t,`);
  return 1.0;
}
`)}var vd=`
float computeEdgeLabelScreenDir(int pathId, float t, vec2 source, vec2 target, mat3 matrix) {
  vec2 tangent = queryPathTangent(pathId, t, source, target);
  return (matrix * vec3(tangent, 0.0)).x < 0.0 ? -1.0 : 1.0;
}
`,md=`
float computeEdgeLabelPerpOffset(
  float positionMode,
  float halfThickness, float marginWebGL, float halfTextHeight,
  float readingSign, float screenDir
) {
  float magnitude = halfThickness + marginWebGL + halfTextHeight;
  if (positionMode == 1.0) return magnitude * readingSign;
  if (positionMode == 2.0) return -magnitude * readingSign;
  if (positionMode == 3.0) return magnitude * screenDir;
  return 0.0;
}
`;function lo(a){var n=a.paths,t=a.minVisibilityThreshold,e=a.fullVisibilityThreshold,r=n.some(function(s){return s.hasSharpCorners}),i=n.map(function(s){return"// --- Path: ".concat(s.name,` ---
`).concat(s.glsl,`

// Tangent/normal functions: analytical if provided, otherwise numerical
`).concat(s.analyticalTangentGlsl||ro(s.name),`

// Auto-generated fallbacks for any missing path functions
`).concat(io(s.name,s.glsl),`

// Corner skip helpers (for paths with sharp corners like step/taxi)
`).concat(s.cornerSkipGlsl||"",`
`)}).join(`
`),o=r?`// Corner function selectors (only some paths have sharp corners)
vec2 queryGetCornerTs(int pathId, vec2 source, vec2 target) {
  switch (pathId) {
`.concat(n.map(function(s,l){return s.hasSharpCorners?"    case ".concat(l,": return path_").concat(s.name,"_getCornerTs(source, target);"):"    case ".concat(l,": return vec2(-1.0, -1.0); // No corners for ").concat(s.name)}).join(`
`),`
    default: return vec2(-1.0, -1.0);
  }
}

vec2 queryGetCornerConcavity(int pathId, vec2 source, vec2 target, float perpOffset) {
  switch (pathId) {
`).concat(n.map(function(s,l){return s.hasSharpCorners?"    case ".concat(l,": return path_").concat(s.name,"_getCornerConcavity(source, target, perpOffset);"):"    case ".concat(l,": return vec2(0.0, 0.0); // No corners for ").concat(s.name)}).join(`
`),`
    default: return vec2(0.0, 0.0);
  }
}`):"";return`
// ============================================================================
// Node data fetch (geometry texel) and per-edge clamp fetch (edge-frame texture)
// ============================================================================

`.concat(He,`
`).concat(Pt,`

// ============================================================================
// Path Functions (one block per path)
// ============================================================================

`).concat(i,`

// ============================================================================
// Path Query Selectors (dispatch by pathId)
// ============================================================================

`).concat(Qt(n,"queryPathPosition","position","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Qt(n,"queryPathTangent","tangent","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Qt(n,"queryPathNormal","normal","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Qt(n,"queryPathLength","length","float","vec2 source, vec2 target","source, target"),`

`).concat(Qt(n,"queryPathTAtDistance","t_at_distance","float","float dist, vec2 source, vec2 target","dist, source, target"),`

`).concat(o,`

// ============================================================================
// Shared helpers (body bounds, alpha ramp, screen direction, perpendicular offset)
// ============================================================================

`).concat(gd,`
`).concat(pd(t,e),`
`).concat(vd,`
`).concat(md,`
`)}var bd=Ne.fontSize,yd=new Map,uo=24,Hi=(uo+1)*2;function xd(a){var n=a.paths,t=a.fontSizeMode,e=a.headLengthRatio,r=a.tailLengthRatio,i=a.minVisibilityThreshold,o=a.fullVisibilityThreshold,s=a.keepUpright,l=t==="scaled",u=at(),d=Fe([].concat(J(n),[u])),h=rn(d),f=`#version 300 es

// Per-instance attributes
in float a_edgeIndex;
in float a_edgeAttrIndex;
in float a_baseFontSize;
in float a_totalTextWidth;
in float a_positionMode;
in float a_margin;
in float a_padding;
in vec4 a_color;
in vec4 a_id;

// Per-vertex (constant) attribute: strip vertex index
in float a_vertexIndex;

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_pixelRatio;
uniform float u_cameraAngle;
uniform vec2 u_resolution;
uniform sampler2D u_nodeDataTexture;
uniform int u_nodeDataTextureWidth;
uniform sampler2D u_edgeDataTexture;
uniform int u_edgeDataTextureWidth;
uniform sampler2D u_edgeFrameTexture; // Per-edge clamp (tStart, tEnd, straightenFactor, pathLength)
uniform int u_edgeFrameTextureWidth;
`.concat(l?"uniform float u_zoomSizeRatio;":"",`

`).concat(h.uniformDeclarations,`

out vec4 v_color;
out vec4 v_id;
out float v_alphaModifier;

const float ATLAS_FONT_SIZE = `).concat(me(bd),`;
const float HEAD_RATIO = `).concat(me(e),`;
const float TAIL_RATIO = `).concat(me(r),`;
const int RIBBON_SEGMENTS = `).concat(uo,`;

// Path attribute varyings (declared as plain locals since this shader has no FS inputs for them)
`).concat(h.vertexVaryingDeclarations.replace(/out /g,""),`

// Node size variables used by some path functions (self-loops, etc.)
float v_sourceNodeSize;
float v_targetNodeSize;

// Shared preamble: shape SDFs, path functions + selectors, clamp, helpers.
`).concat(lo({paths:n,minVisibilityThreshold:i,fullVisibilityThreshold:o}),`

void main() {
  int vIdx = int(a_vertexIndex);
  int pairIdx = vIdx / 2;
  int side = vIdx - pairIdx * 2; // 0 = left/bottom, 1 = right/top

  // --- Fetch edge data (2 texels per edge) ---
  int edgeIdx = int(a_edgeIndex);
  int texel0Idx = edgeIdx * 2;
  int texel1Idx = edgeIdx * 2 + 1;
  ivec2 e0 = ivec2(texel0Idx % u_edgeDataTextureWidth, texel0Idx / u_edgeDataTextureWidth);
  ivec2 e1 = ivec2(texel1Idx % u_edgeDataTextureWidth, texel1Idx / u_edgeDataTextureWidth);
  vec4 edgeData0 = texelFetch(u_edgeDataTexture, e0, 0);
  vec4 edgeData1 = texelFetch(u_edgeDataTexture, e1, 0);

  int srcIdx = int(edgeData0.x);
  int tgtIdx = int(edgeData0.y);
  float thickness = edgeData0.z;
  int pathId = int(edgeData1.z);

  // --- Fetch path attributes (curvature, etc.) ---
  {
    int edgeIdx = int(a_edgeAttrIndex);
`).concat(h.fetchCode,`
`).concat(h.varyingAssignments,`
  }

  // --- Fetch node data: geometry (texel 0) + rotation flags (texel 1) ---
  vec4 srcN = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx);
  vec4 tgtN = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx);

  vec2 source = srcN.xy;
  vec2 target = tgtN.xy;
  float sourceSize = srcN.z;
  float targetSize = tgtN.z;
  v_sourceNodeSize = sourceSize;
  v_targetNodeSize = targetSize;

  // --- Pixel-to-graph conversion (fixed font mode) ---
  float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
  float pixelToGraph = 2.0 / (matrixScaleX * u_resolution.x);

  float webGLThickness = thickness * u_correctionRatio / u_sizeRatio;

  // --- Body bounds (shared with edge label shader) ---
  vec4 edgeClamp = readFrameTexel(u_edgeFrameTexture, u_edgeFrameTextureWidth, edgeIdx);
  vec3 bodyBounds = computeEdgeLabelBodyBounds(
    edgeClamp.x, edgeClamp.y, edgeClamp.w,
    webGLThickness, HEAD_RATIO, TAIL_RATIO
  );
  float bodyStartDist = bodyBounds.x;
  float bodyEndDist = bodyBounds.y;
  float bodyLength = bodyBounds.z;

  // --- Text dimensions ---
  float baseFontSize = a_baseFontSize;
  `).concat(l?`float fontScale = baseFontSize / ATLAS_FONT_SIZE * u_zoomSizeRatio;
  float textWidthWebGL = a_totalTextWidth * fontScale * u_correctionRatio / u_sizeRatio;
  float halfTextHeight = baseFontSize * 0.35 * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;
  float marginWebGL = a_margin * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;`:`float fontScale = baseFontSize / ATLAS_FONT_SIZE;
  float textWidthWebGL = a_totalTextWidth * fontScale * pixelToGraph;
  float halfTextHeight = baseFontSize * 0.35 * pixelToGraph;
  float marginWebGL = a_margin * pixelToGraph;`,`
  float paddingWebGL = a_padding * pixelToGraph;

  // --- Alpha modifier (shared with edge label shader) ---
  float alphaModifier = computeEdgeLabelAlpha(bodyLength, textWidthWebGL);
  if (alphaModifier <= 0.0 || textWidthWebGL <= 0.0) {
    gl_Position = vec4(2.0, 0.0, 0.0, 1.0);
    v_color = vec4(0.0);
    v_id = vec4(0.0);
    v_alphaModifier = 0.0;
    return;
  }

  // --- Perpendicular offset (shared with edge label shader) ---
  float bodyCenterDist = (bodyStartDist + bodyEndDist) * 0.5;
  float centerT = queryPathTAtDistance(pathId, bodyCenterDist, source, target);
  float screenDir = computeEdgeLabelScreenDir(pathId, centerT, source, target, u_matrix);
  float readingSign = `).concat(s?"screenDir":"1.0",`;
  float halfThickness = webGLThickness * 0.5;
  float perpOffset = computeEdgeLabelPerpOffset(
    a_positionMode, halfThickness, marginWebGL, halfTextHeight, readingSign, screenDir
  );

  // --- Ribbon span (clipped to body) ---
  float halfTextWebGL = textWidthWebGL * 0.5;
  float labelStartDist = max(bodyCenterDist - halfTextWebGL, bodyStartDist);
  float labelEndDist = min(bodyCenterDist + halfTextWebGL, bodyEndDist);

  // Sample centerline at this pair, then apply perpendicular offset.
  // The ribbon follows the centerline path (simple & robust); for curved edges
  // this closely matches the offset path the characters sit on.
  float u = float(pairIdx) / float(RIBBON_SEGMENTS);
  float arcDist = mix(labelStartDist, labelEndDist, u);
  float t = queryPathTAtDistance(pathId, arcDist, source, target);
  vec2 pos = queryPathPosition(pathId, t, source, target);
  vec2 tan = queryPathTangent(pathId, t, source, target);
  vec2 perp = vec2(-tan.y, tan.x);

  vec2 centerPos = pos + perp * perpOffset;

  float halfRibbon = halfTextHeight + paddingWebGL;
  float sideSign = side == 0 ? -1.0 : 1.0;
  vec2 ribbonPos = centerPos + perp * (sideSign * halfRibbon);

  vec3 clipPos = u_matrix * vec3(ribbonPos, 1.0);
  gl_Position = vec4(clipPos.xy, 0.0, 1.0);

  v_color = a_color;
  v_id = a_id;
  v_alphaModifier = alphaModifier;
}
`);return f}var _d=`#version 300 es
precision highp float;

in vec4 v_color;
in vec4 v_id;
in float v_alphaModifier;

out vec4 fragColor;

void main() {
#ifdef PICKING_MODE
  if (v_alphaModifier <= 0.0) discard;
  fragColor = v_id;
#else
  float alpha = v_color.a * v_alphaModifier;
  if (alpha <= 0.0) discard;
  fragColor = vec4(v_color.rgb * alpha, alpha);
#endif
}
`;function Td(a,n,t,e){var r=e.shaderConfig,i=r.paths,o=r.fontSizeMode;if(i.length===0)throw new Error("createEdgeLabelBackgroundProgram: shaderConfig must declare at least one path");var s=at(),l=Fe([].concat(J(i),[s])),u=Qn([].concat(J(i),[s]),l),d=xd(r),h=(function(f){function c(g,b,m){var p;return ne(this,c),p=fe(this,c,[g,b,m]),E(p,"totalCount",0),E(p,"bufferCapacity",0),E(p,"edgeAttributeTexture",null),p.edgeAttributeTexture=new Zn(g,l),p.packedAttributeData=new Float32Array(l.floatsPerItem),p}return ge(c,f),ae(c,[{key:"getDefinition",value:function(){for(var b=WebGL2RenderingContext,m=b.FLOAT,p=b.UNSIGNED_BYTE,v=b.TRIANGLE_STRIP,y=[],T=0;T<Hi;T++)y.push([T]);var _=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_resolution","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth"];return o==="scaled"&&_.push("u_zoomSizeRatio"),l.floatsPerItem>0&&_.push("u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"),{VERTICES:Hi,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:_d,METHOD:v,UNIFORMS:_,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:m},{name:"a_edgeAttrIndex",size:1,type:m},{name:"a_baseFontSize",size:1,type:m},{name:"a_totalTextWidth",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_margin",size:1,type:m},{name:"a_padding",size:1,type:m},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_id",size:4,type:p,normalized:!0}],CONSTANT_ATTRIBUTES:[{name:"a_vertexIndex",size:1,type:m}],CONSTANT_DATA:y}}},{key:"processEdgeLabelBackground",value:function(b,m,p){var v=0;if(this.edgeAttributeTexture&&l.floatsPerItem>0){v=this.edgeAttributeTexture.allocate(m);var y=this.packedAttributeData;Jn(u,p.edgeAttributes,y,"",yd,0),this.edgeAttributeTexture.updateAllAttributes(m,y)}var T=this.floats,_=this.ints,S=b*this.STRIDE;T[S++]=p.edgeIndex,T[S++]=v,T[S++]=p.baseFontSize,T[S++]=p.totalTextWidth,T[S++]=p.positionMode,T[S++]=p.margin,T[S++]=p.padding,T[S++]=p.color,_[S++]=p.id}},{key:"setUniforms",value:function(b,m){var p=m.gl,v=m.uniformLocations;if(p.uniformMatrix3fv(v.u_matrix,!1,b.matrix),p.uniform1f(v.u_sizeRatio,b.sizeRatio),p.uniform1f(v.u_correctionRatio,b.correctionRatio),p.uniform1f(v.u_pixelRatio,b.pixelRatio),p.uniform1f(v.u_cameraAngle,b.cameraAngle),p.uniform2f(v.u_resolution,b.width,b.height),v.u_nodeDataTexture!==void 0&&p.uniform1i(v.u_nodeDataTexture,b.nodeDataTextureUnit),v.u_nodeDataTextureWidth!==void 0&&p.uniform1i(v.u_nodeDataTextureWidth,b.nodeDataTextureWidth),v.u_edgeDataTexture!==void 0&&p.uniform1i(v.u_edgeDataTexture,b.edgeDataTextureUnit),v.u_edgeDataTextureWidth!==void 0&&p.uniform1i(v.u_edgeDataTextureWidth,b.edgeDataTextureWidth),v.u_edgeFrameTexture!==void 0&&p.uniform1i(v.u_edgeFrameTexture,b.edgeFrameTextureUnit),v.u_edgeFrameTextureWidth!==void 0&&p.uniform1i(v.u_edgeFrameTextureWidth,b.edgeFrameTextureWidth),o==="scaled"&&v.u_zoomSizeRatio!==void 0){var y=this.renderer.getSetting("zoomToSizeRatioFunction");p.uniform1f(v.u_zoomSizeRatio,1/y(b.zoomRatio))}this.edgeAttributeTexture&&l.floatsPerItem>0&&v.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(Ye),p.uniform1i(v.u_edgeAttributeTexture,Ye),p.uniform1i(v.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),p.uniform1i(v.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()))}},{key:"renderProgram",value:function(b,m){this.edgeAttributeTexture&&l.floatsPerItem>0&&this.edgeAttributeTexture.upload(),ye(c,"renderProgram",this,3)([b,m])}},{key:"kill",value:function(){this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),ye(c,"kill",this,3)([])}},{key:"drawWebGL",value:function(b,m){var p=m.gl;this.totalCount!==0&&p.drawArraysInstanced(p.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(b){this.totalCount=b,b>this.bufferCapacity&&(this.bufferCapacity=Math.max(b,Math.ceil(this.bufferCapacity*1.5)||10),ye(c,"reallocate",this,3)([this.bufferCapacity]))}}])})(st);return new h(a,n,t)}function ho(a){var n,t,e,r,i,o;return{paths:a.paths,headLengthRatio:(n=a.headLengthRatio)!==null&&n!==void 0?n:0,tailLengthRatio:(t=a.tailLengthRatio)!==null&&t!==void 0?t:0,fontSizeMode:(e=a.fontSizeMode)!==null&&e!==void 0?e:"fixed",minVisibilityThreshold:(r=a.minVisibilityThreshold)!==null&&r!==void 0?r:.7,fullVisibilityThreshold:(i=a.fullVisibilityThreshold)!==null&&i!==void 0?i:.8,keepUpright:(o=a.keepUpright)!==null&&o!==void 0?o:!0}}var Sd=Ne.fontSize,Ed=17/64;function Ad(a){var n=a.paths,t=a.hasBorder,e=t===void 0?!1:t,r=a.fontSizeMode,i=r===void 0?"fixed":r,o=a.minVisibilityThreshold,s=o===void 0?.5:o,l=a.fullVisibilityThreshold,u=l===void 0?.6:l,d=a.keepUpright,h=d===void 0?!0:d,f=i==="scaled",c=n.some(function(v){return v.hasSharpCorners}),g=at(),b=Fe([].concat(J(n),[g])),m=rn(b),p=`#version 300 es

// ============================================================================
// Attributes - Per Character (Instanced)
// ============================================================================

// Edge geometry: indices for texture lookup
// Edge data (source/target node indices, thickness, head/tail ratios) is fetched from edge data texture
// Edge path attributes (curvature, etc.) are fetched from edge attribute texture
in float a_edgeIndex;       // Index into edge data texture
in float a_edgeAttrIndex;   // Index into edge attribute texture (for curvature, etc.)
in float a_baseFontSize;    // Base font size in pixels (per-label)

// Character metrics (in glyph units = atlas font size pixels)
in vec4 a_charMetrics;      // (charTextOffset, charAdvance, totalTextWidth, positionMode)
in vec4 a_charDims;         // (charSize.x, charSize.y, charOffset.x, charOffset.y)

// Atlas texture coordinates
in vec4 a_texCoords;        // (x, y, width, height) in atlas pixels

// Label parameters
in vec2 a_labelParams;      // (margin, unused)

// Appearance
in vec4 a_color;            // Character color (RGBA, normalized)
`.concat(e?"in vec4 a_borderColor;      // Border color (RGBA, normalized)":"",`

// ============================================================================
// Attributes - Per Vertex (Constant)
// ============================================================================

in vec2 a_quadCorner;       // Quad corner: (0,0), (1,0), (0,1), (1,1)

// ============================================================================
// Uniforms
// ============================================================================

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_correctionRatio;
uniform float u_pixelRatio;
uniform float u_cameraAngle;    // Required by node shape SDFs
// u_sdfBufferPixels kept for ABI compatibility but unused in shader
uniform float u_sdfBufferPixels;
uniform vec2 u_resolution;
uniform vec2 u_atlasSize;
uniform sampler2D u_nodeDataTexture; // Shared texture with node position/size/shape data
uniform int u_nodeDataTextureWidth;  // Width of 2D node data texture for coordinate calculation
uniform sampler2D u_edgeDataTexture; // Shared texture with edge data
uniform int u_edgeDataTextureWidth;  // Width of 2D edge data texture for coordinate calculation
uniform sampler2D u_edgeFrameTexture; // Per-edge clamp (tStart, tEnd, straightenFactor, pathLength)
uniform int u_edgeFrameTextureWidth;
`).concat(f?"uniform float u_zoomSizeRatio;  // Zoom-based size ratio from zoomToSizeRatioFunction":"",`

// Edge path attribute texture uniforms (for curvature and other path attributes)
`).concat(m.uniformDeclarations,`

// ============================================================================
// Varyings
// ============================================================================

out vec2 v_texCoord;
out vec4 v_color;
`).concat(e?"out vec4 v_borderColor;":"",`
out float v_edgeFade;  // 0 = fully visible, 1 = fully faded (outside body)
out float v_alphaModifier;  // 0-1 based on label visibility ratio
out float v_fontScale;  // Ratio of rendered font size to atlas font size
`).concat(e?"out float v_positionMode;  // Position mode for conditional border (0=over needs border)":"",`

// ============================================================================
// Constants
// ============================================================================

const float bias = 255.0 / 254.0;
const float FADE_WIDTH_PIXELS = 15.0;  // Width of fade gradient in pixels
const float ATLAS_FONT_SIZE = `).concat(me(Sd),`;  // Base font size used in SDF atlas
const float VERTICAL_CENTER_RATIO = `).concat(me(Ed),`;  // Baseline to visual center ratio

// ============================================================================
// Path Attribute Variables (set in main, used by path functions)
// ============================================================================
// Path attributes are fetched from the edge attribute texture and stored in
// variables with v_ prefix (e.g., v_curvature) for path functions to access.
`).concat(m.vertexVaryingDeclarations.replace(/out /g,""),`

// Node size variables (set in main, used by some path functions like loops).
// These mirror the v_sourceNodeSize / v_targetNodeSize varyings in generator.ts,
// but are plain floats here since the label shader is vertex-only.
float v_sourceNodeSize;
float v_targetNodeSize;

// Shared preamble: shape SDFs, path functions + selectors, clamp, helpers.
`).concat(lo({paths:n,minVisibilityThreshold:s,fullVisibilityThreshold:u}),`

// ============================================================================
// Main
// ============================================================================

void main() {
  // -------------------------------------------------------------------------
  // Fetch edge data from edge texture (2 texels per edge)
  // -------------------------------------------------------------------------
  // Texel 0: sourceNodeIndex, targetNodeIndex, thickness, reserved
  // Texel 1: headLengthRatio, tailLengthRatio, pathId, extremityIds
  int edgeIdx = int(a_edgeIndex);
  int texel0Idx = edgeIdx * 2;
  int texel1Idx = edgeIdx * 2 + 1;
  ivec2 edgeTexCoord0 = ivec2(texel0Idx % u_edgeDataTextureWidth, texel0Idx / u_edgeDataTextureWidth);
  ivec2 edgeTexCoord1 = ivec2(texel1Idx % u_edgeDataTextureWidth, texel1Idx / u_edgeDataTextureWidth);
  vec4 edgeData0 = texelFetch(u_edgeDataTexture, edgeTexCoord0, 0);
  vec4 edgeData1 = texelFetch(u_edgeDataTexture, edgeTexCoord1, 0);

  // Unpack edge data
  int srcIdx = int(edgeData0.x);
  int tgtIdx = int(edgeData0.y);
  float thickness = edgeData0.z;
  // edgeData0.w is reserved
  float headLengthRatio = edgeData1.x;
  float tailLengthRatio = edgeData1.y;
  int pathId = int(edgeData1.z);  // Path type for multi-path support
  float baseFontSize = a_baseFontSize;

  // -------------------------------------------------------------------------
  // Fetch path attributes from edge attribute texture
  // -------------------------------------------------------------------------
  // Note: The fetch code uses 'edgeIdx' variable, so we set it to the attribute texture index
  {
    int edgeIdx = int(a_edgeAttrIndex);  // Use attribute texture index for path attributes
`).concat(m.fetchCode,`
`).concat(m.varyingAssignments,`
  }

  // -------------------------------------------------------------------------
  // Fetch node data from node texture (geometry texel + rotation-flags texel)
  // -------------------------------------------------------------------------
  vec4 srcNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, srcIdx);
  vec4 tgtNodeData = readNodeData(u_nodeDataTexture, u_nodeDataTextureWidth, tgtIdx);

  vec2 source = srcNodeData.xy;
  vec2 target = tgtNodeData.xy;
  float sourceSize = srcNodeData.z;
  float targetSize = tgtNodeData.z;
  v_sourceNodeSize = sourceSize;
  v_targetNodeSize = targetSize;
  float charTextOffset = a_charMetrics.x;
  float charAdvance = a_charMetrics.y;
  float totalTextWidth = a_charMetrics.z;
  float positionMode = a_charMetrics.w;
  vec2 charSize = a_charDims.xy;
  vec2 charOffset = a_charDims.zw;
  float margin = a_labelParams.x;

  // -------------------------------------------------------------------------
  // Compute pixel-to-graph conversion (for fixed font size mode)
  // -------------------------------------------------------------------------
  // This converts screen pixels to graph units such that N pixels on screen
  // becomes N pixels regardless of zoom level.
  // matrixScaleX is how much the matrix scales graph units to clip space
  float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
  float pixelToGraph = 2.0 / (matrixScaleX * u_resolution.x);

  // -------------------------------------------------------------------------
  // Step 1: Convert thickness to WebGL units
  // -------------------------------------------------------------------------
  float webGLThickness = thickness * u_correctionRatio / u_sizeRatio;

  // -------------------------------------------------------------------------
  // Step 2: Compute body bounds (truncated at node boundaries + extremities)
  // -------------------------------------------------------------------------
  vec4 edgeClamp = readFrameTexel(u_edgeFrameTexture, u_edgeFrameTextureWidth, edgeIdx);
  vec3 bodyBounds = computeEdgeLabelBodyBounds(
    edgeClamp.x, edgeClamp.y, edgeClamp.w,
    webGLThickness, headLengthRatio, tailLengthRatio
  );
  float bodyStartDist = bodyBounds.x;
  float bodyEndDist = bodyBounds.y;
  float bodyLength = bodyBounds.z;

  // -------------------------------------------------------------------------
  // Step 3: Compute font scale and text dimensions
  // -------------------------------------------------------------------------
  // Font size modes:
  // - "fixed": Constant pixel size regardless of zoom, using pixelToGraph conversion
  // - "scaled": Scales with zoom using zoomToSizeRatioFunction
  `).concat(f?`// Scaled mode: font scales with zoom
  float fontScale = baseFontSize / ATLAS_FONT_SIZE * u_zoomSizeRatio;
  // Convert glyph-unit metrics to WebGL units (scales with zoom)
  float textWidthWebGL = totalTextWidth * fontScale * u_correctionRatio / u_sizeRatio;
  float charOffsetWebGL = charTextOffset * fontScale * u_correctionRatio / u_sizeRatio;
  float charAdvanceWebGL = charAdvance * fontScale * u_correctionRatio / u_sizeRatio;`:`// Fixed mode: font stays constant in screen pixels
  float fontScale = baseFontSize / ATLAS_FONT_SIZE;
  // Convert glyph-unit metrics to graph units using pixelToGraph (zoom-independent)
  float textWidthWebGL = totalTextWidth * fontScale * pixelToGraph;
  float charOffsetWebGL = charTextOffset * fontScale * pixelToGraph;
  float charAdvanceWebGL = charAdvance * fontScale * pixelToGraph;`,`

  // Font scale varying for fragment shader gamma scaling.
  // Ratio of rendered font size to atlas font size \u2014 used to tighten the
  // anti-aliasing band for large labels so they stay sharp.
  v_fontScale = fontScale;

  // -------------------------------------------------------------------------
  // Step 4: Alpha modifier from how much of the label fits in the body
  // -------------------------------------------------------------------------
  float alphaModifier = computeEdgeLabelAlpha(bodyLength, textWidthWebGL);

  // -------------------------------------------------------------------------
  // Step 5: Compute character center offset (truncation check moved to after curvature adjustment)
  // -------------------------------------------------------------------------
  // Body center in arc distance
  float bodyCenterDist = (bodyStartDist + bodyEndDist) * 0.5;

  float centerT = queryPathTAtDistance(pathId, bodyCenterDist, source, target);
  float screenDir = computeEdgeLabelScreenDir(pathId, centerT, source, target, u_matrix);

  // Reversed labels lay their characters out from target to source
  float readingSign = `).concat(h?"screenDir":"1.0",`;

  // Character center position relative to label center (on centerline, before curvature adjustment)
  float charCenterOffset = readingSign * (charOffsetWebGL + charAdvanceWebGL * 0.5 - textWidthWebGL * 0.5);

  // -------------------------------------------------------------------------
  // Step 6: Compute perpendicular offset based on position mode
  // -------------------------------------------------------------------------
  // Position modes: 0=over, 1=above, 2=below, 3=auto
  // Needed early for curvature-adaptive character spacing in Step 7
  float halfThickness = webGLThickness * 0.5;
  `).concat(f?`// Scaled mode: margin and text height scale with zoom (same factor as font)
  float marginWebGL = margin * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;
  float halfTextHeight = baseFontSize * 0.35 * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;`:`// Fixed mode: margin and text height stay constant in screen pixels
  float marginWebGL = margin * pixelToGraph;
  float halfTextHeight = baseFontSize * 0.35 * pixelToGraph;`,`
  float perpOffset = computeEdgeLabelPerpOffset(
    positionMode, halfThickness, marginWebGL, halfTextHeight, readingSign, screenDir
  );

  // -------------------------------------------------------------------------
  // Step 7: Position character on path using offset path traversal
  // -------------------------------------------------------------------------
  // For "over" mode (perpOffset = 0), use simple centerline placement
  // For above/below modes, walk along the offset path to find correct position
  float charT;

  if (perpOffset == 0.0) {
    // Simple case: place on centerline
    float charArcDist = bodyCenterDist + charCenterOffset;
    charT = queryPathTAtDistance(pathId, charArcDist, source, target);
  } else {
    // Offset path traversal: walk along the offset curve to find character position
    // This ensures even character spacing regardless of curvature

    `).concat(c?`// -----------------------------------------------------------------------
    // Corner skip setup for step/taxi edges with above/below labels
    // -----------------------------------------------------------------------
    // At concave corners (inner side of the bend), characters would bunch up
    // because the offset path has near-zero arc length. We detect corner
    // crossings during the offset path traversal and add skip distance.

    // Get corner t values and concavity
    vec2 cornerTs = queryGetCornerTs(pathId, source, target);
    vec2 concavity = queryGetCornerConcavity(pathId, source, target, perpOffset);

    // Skip distance in graph units, proportional to on-screen font size
    // For fixed mode: use pixelToGraph so gap stays constant regardless of zoom
    // For scaled mode: use the same conversion as text width
    `.concat(f?"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;":"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * pixelToGraph;",`

    // Corner t values for detecting crossings during traversal
    float corner1T = cornerTs.x;
    float corner2T = cornerTs.y;
    bool corner1IsConcave = concavity.x > 0.5;
    bool corner2IsConcave = concavity.y > 0.5;`):"",`

    // Target distance along offset path from center
    float targetOffsetDist = abs(charCenterOffset);

    // Handle center character (charCenterOffset \u2248 0) - no search needed
    if (targetOffsetDist < 0.0001) {
      charT = centerT;
    } else {
      vec2 centerPos = queryPathPosition(pathId, centerT, source, target);
      vec2 centerNormal = queryPathNormal(pathId, centerT, source, target);
      vec2 offsetCenter = centerPos + centerNormal * perpOffset;

      float searchDir = charCenterOffset > 0.0 ? 1.0 : -1.0;

      // Search bounds (t values for body start and end)
      float tBodyStart = queryPathTAtDistance(pathId, bodyStartDist, source, target);
      float tBodyEnd = queryPathTAtDistance(pathId, bodyEndDist, source, target);

      // Walk along offset path to find character position
      float accumDist = 0.0;
      vec2 prevOffsetPos = offsetCenter;
      float prevT = centerT;
      float foundT = centerT;

      // Search range depends on direction
      float tSearchEnd = searchDir > 0.0 ? tBodyEnd : tBodyStart;

      `).concat(c?`// Track which concave corners we've crossed to add skip distance
      bool crossedCorner1 = false;
      bool crossedCorner2 = false;
      float effectiveTargetDist = targetOffsetDist;`:"",`

      const int STEPS = 32;
      for (int i = 1; i <= STEPS; i++) {
        // Step along centerline t, from center toward target
        float stepT = centerT + searchDir * float(i) * abs(tSearchEnd - centerT) / float(STEPS);

        `).concat(c?`// Check for concave corner crossings and add skip distance
        // Corner 1 crossing check
        if (corner1IsConcave && !crossedCorner1) {
          bool crossingCorner1 = (searchDir > 0.0)
            ? (prevT < corner1T && stepT >= corner1T)
            : (prevT > corner1T && stepT <= corner1T);
          if (crossingCorner1) {
            crossedCorner1 = true;
            effectiveTargetDist += skipDistGraph;
          }
        }

        // Corner 2 crossing check
        if (corner2IsConcave && !crossedCorner2) {
          bool crossingCorner2 = (searchDir > 0.0)
            ? (prevT < corner2T && stepT >= corner2T)
            : (prevT > corner2T && stepT <= corner2T);
          if (crossingCorner2) {
            crossedCorner2 = true;
            effectiveTargetDist += skipDistGraph;
          }
        }`:"",`

        // Compute offset position at this t
        vec2 stepPos = queryPathPosition(pathId, stepT, source, target);
        vec2 stepNormal = queryPathNormal(pathId, stepT, source, target);
        vec2 offsetPos = stepPos + stepNormal * perpOffset;

        // Distance along offset path
        float segDist = length(offsetPos - prevOffsetPos);

        `).concat(c?`if (accumDist + segDist >= effectiveTargetDist) {
          // Interpolate within segment to find exact t
          float remaining = effectiveTargetDist - accumDist;
          float segT = remaining / max(segDist, 0.0001);
          foundT = mix(prevT, stepT, segT);
          break;
        }`:`if (accumDist + segDist >= targetOffsetDist) {
          // Interpolate within segment to find exact t
          float remaining = targetOffsetDist - accumDist;
          float segT = remaining / max(segDist, 0.0001);
          foundT = mix(prevT, stepT, segT);
          break;
        }`,`

        accumDist += segDist;
        prevOffsetPos = offsetPos;
        prevT = stepT;
        // Update foundT to last valid position in case loop exhausts without finding target
        foundT = stepT;
      }

      charT = foundT;
    }
  }

  // Get position and tangent at final character position
  vec2 pathPos = queryPathPosition(pathId, charT, source, target);
  vec2 tangent = queryPathTangent(pathId, charT, source, target);

  // Compute perpendicular direction (90 degrees from tangent)
  vec2 perpDir = vec2(-tangent.y, tangent.x);

  // Apply perpendicular offset to path position
  vec2 offsetPathPos = pathPos + perpDir * perpOffset;

  // -------------------------------------------------------------------------
  // Step 8: Build character quad
  // -------------------------------------------------------------------------
  // Character size in screen pixels
  vec2 charSizePixels = charSize * fontScale;

  // Character offset from origin to the atlas region's top-left corner.
  // bearingX/bearingY already include the SDF buffer.
  vec2 charOffsetPixels = charOffset * fontScale;

  // The character's local X offset from pathPos (which is at character center)
  float charLocalX = -charAdvance * 0.5 * fontScale;

  // Build quad position:
  // - Start at character origin (charLocalX on X axis, 0 on Y axis = baseline)
  // - Add bearing offset to get to atlas region corner
  // - Add quad corner * size to get vertex position
  vec2 quadPos;
  quadPos.x = charLocalX + charOffsetPixels.x + a_quadCorner.x * charSizePixels.x;
  // charOffset.y = -bearingY (negated), so -charOffsetPixels.y = bearingY * fontScale
  // (distance from baseline to atlas region top, positive = upward)
  // Quad corner (0,0) = bottom-left, (1,1) = top-right
  quadPos.y = -charOffsetPixels.y - charSizePixels.y * (1.0 - a_quadCorner.y);

  // Center vertically on the path by offsetting by half the visual text height
  // VERTICAL_CENTER_RATIO is the distance from baseline to visual center as a ratio of atlas font size
  float verticalCenterOffset = VERTICAL_CENTER_RATIO * ATLAS_FONT_SIZE * fontScale;
  quadPos.y -= verticalCenterOffset;

  // -------------------------------------------------------------------------
  // Step 9: Rotate quad to align with tangent
  // -------------------------------------------------------------------------
  // Rotation matrix from the reading direction
  // readingDir = (cos(angle), sin(angle)), so we can build rotation directly
  vec2 readingDir = tangent * readingSign;
  mat2 rotation = mat2(readingDir.x, readingDir.y, -readingDir.y, readingDir.x);

  // Convert pixel offset to WebGL units for rotation
  `).concat(f?"vec2 quadPosWebGL = quadPos * u_correctionRatio / u_sizeRatio; // Scaled mode":"vec2 quadPosWebGL = quadPos * pixelToGraph; // Fixed mode: use pixelToGraph for zoom-independent size",`

  // Rotate around character center on path
  vec2 rotatedOffset = rotation * quadPosWebGL;

  // Final position in graph space (using offset path position for above/below modes)
  vec2 worldPos = offsetPathPos + rotatedOffset;

  // -------------------------------------------------------------------------
  // Step 10: Transform to clip space
  // -------------------------------------------------------------------------
  vec3 clipPos = u_matrix * vec3(worldPos, 1.0);
  gl_Position = vec4(clipPos.xy, 0.0, 1.0);

  // -------------------------------------------------------------------------
  // Step 11: Texture coordinates
  // -------------------------------------------------------------------------
  // Flip Y for texture coordinates (texture Y goes down, quad Y goes up)
  vec2 texCorner = vec2(a_quadCorner.x, 1.0 - a_quadCorner.y);
  v_texCoord = (a_texCoords.xy + texCorner * a_texCoords.zw) / u_atlasSize;

  // -------------------------------------------------------------------------
  // Step 12: Pass color, border color, and alpha modifier
  // -------------------------------------------------------------------------
  v_color = a_color;
  v_color.a *= bias;
`).concat(e?`  v_borderColor = a_borderColor;
  v_borderColor.a *= bias;
  v_positionMode = positionMode;`:"",`
  v_alphaModifier = alphaModifier;

  // -------------------------------------------------------------------------
  // Step 13: Compute edge fade for soft truncation
  // -------------------------------------------------------------------------
  // Convert fade width from pixels to WebGL units
  `).concat(f?"float fadeWidthWebGL = FADE_WIDTH_PIXELS * u_correctionRatio / u_sizeRatio;":"float fadeWidthWebGL = FADE_WIDTH_PIXELS * pixelToGraph;",`

  // Compute the arc position of THIS VERTEX (not just character center)
  // The quad extends from charCenter - advance/2 to charCenter + advance/2
  // a_quadCorner.x is 0 for left edge, 1 for right edge
  float vertexLocalOffset = readingSign * (a_quadCorner.x - 0.5) * charAdvanceWebGL;
  float vertexArcOffset = charCenterOffset + vertexLocalOffset;

  // Compute distance from body edges (positive = inside body, negative = outside)
  float halfBody = bodyLength * 0.5;
  float distFromStart = vertexArcOffset + halfBody;  // Distance from body start edge
  float distFromEnd = halfBody - vertexArcOffset;    // Distance from body end edge
  float distFromEdge = min(distFromStart, distFromEnd);

  // Compute fade: 0 = fully visible (deep inside body), 1 = fully faded (at body edge)
  // Fade goes from 0 (at 2*fadeWidth inside) to 1 (at body edge)
  // This ensures text is fully transparent before reaching extremities
  v_edgeFade = 1.0 - smoothstep(0.0, fadeWidthWebGL * 2.0, distFromEdge);
}
`);return p}function Dd(){var a=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},n=a.hasBorder,t=n===void 0?!1:n,e=`#version 300 es
precision highp float;

in vec2 v_texCoord;
in vec4 v_color;
`.concat(t?"in vec4 v_borderColor;":"",`
in float v_edgeFade;  // 0 = fully visible, 1 = fully faded
in float v_alphaModifier;  // 0-1 based on label visibility ratio
in float v_fontScale;  // Ratio of rendered font size to atlas font size
`).concat(t?"in float v_positionMode;  // Position mode (0=over, 1=above, 2=below, 3=auto)":"",`

uniform sampler2D u_atlas;
uniform float u_gamma;
uniform float u_sdfBuffer;
uniform float u_pixelRatio;
`).concat(t?"uniform float u_borderWidth;  // Border width in SDF units (normalized)":"",`

// Fragment output (single target - picking handled via separate pass)
out vec4 fragColor;

void main() {
  #ifdef PICKING_MODE
    // Edge labels are not pickable - discard all fragments in picking mode
    fragColor = vec4(0.0);
    discard;
  #else
  // SDF stores normalized distance: 0.5 = on edge, >0.5 = inside glyph
  float sdfValue = texture(u_atlas, v_texCoord).a;

  // Edge threshold accounting for SDF buffer padding
  float edge = 1.0 - u_sdfBuffer;

  // Scale gamma inversely with font scale so small labels get a wider AA band
  // (smoother) and large labels get a tighter band (sharper).
  float aaWidth = u_gamma / (u_pixelRatio * v_fontScale);

  // Apply edge fade for soft truncation at body boundaries
  // Also apply visibility-based alpha modifier for short edge labels
  float edgeAlpha = (1.0 - v_edgeFade) * v_alphaModifier;

`).concat(t?`  // Fill alpha: fully opaque inside the glyph
  float fillAlpha = smoothstep(edge - aaWidth, edge + aaWidth, sdfValue);

  // Only apply border for "over" position mode (v_positionMode == 0.0)
  // Labels positioned above/below/auto don't overlap the edge line and don't need borders
  if (v_positionMode < 0.5) {
    // Border rendering: compute alpha for both fill and border regions
    // Border extends from (edge - borderWidth) to edge
    float borderEdge = edge - u_borderWidth;

    // Border alpha: opaque in the border region (between borderEdge and edge)
    float borderAlpha = smoothstep(borderEdge - aaWidth, borderEdge + aaWidth, sdfValue);

    // Composite: fill on top of border
    // Border is visible where borderAlpha > 0 but fillAlpha < 1
    vec3 borderColorPremult = v_borderColor.rgb * v_borderColor.a * borderAlpha * edgeAlpha;
    vec3 fillColorPremult = v_color.rgb * v_color.a * fillAlpha * edgeAlpha;

    // Blend fill over border (fill replaces border where fill is opaque)
    float finalBorderAlpha = borderAlpha * (1.0 - fillAlpha);
    vec3 finalColor = fillColorPremult + v_borderColor.rgb * v_borderColor.a * finalBorderAlpha * edgeAlpha;
    float finalAlpha = (v_color.a * fillAlpha + v_borderColor.a * finalBorderAlpha) * edgeAlpha;

    fragColor = vec4(finalColor, finalAlpha);
  } else {
    // No border for above/below/auto positions - simple text rendering
    float finalAlpha = v_color.a * fillAlpha * edgeAlpha;
    fragColor = vec4(v_color.rgb * finalAlpha, finalAlpha);
  }`:`  // Smooth transition from transparent to opaque at glyph edge
  float alpha = smoothstep(edge - aaWidth, edge + aaWidth, sdfValue);

  // Premultiplied alpha output
  float finalAlpha = v_color.a * alpha * edgeAlpha;
  fragColor = vec4(v_color.rgb * finalAlpha, finalAlpha);`,`
  #endif
}
`);return e}function wd(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,t=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"fixed",e=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_sdfBufferPixels","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"];t==="scaled"&&e.push("u_zoomSizeRatio"),n&&e.push("u_borderWidth");var r=U(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=U(o.uniforms),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;e.includes(u.name)||e.push(u.name)}}catch(d){s.e(d)}finally{s.f()}}}catch(d){r.e(d)}finally{r.f()}return e}function Rd(a){var n=a.hasBorder,t=n===void 0?!1:n,e=a.fontSizeMode,r=e===void 0?"fixed":e;return{vertexShader:Ad(a),fragmentShader:Dd({hasBorder:t}),uniforms:wd(a.paths,t,r)}}var Ld=new Map;function Cd(a){switch(a){case"over":return 0;case"above":return 1;case"below":return 2;case"auto":return 3;default:return 0}}function kd(a,n,t,e){var r=e.color,i=e.margin,o=e.textBorder,s=ho(e),l=s.paths,u=s.fontSizeMode,d=s.minVisibilityThreshold,h=s.fullVisibilityThreshold,f=s.keepUpright,c=!!o,g=Rd({paths:l,hasBorder:c,fontSizeMode:u,minVisibilityThreshold:d,fullVisibilityThreshold:h,keepUpright:f}),b=(function(m){function p(v,y,T){var _;ne(this,p),_=fe(this,p,[v,y,T]),E(_,"atlasTexture",null),E(_,"atlasNeedsUpdate",!1),E(_,"refreshTimeout",null),E(_,"labelGlyphCache",new Map),E(_,"edgeAttributeTexture",null);var S=at();if(_.attributeLayout=Fe([].concat(J(l),[S])),_.attrDescriptors=Qn([].concat(J(l),[S]),_.attributeLayout),_.edgeAttributeTexture=new Zn(v,_.attributeLayout),_.packedAttributeData=new Float32Array(_.attributeLayout.floatsPerItem),_.atlasManager=new it,_.gamma=.025,_.sdfBuffer=Ne.cutoff,_.atlasTexture=v.createTexture(),!_.atlasTexture)throw new Error("EdgeLabelProgram: failed to create atlas texture");v.bindTexture(v.TEXTURE_2D,_.atlasTexture),v.texParameteri(v.TEXTURE_2D,v.TEXTURE_WRAP_S,v.CLAMP_TO_EDGE),v.texParameteri(v.TEXTURE_2D,v.TEXTURE_WRAP_T,v.CLAMP_TO_EDGE),v.texParameteri(v.TEXTURE_2D,v.TEXTURE_MIN_FILTER,v.LINEAR),v.texParameteri(v.TEXTURE_2D,v.TEXTURE_MAG_FILTER,v.LINEAR),v.bindTexture(v.TEXTURE_2D,null),_.atlasManager.on(it.ATLAS_UPDATED_EVENT,function(){_.atlasNeedsUpdate=!0,_.refreshTimeout!==null&&clearTimeout(_.refreshTimeout),_.refreshTimeout=setTimeout(function(){_.refreshTimeout=null,_.renderer.refresh()},0)});var D={family:"sans-serif",weight:"normal",style:"normal"};return _.defaultFontKey=_.atlasManager.registerFont(D),_}return ge(p,m),ae(p,[{key:"getDefinition",value:function(){var y=WebGL2RenderingContext,T=y.FLOAT,_=y.UNSIGNED_BYTE,S=y.TRIANGLE_STRIP,D=new Set,w=[],k=U(l),L;try{for(k.s();!(L=k.n()).done;){var N=L.value,R=U(N.attributes),z;try{for(R.s();!(z=R.n()).done;){var C=z.value,I=C.name.startsWith("a_")?C.name:"a_".concat(C.name);D.has(I)||(D.add(I),w.push({name:I,size:C.size,type:C.type}))}}catch(B){R.e(B)}finally{R.f()}}}catch(B){k.e(B)}finally{k.f()}return{VERTICES:4,VERTEX_SHADER_SOURCE:g.vertexShader,FRAGMENT_SHADER_SOURCE:g.fragmentShader,METHOD:S,UNIFORMS:g.uniforms,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:T},{name:"a_edgeAttrIndex",size:1,type:T},{name:"a_baseFontSize",size:1,type:T},{name:"a_charMetrics",size:4,type:T},{name:"a_charDims",size:4,type:T},{name:"a_texCoords",size:4,type:T},{name:"a_labelParams",size:2,type:T},{name:"a_color",size:4,type:_,normalized:!0}].concat(J(c?[{name:"a_borderColor",size:4,type:_,normalized:!0}]:[]),J(w.filter(function(B){return!["a_curvature","curvature"].includes(B.name)}))),CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:T}],CONSTANT_DATA:[[0,0],[1,0],[0,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(y,T){if(T.hidden||!T.text){this.labelGlyphCache.delete(y);return}var _=T.text,S=T.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(_,S);var D=[],w=[],k=0,L=U(_),N;try{for(L.s();!(N=L.n()).done;){var R=N.value,z=R.codePointAt(0);if(z===void 0){D.push(void 0),w.push(k);continue}var C=this.atlasManager.getGlyph(z,S);D.push(C),w.push(k),C&&(k+=C.advance)}}catch(I){L.e(I)}finally{L.f()}this.labelGlyphCache.set(y,{glyphs:D,xOffsets:w,totalWidth:k})}},{key:"processCharacter",value:function(y,T,_,S){var D,w,k,L=this.floats,N=this.STRIDE,R=y*N,z=this.labelGlyphCache.get(T.parentKey);if(!z||!z.glyphs[S]){for(var C=0;C<N;C++)L[R+C]=0;return}var I=z.glyphs[S],B=z.xOffsets[S],F=p.labelColor,x=F&&ie(F)==="object"&&"color"in F&&F.color?F.color:T.color,P=ke(x),W=R;L[W++]=T.edgeIndex,L[W++]=(D=(w=this.edgeAttributeTexture)===null||w===void 0?void 0:w.getIndex(T.parentKey))!==null&&D!==void 0?D:0,L[W++]=T.size,L[W++]=B,L[W++]=I.advance,L[W++]=z.totalWidth,L[W++]=Cd(T.position),L[W++]=I.atlasWidth,L[W++]=I.atlasHeight,L[W++]=I.bearingX,L[W++]=-I.bearingY,L[W++]=I.atlasX,L[W++]=I.atlasY,L[W++]=I.atlasWidth,L[W++]=I.atlasHeight;var G=(k=p.labelMargin)!==null&&k!==void 0?k:T.margin;if(L[W++]=G,L[W++]=0,L[W++]=P,c&&o){var A;typeof o.color=="string"?A=o.color:A=o.color.color||"#ffffff",L[W++]=ke(A)}var q=new Set,V=U(l),Y;try{for(V.s();!(Y=V.n()).done;){var pe=Y.value,se=U(pe.attributes),Z;try{for(se.s();!(Z=se.n()).done;){var ue=Z.value,ve=ue.name.startsWith("a_")?ue.name.slice(2):ue.name;if(ve!=="curvature"&&!q.has(ve)){q.add(ve);for(var De=0;De<ue.size;De++)L[W++]=0}}}catch(Te){se.e(Te)}finally{se.f()}}}catch(Te){V.e(Te)}finally{V.f()}}},{key:"processEdgeLabel",value:function(y,T,_){if(this.prepareLabelGlyphs(y,_),this.edgeAttributeTexture&&!_.hidden&&_.text){this.edgeAttributeTexture.allocate(y);var S=this.packedAttributeData;Jn(this.attrDescriptors,_.edgeAttributes,S,"",Ld,0),this.edgeAttributeTexture.updateAllAttributes(y,S)}return ye(p,"processLabel",this,3)([y,T,_])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var y=this.normalProgram.gl,T=this.atlasManager.getTextures();if(T.length!==0){var _=T[0];y.bindTexture(y.TEXTURE_2D,this.atlasTexture),y.texImage2D(y.TEXTURE_2D,0,y.RGBA,_.width,_.height,0,y.RGBA,y.UNSIGNED_BYTE,_.data),y.bindTexture(y.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(y,T){var _=T.gl,S=T.uniformLocations,D=this.atlasManager.getTextures(),w=D.length>0?[D[0].width,D[0].height]:[1,1];if(_.uniformMatrix3fv(S.u_matrix,!1,y.matrix),_.uniform1f(S.u_sizeRatio,y.sizeRatio),_.uniform1f(S.u_correctionRatio,y.correctionRatio),_.uniform1f(S.u_pixelRatio,y.pixelRatio),_.uniform1f(S.u_cameraAngle,y.cameraAngle),_.uniform1f(S.u_sdfBufferPixels,Ne.buffer),_.uniform2f(S.u_resolution,y.width,y.height),_.uniform2f(S.u_atlasSize,w[0],w[1]),_.uniform1f(S.u_gamma,this.gamma),_.uniform1f(S.u_sdfBuffer,this.sdfBuffer),_.activeTexture(_.TEXTURE0),_.bindTexture(_.TEXTURE_2D,this.atlasTexture),_.uniform1i(S.u_atlas,0),S.u_nodeDataTexture!==void 0&&_.uniform1i(S.u_nodeDataTexture,y.nodeDataTextureUnit),S.u_nodeDataTextureWidth!==void 0&&_.uniform1i(S.u_nodeDataTextureWidth,y.nodeDataTextureWidth),S.u_edgeDataTexture!==void 0&&_.uniform1i(S.u_edgeDataTexture,y.edgeDataTextureUnit),S.u_edgeDataTextureWidth!==void 0&&_.uniform1i(S.u_edgeDataTextureWidth,y.edgeDataTextureWidth),S.u_edgeFrameTexture!==void 0&&_.uniform1i(S.u_edgeFrameTexture,y.edgeFrameTextureUnit),S.u_edgeFrameTextureWidth!==void 0&&_.uniform1i(S.u_edgeFrameTextureWidth,y.edgeFrameTextureWidth),c&&o&&S.u_borderWidth!==void 0){var k=o.width/Ne.buffer*this.sdfBuffer;_.uniform1f(S.u_borderWidth,k)}if(this.edgeAttributeTexture&&S.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(Ye),_.uniform1i(S.u_edgeAttributeTexture,Ye),_.uniform1i(S.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),_.uniform1i(S.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem())),u==="scaled"&&S.u_zoomSizeRatio!==void 0){var L=this.renderer.getSetting("zoomToSizeRatioFunction"),N=1/L(y.zoomRatio);_.uniform1f(S.u_zoomSizeRatio,N)}}},{key:"renderProgram",value:function(y,T){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),this.edgeAttributeTexture&&this.edgeAttributeTexture.upload(),ye(p,"renderProgram",this,3)([y,T])}},{key:"registerFont",value:function(y){var T=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",_=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:y,weight:T,style:_})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"ensureGlyphsReady",value:function(y,T){var _=T||this.defaultFontKey,S=U(y),D;try{for(S.s();!(D=S.n()).done;){var w=D.value;this.atlasManager.ensureGlyphs(w,_)}}catch(k){S.e(k)}finally{S.f()}this.atlasManager.flush()}},{key:"measureLabelAtlasWidth",value:function(y,T){var _=T||this.defaultFontKey;this.atlasManager.ensureGlyphs(y,_),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var S=0,D=U(y),w;try{for(D.s();!(w=D.n()).done;){var k=w.value,L=k.codePointAt(0);if(L!==void 0){var N=this.atlasManager.getGlyph(L,_);N&&(S+=N.advance)}}}catch(R){D.e(R)}finally{D.f()}return S}},{key:"measureLabel",value:function(y,T,_){var S=this.measureLabelAtlasWidth(y,_),D=T/Ne.fontSize;return{width:S*D,height:T,textHeight:T}}},{key:"kill",value:function(){var y=this.normalProgram.gl;this.refreshTimeout!==null&&(clearTimeout(this.refreshTimeout),this.refreshTimeout=null),this.atlasTexture&&(y.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),ye(p,"kill",this,3)([])}}])})(to);return E(b,"labelColor",r),E(b,"labelMargin",i),new b(a,n,t)}function Pd(){var a=`
// No extremity - always returns positive (outside)
float extremity_none(vec2 uv, float lengthRatio, float widthRatio) {
  return 1.0;
}
`;return{name:"none",glsl:a,length:0,widthFactor:1,margin:0,uniforms:[],attributes:[]}}function co(a,n,t,e,r,i,o){var s,l,u=oo(e),d=u.paths,h=u.layers,f=u.defaultHead,c=u.defaultTail,g=[Pd()].concat(J(u.extremities)),b={},m={};d.forEach(function(R,z){return b[R.name]=z}),g.forEach(function(R,z){return m[R.name]=z});var p=(s=m[f])!==null&&s!==void 0?s:0,v=(l=m[c])!==null&&l!==void 0?l:0,y=null,T=Fe([].concat(J(d),J(h))),_=(function(R){function z(C,I,B){var F;ne(this,z),y||(y=Oi({paths:d,extremities:g,layers:h,antialias:r})),F=fe(this,z,[C,I,B]),E(F,"layerLifecycles",new Map),E(F,"needsShaderRegeneration",!1),E(F,"edgeAttributeTexture",null),E(F,"layout",T),E(F,"attrDescriptors",[]),E(F,"lifecycleIndexOffset",d.length),F._pickingBuffer=I,F.edgeAttributeTexture=new Zn(C,F.layout),F.packedAttributeData=new Float32Array(F.layout.floatsPerItem),h.forEach(function(P,W){if(P.lifecycle){var G={gl:C,renderer:{refresh:function(){return B.refresh()}},getUniformLocation:function(q){return C.getUniformLocation(F.normalProgram.program,q)},requestShaderRegeneration:function(){F.needsShaderRegeneration=!0},requestRefresh:function(){B.refresh()}};F.layerLifecycles.set(W,P.lifecycle(G))}}),F.layerLifecycles.forEach(function(P){var W;return(W=P.init)===null||W===void 0?void 0:W.call(P)});var x=new Map;return F.layerLifecycles.forEach(function(P,W){P.getAttributeData&&x.set(d.length+W,P)}),F.attrDescriptors=Qn([].concat(J(d),J(h)),F.layout,x),F}return ge(z,R),ae(z,[{key:"getAttributeTexture",value:function(){return this.edgeAttributeTexture}},{key:"resolveEdgeIds",value:function(I,B,F){var x=I,P=0,W=p,G=v,A=B?x.selfLoopPath:F&&x.parallelPath?x.parallelPath:x.path;A&&b[A]!==void 0&&(P=b[A]),x.head&&x.head!=="none"&&m[x.head]!==void 0&&(W=m[x.head]),x.tail&&x.tail!=="none"&&m[x.tail]!==void 0&&(G=m[x.tail]);var q=g[W],V=g[G];return{pathId:P,headId:W,tailId:G,headLengthRatio:Ue(q.length)?0:q.length,tailLengthRatio:Ue(V.length)?0:V.length}}},{key:"getDefinition",value:function(){var I=WebGL2RenderingContext,B=I.TRIANGLE_STRIP,F=B,x=y;return{VERTICES:x.verticesPerEdge,VERTEX_SHADER_SOURCE:x.vertexShader,FRAGMENT_SHADER_SOURCE:x.fragmentShader,METHOD:F,UNIFORMS:x.uniforms,ATTRIBUTES:x.attributes,CONSTANT_ATTRIBUTES:x.constantAttributes,CONSTANT_DATA:x.constantData}}},{key:"maybeRegenerateShaders",value:function(){var I=this;if(this.needsShaderRegeneration){this.needsShaderRegeneration=!1;var B=h.map(function(q,V){var Y=I.layerLifecycles.get(V);return Y!=null&&Y.regenerate?Y.regenerate():q});y=Oi({paths:d,extremities:g,layers:B,antialias:this.renderer.getSetting("antialiasEdges")});var F=this.normalProgram.gl,x=this.normalProgram,P=x.program,W=x.buffer,G=x.vertexShader,A=x.fragmentShader;F.deleteProgram(P),F.deleteBuffer(W),F.deleteShader(G),F.deleteShader(A),this.normalProgram=this.getProgramInfo("normal",F,y.vertexShader,y.fragmentShader,this._pickingBuffer)}}},{key:"process",value:function(I,B,F,x,P,W){var G=B*this.STRIDE;if(P.visibility==="hidden"||F.visibility==="hidden"||x.visibility==="hidden"){for(var A=G+this.STRIDE;G<A;G++)this.floats[G]=0;this.floats[B*this.STRIDE]=Qi;return}this.processVisibleItem(St(I),G,F,x,P,W)}},{key:"processVisibleItem",value:function(I,B,F,x,P,W){var G,A=this.floats,q=this.ints;A[B++]=W,A[B++]=ke(P.color),q[B++]=I,A[B++]=(G=P.opacity)!==null&&G!==void 0?G:1;var V=this.packedAttributeData;Jn(this.attrDescriptors,P,V,P.color,this.layerLifecycles,this.lifecycleIndexOffset),this.edgeAttributeTexture.updateAllAttributesAtRow(W,V)}},{key:"setUniforms",value:function(I,B){var F=this,x=B.gl,P=B.uniformLocations;P.u_matrix&&x.uniformMatrix3fv(P.u_matrix,!1,I.matrix),P.u_sizeRatio&&x.uniform1f(P.u_sizeRatio,I.sizeRatio),P.u_correctionRatio&&x.uniform1f(P.u_correctionRatio,I.correctionRatio),P.u_zoomRatio&&x.uniform1f(P.u_zoomRatio,I.zoomRatio),P.u_pixelRatio&&x.uniform1f(P.u_pixelRatio,I.pixelRatio),P.u_cameraAngle&&x.uniform1f(P.u_cameraAngle,I.cameraAngle),P.u_feather&&x.uniform1f(P.u_feather,I.antiAliasingFeather),P.u_minEdgeThickness&&x.uniform1f(P.u_minEdgeThickness,I.minEdgeThickness),P.u_pickingPadding&&x.uniform1f(P.u_pickingPadding,I.edgePickingPadding),P.u_nodeDataTexture&&x.uniform1i(P.u_nodeDataTexture,I.nodeDataTextureUnit),P.u_nodeDataTextureWidth&&x.uniform1i(P.u_nodeDataTextureWidth,I.nodeDataTextureWidth),P.u_edgeDataTexture&&x.uniform1i(P.u_edgeDataTexture,I.edgeDataTextureUnit),P.u_edgeDataTextureWidth&&x.uniform1i(P.u_edgeDataTextureWidth,I.edgeDataTextureWidth),P.u_edgeFrameTexture&&x.uniform1i(P.u_edgeFrameTexture,I.edgeFrameTextureUnit),P.u_edgeFrameTextureWidth&&x.uniform1i(P.u_edgeFrameTextureWidth,I.edgeFrameTextureWidth),this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&(this.edgeAttributeTexture.bind(Ye),P.u_edgeAttributeTexture&&x.uniform1i(P.u_edgeAttributeTexture,Ye),P.u_edgeAttributeTextureWidth&&x.uniform1i(P.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),P.u_edgeAttributeTexelsPerEdge&&x.uniform1i(P.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()));var W=new Set;d.forEach(function(G){G.uniforms.forEach(function(A){W.has(A.name)||(W.add(A.name),F.setTypedUniform(A,B))})}),g.forEach(function(G){G.uniforms.forEach(function(A){W.has(A.name)||(W.add(A.name),F.setTypedUniform(A,B))})}),h.forEach(function(G){G.uniforms.forEach(function(A){W.has(A.name)||(W.add(A.name),F.setTypedUniform(A,B))})})}},{key:"render",value:function(I,B,F){this.maybeRegenerateShaders(),ye(z,"render",this,3)([I,B,F])}},{key:"renderProgram",value:function(I,B){this.layerLifecycles.forEach(function(F){var x;return(x=F.beforeRender)===null||x===void 0?void 0:x.call(F)}),ye(z,"renderProgram",this,3)([I,B])}},{key:"uploadAttributeTexture",value:function(){this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&this.edgeAttributeTexture.upload()}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(I){var B;return(B=I.kill)===null||B===void 0?void 0:B.call(I)}),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),ye(z,"kill",this,3)([])}}])})(st),S=g[p],D=g[v],w=H({paths:d,headLengthRatio:Ue(S.length)?0:S.length,tailLengthRatio:Ue(D.length)?0:D.length},e.label),k=ho(w),L=function(){return{labelProgram:kd(a,null,t,w),labelBackgroundProgram:Td(a,n,t,{shaderConfig:k})}},N=new fd(a,{paths:d,extremities:g,layers:h,nodeShapes:i,nodeLayers:o});return{edgeProgram:new _(a,null,t),createLabelPrograms:L,framePass:N}}function Fd(a){return ie(a)==="object"&&"uniforms"in a&&Array.isArray(a.uniforms)}function Id(a){return ie(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function Gd(a){return ie(a)==="object"&&"uniforms"in a&&"attributes"in a&&"segments"in a}function Nd(a){return ie(a)==="object"&&"uniforms"in a&&"attributes"in a&&"length"in a}function zd(a){return ie(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function Md(a){return ie(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function Bd(a){return ie(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function Wd(a){return ie(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function Ud(a){if(Fd(a))return a;if(Md(a))return{name:a.name,glsl:a.glsl,inradiusFactor:a.inradiusFactor,uniforms:[]};throw new Error("Invalid node shape specification: ".concat(JSON.stringify(a)))}function Od(a){if(Id(a))return a;if(vi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid node layer specification: ".concat(JSON.stringify(a)))}function Hd(a){if(Gd(a))return a;if(Bd(a))return{name:a.name,glsl:a.glsl,segments:a.segments,uniforms:[],attributes:[]};throw new Error("Invalid edge path specification: ".concat(JSON.stringify(a)))}function $d(a){if(zd(a))return a;if(mi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid edge layer specification: ".concat(JSON.stringify(a)))}function Vd(a){if(Nd(a))return a;if(Wd(a))return{name:a.name,glsl:a.glsl,length:a.length,widthFactor:a.widthFactor,margin:0,uniforms:[],attributes:[]};throw new Error("Invalid edge extremity specification: ".concat(JSON.stringify(a)))}function fo(a){var n,t,e=(n=a?.shapes)!==null&&n!==void 0?n:jt.shapes,r=(t=a?.layers)!==null&&t!==void 0?t:jt.layers,i=e.map(Ud),o=r.map(Od);if(i.length===0)throw new Error("At least one node shape must be specified.");if(o.length===0)throw new Error("At least one node layer must be specified.");return{shapes:i,layers:o}}function jd(a){var n,t,e,r=(n=a?.paths)!==null&&n!==void 0?n:Et.paths,i=(t=a?.extremities)!==null&&t!==void 0?t:Et.extremities,o=(e=a?.layers)!==null&&e!==void 0?e:Et.layers,s=r.map(Hd),l=i.map(Vd).filter(function(d){return d!==null}),u=o.map($d);if(s.length===0)throw new Error("At least one edge path must be specified.");if(u.length===0)throw new Error("At least one edge layer must be specified.");return{paths:s,extremities:l,layers:u}}function go(a,n){var t={},e=U(a),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;i.variables&&Object.assign(t,i.variables)}}catch(o){e.e(o)}finally{e.f()}return Object.assign(t,n||{}),t}function po(a,n,t,e,r){var i=fo(e),o=i.shapes,s=i.layers,l=go(o,e?.variables),u=ao(a,n,t,{shapes:o,layers:s,label:e?.label,backdrop:e?.backdrop,labelAttachments:e?.labelAttachments},r);return H(H({},u),{},{variables:l})}function vo(a,n,t,e,r,i){var o=jd(e),s=o.paths,l=o.extremities,u=o.layers,d=fo(i),h=d.shapes,f=d.layers,c=go(s,e?.variables),g=co(a,n,t,{paths:s,extremities:l,layers:u,defaultHead:e?.defaultHead,defaultTail:e?.defaultTail,label:e?.label},r,h,f);return H(H({},g),{},{variables:c,paths:s})}var Xd=2;function mo(a,n,t){var e=arguments.length>3&&arguments[3]!==void 0?arguments[3]:Xd,r=n.canvas,i=r.width,o=r.height,s=t.x,l=t.y,u=t.rowHeight,d=t.maxRowWidth,h={},f=[],c=U(a),g;try{for(c.s();!(g=c.n()).done;){var b=g.value,m=b.width+e,p=b.height+e;if(m>i||p>o||s+m>i&&l+u+p>o){f.push(b);continue}s+m>i&&(d=Math.max(d,s),s=0,l+=u,u=p),b.draw(n,s,l),h[b.key]={x:s,y:l,width:b.width,height:b.height},s+=m,u=Math.max(u,p)}}catch(v){c.e(v)}finally{c.f()}return d=Math.max(d,s),{atlas:h,cursor:{x:s,y:l,rowHeight:u,maxRowWidth:d},remaining:f}}function Se(a,n,t,e){var r=Object.defineProperty;try{r({},"",{})}catch{r=0}Se=function(i,o,s,l){function u(d,h){Se(i,d,function(f){return this._invoke(d,h,f)})}o?r?r(i,o,{value:s,enumerable:!l,configurable:!l,writable:!l}):i[o]=s:(u("next",0),u("throw",1),u("return",2))},Se(a,n,t,e)}function Ee(){var a,n,t=typeof Symbol=="function"?Symbol:{},e=t.iterator||"@@iterator",r=t.toStringTag||"@@toStringTag";function i(c,g,b,m){var p=g&&g.prototype instanceof s?g:s,v=Object.create(p.prototype);return Se(v,"_invoke",(function(y,T,_){var S,D,w,k=0,L=_||[],N=!1,R={p:0,n:0,v:a,a:z,f:z.bind(a,4),d:function(C,I){return S=C,D=0,w=a,R.n=I,o}};function z(C,I){for(D=C,w=I,n=0;!N&&k&&!B&&n<L.length;n++){var B,F=L[n],x=R.p,P=F[2];C>3?(B=P===I)&&(w=F[(D=F[4])?5:(D=3,3)],F[4]=F[5]=a):F[0]<=x&&((B=C<2&&x<F[1])?(D=0,R.v=I,R.n=F[1]):x<P&&(B=C<3||F[0]>I||I>P)&&(F[4]=C,F[5]=I,R.n=P,D=0))}if(B||C>1)return o;throw N=!0,I}return function(C,I,B){if(k>1)throw TypeError("Generator is already running");for(N&&I===1&&z(I,B),D=I,w=B;(n=D<2?a:w)||!N;){S||(D?D<3?(D>1&&(R.n=-1),z(D,w)):R.n=w:R.v=w);try{if(k=2,S){if(D||(C="next"),n=S[C]){if(!(n=n.call(S,w)))throw TypeError("iterator result is not an object");if(!n.done)return n;w=n.value,D<2&&(D=0)}else D===1&&(n=S.return)&&n.call(S),D<2&&(w=TypeError("The iterator does not provide a '"+C+"' method"),D=1);S=a}else if((n=(N=R.n<0)?w:y.call(T,R))!==o)break}catch(F){S=a,D=1,w=F}finally{k=1}}return{value:n,done:N}}})(c,b,m),!0),v}var o={};function s(){}function l(){}function u(){}n=Object.getPrototypeOf;var d=[][e]?n(n([][e]())):(Se(n={},e,function(){return this}),n),h=u.prototype=s.prototype=Object.create(d);function f(c){return Object.setPrototypeOf?Object.setPrototypeOf(c,u):(c.__proto__=u,Se(c,r,"GeneratorFunction")),c.prototype=Object.create(h),c}return l.prototype=u,Se(h,"constructor",u),Se(u,"constructor",l),l.displayName="GeneratorFunction",Se(u,r,"GeneratorFunction"),Se(h),Se(h,r,"Generator"),Se(h,e,function(){return this}),Se(h,"toString",function(){return"[object Generator]"}),(Ee=function(){return{w:i,m:f}})()}function bo(a,n,t,e,r,i,o){try{var s=a[i](o),l=s.value}catch(u){return void t(u)}s.done?n(l):Promise.resolve(l).then(e,r)}function Gt(a){return function(){var n=this,t=arguments;return new Promise(function(e,r){var i=a.apply(n,t);function o(l){bo(i,e,r,o,s,"next",l)}function s(l){bo(i,e,r,o,s,"throw",l)}o(void 0)})}}var ta=new Map,yo=!1;function qd(a){return new Promise(function(n,t){var e=new FileReader;e.onload=function(){return n(e.result)},e.onerror=t,e.readAsDataURL(a)})}function Kd(a){if(!ta.has(a)){var n=fetch(a).then(function(t){return t.blob()}).then(qd).catch(function(){return ta.delete(a),null});ta.set(a,n)}return ta.get(a)}function Yd(a){return Xa.apply(this,arguments)}function Xa(){return Xa=Gt(Ee().m(function a(n){var t,e,r;return Ee().w(function(i){for(;;)switch(i.n){case 0:return t=document.createElement("div"),t.innerHTML=n,e=Array.from(t.querySelectorAll("img[src]")),r=e.filter(function(o){return!o.getAttribute("src").startsWith("data:")}),i.n=1,Promise.all(r.map((function(){var o=Gt(Ee().m(function s(l){var u;return Ee().w(function(d){for(;;)switch(d.n){case 0:return d.n=1,Kd(l.getAttribute("src"));case 1:u=d.v,u&&l.setAttribute("src",u);case 2:return d.a(2)}},s)}));return function(s){return o.apply(this,arguments)}})()));case 1:return i.a(2,t.innerHTML)}},a)})),Xa.apply(this,arguments)}function xo(a){return qa.apply(this,arguments)}function qa(){return qa=Gt(Ee().m(function a(n){var t,e,r,i,o,s,l,u,d,h,f,c,g,b=arguments;return Ee().w(function(m){for(;;)switch(m.n){case 0:if(t=b.length>1&&b[1]!==void 0?b[1]:1,e=n instanceof SVGElement?new XMLSerializer().serializeToString(n):n,r=new DOMParser,i=r.parseFromString(e,"image/svg+xml"),o=i.querySelector("svg"),o){m.n=1;break}return m.a(2,null);case 1:if(s=parseFloat(o.getAttribute("width")||""),l=parseFloat(o.getAttribute("height")||""),(!(s>0)||!(l>0))&&(u=o.getAttribute("viewBox"),u&&(d=u.trim().split(/[\s,]+/),s=parseFloat(d[2]),l=parseFloat(d[3]))),!(!(s>0)||!(l>0))){m.n=2;break}return console.warn("Sigma: SVG label attachment has no parseable dimensions \u2014 skipped."),m.a(2,null);case 2:return h=Math.ceil(s*t),f=Math.ceil(l*t),c=new Blob([e],{type:"image/svg+xml"}),g=URL.createObjectURL(c),m.a(2,new Promise(function(p){var v=new Image;v.onload=function(){var y=document.createElement("canvas");y.width=h,y.height=f,y.getContext("2d").drawImage(v,0,0,h,f),URL.revokeObjectURL(g);try{y.getContext("2d").getImageData(0,0,1,1)}catch(T){if(T instanceof DOMException&&T.name==="SecurityError"){yo||(yo=!0,console.warn('Sigma: A label attachment was skipped because the rendered canvas is tainted. SVG with <foreignObject> (used by the "html" attachment type) is blocked in Chromium and Safari. Use type: "canvas" with Canvas 2D rendering instead.')),p(null);return}throw T}p(y)},v.onerror=function(){URL.revokeObjectURL(g),p(null)},v.src=g}))}},a)})),qa.apply(this,arguments)}function Zd(a,n){var t=document.createElement("div");t.style.cssText="position:fixed;left:-99999px;top:0;visibility:hidden;width:max-content;height:max-content",t.innerHTML=(n?"<style>".concat(n,"</style>"):"")+a,document.body.appendChild(t);var e=t.getBoundingClientRect(),r=e.width,i=e.height;return document.body.removeChild(t),{width:Math.ceil(r),height:Math.ceil(i)}}function Qd(a,n,t,e){return Ka.apply(this,arguments)}function Ka(){return Ka=Gt(Ee().m(function a(n,t,e,r){var i,o,s,l,u,d,h,f,c=arguments;return Ee().w(function(g){for(;;)switch(g.n){case 0:return i=c.length>4&&c[4]!==void 0?c[4]:1,o=n instanceof HTMLElement?n.outerHTML:n,g.n=1,Yd(o);case 1:return s=g.v,(e==null||r==null)&&(l=Zd(s,t),e=e??l.width,r=r??l.height),u=Math.ceil(e*i),d=Math.ceil(r*i),h=t?"<style>".concat(t,"</style>"):"",f='<svg xmlns="http://www.w3.org/2000/svg" '+'width="'.concat(u,'" height="').concat(d,'" viewBox="0 0 ').concat(e," ").concat(r,'">')+'<foreignObject width="'.concat(e,'" height="').concat(r,'">')+'<body xmlns="http://www.w3.org/1999/xhtml" style="margin:0;padding:0">'.concat(h).concat(s,"</body>")+"</foreignObject></svg>",g.a(2,xo(f,1))}},a)})),Ka.apply(this,arguments)}function Jd(a){return Ya.apply(this,arguments)}function Ya(){return Ya=Gt(Ee().m(function a(n){var t,e=arguments,r;return Ee().w(function(i){for(;;)switch(i.n){case 0:t=e.length>1&&e[1]!==void 0?e[1]:1,r=n.type,i.n=r==="canvas"?1:r==="svg"?2:r==="html"?3:4;break;case 1:return i.a(2,n.canvas);case 2:return i.a(2,xo(n.svg,t));case 3:return i.a(2,Qd(n.html,n.css,n.width,n.height,t));case 4:return i.a(2)}},a)})),Ya.apply(this,arguments)}var It=2048,_o=(function(){function a(n,t,e){ne(this,a),E(this,"cache",new Map),E(this,"pending",new Set),E(this,"atlas",{}),E(this,"glTexture",null),E(this,"dirty",!0),this.gl=n,this.renderers=t,this.scheduleRender=e,this.packCanvas=document.createElement("canvas"),this.packCanvas.width=It,this.packCanvas.height=It,this.packCtx=this.packCanvas.getContext("2d")}return ae(a,[{key:"renderAttachment",value:function(t,e,r){var i=this,o="".concat(t,":").concat(e);if(!(this.cache.has(o)||this.pending.has(o))){var s=this.renderers[e];if(s){var l=s(r);if(l){this.pending.add(o);var u=r.pixelRatio;Promise.resolve(l).then((function(){var d=Gt(Ee().m(function h(f){var c;return Ee().w(function(g){for(;;)switch(g.n){case 0:if(i.pending.has(o)){g.n=1;break}return g.a(2);case 1:if(i.pending.delete(o),f){g.n=2;break}return g.a(2);case 2:return g.n=3,Jd(f,u);case 3:if(c=g.v,!(!c||c.width===0||c.height===0)){g.n=4;break}return g.a(2);case 4:i.cache.set(o,{image:c,width:c.width,height:c.height}),i.dirty=!0,i.scheduleRender();case 5:return g.a(2)}},h)}));return function(h){return d.apply(this,arguments)}})())}}}}},{key:"regenerateAtlas",value:function(){if(this.dirty){this.dirty=!1;var t=[];if(this.cache.forEach(function(u,d){t.push({key:d,width:u.width,height:u.height,draw:function(f,c,g){f.drawImage(u.image,c,g)}})}),t.length===0){this.atlas={},this.deleteGLTexture();return}var e={x:0,y:0,rowHeight:0,maxRowWidth:0};this.packCtx.clearRect(0,0,It,It);var r=mo(t,this.packCtx,e),i=r.atlas,o=r.remaining;this.atlas=i,o.length>0&&console.warn("Sigma: ".concat(o.length," label attachment(s) could not fit in the ").concat(It,"x").concat(It," atlas and will not be rendered.")),this.deleteGLTexture();var s=this.gl,l=s.createTexture();s.activeTexture(s.TEXTURE0+an),s.bindTexture(s.TEXTURE_2D,l),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!0),s.texImage2D(s.TEXTURE_2D,0,s.RGBA,s.RGBA,s.UNSIGNED_BYTE,this.packCanvas),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MIN_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MAG_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_S,s.CLAMP_TO_EDGE),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_T,s.CLAMP_TO_EDGE),this.glTexture=l}}},{key:"bindTexture",value:function(t){if(this.glTexture){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.glTexture)}}},{key:"getEntry",value:function(t,e){var r="".concat(t,":").concat(e);return this.atlas[r]||null}},{key:"invalidateNode",value:function(t){var e="".concat(t,":"),r=U(this.cache.keys()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;o.startsWith(e)&&(this.cache.delete(o),this.dirty=!0)}}catch(d){r.e(d)}finally{r.f()}var s=U(this.pending),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;u.startsWith(e)&&this.pending.delete(u)}}catch(d){s.e(d)}finally{s.f()}}},{key:"restore",value:function(){this.glTexture=null,this.dirty=!0}},{key:"clear",value:function(){this.cache.clear(),this.pending.clear(),this.atlas={},this.dirty=!0,this.deleteGLTexture()}},{key:"kill",value:function(){this.clear(),this.packCanvas=null,this.packCtx=null,this.gl=null}},{key:"deleteGLTexture",value:function(){this.glTexture&&(this.gl.deleteTexture(this.glTexture),this.glTexture=null)}}])})(),Za=(function(){function a(n){ne(this,a),E(this,"buckets",new Map),E(this,"keyDepth",new Map);var t=U(n),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;this.buckets.set(r,new Set)}}catch(i){t.e(i)}finally{t.f()}}return ae(a,[{key:"has",value:function(t){return this.keyDepth.has(t)}},{key:"getBucket",value:function(t){return this.buckets.get(t)}},{key:"set",value:function(t,e){var r,i=this.keyDepth.get(t);if(i!==e){var o=this.buckets.get(e);if(!o)throw new Error('Sigma: "'.concat(e,'" is not a declared depth layer'));i!==void 0&&((r=this.buckets.get(i))===null||r===void 0||r.delete(t)),o.add(t),this.keyDepth.set(t,e)}}},{key:"remove",value:function(t){var e=this.keyDepth.get(t);e!==void 0&&(this.buckets.get(e).delete(t),this.keyDepth.delete(t))}},{key:"clearAll",value:function(){var t=U(this.buckets.values()),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;r.clear()}}catch(i){t.e(i)}finally{t.f()}this.keyDepth.clear()}},{key:"getSorted",value:function(t,e){var r=this.buckets.get(t);return!r||r.size===0?[]:J(r).sort(function(i,o){return e(i)-e(o)})}}])})(),To=(function(a){function n(t,e){return ne(this,n),fe(this,n,[t,2,e])}return ge(n,a),ae(n,[{key:"updateNode",value:function(e,r,i,o,s,l,u,d){var h=this.indexMap.get(e);if(h===void 0)throw new Error('Node "'.concat(e,'" not allocated in NodeDataTexture'));var f=Xe(d),c=oe(f,4),g=c[0],b=c[1],m=c[2],p=c[3],v=h*2*4;this.data[v]=r,this.data[v+1]=i,this.data[v+2]=o,this.data[v+3]=s,this.data[v+4]=l,this.data[v+5]=u,this.data[v+6]=g*65536+b*256+m,this.data[v+7]=p/255,this.markDirty(h)}}])})(Yn),eh=1024,th=1.5,nh=4096,Qa=(function(){function a(n,t){var e;if(ne(this,a),E(this,"texture",null),E(this,"framebuffer",null),!n.getExtension("EXT_color_buffer_float"))throw new Error("sigma: EXT_color_buffer_float is required for label/edge placement but is unavailable in this WebGL2 context.");this.gl=n,this.channels=t.channels,this.capacity=this.roundUpToPowerOfTwo((e=t.initialCapacity)!==null&&e!==void 0?e:eh);var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.create()}return ae(a,[{key:"roundUpToPowerOfTwo",value:function(t){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,t))))}},{key:"computeDimensions",value:function(t){var e=Math.min(t,nh),r=Math.ceil(t/e);return{width:e,height:r}}},{key:"create",value:function(){var t=this.gl,e=this.channels===1?t.R32F:t.RGBA32F,r=this.channels===1?t.RED:t.RGBA;if(this.texture=t.createTexture(),t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture),t.texImage2D(t.TEXTURE_2D,0,e,this.textureWidth,this.textureHeight,0,r,t.FLOAT,null),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_S,t.CLAMP_TO_EDGE),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_T,t.CLAMP_TO_EDGE),this.framebuffer=t.createFramebuffer(),t.bindFramebuffer(t.FRAMEBUFFER,this.framebuffer),t.framebufferTexture2D(t.FRAMEBUFFER,t.COLOR_ATTACHMENT0,t.TEXTURE_2D,this.texture,0),t.checkFramebufferStatus(t.FRAMEBUFFER)!==t.FRAMEBUFFER_COMPLETE)throw new Error("sigma: float framebuffer for frame-pass placement is incomplete.");t.bindFramebuffer(t.FRAMEBUFFER,null),t.bindTexture(t.TEXTURE_2D,null)}},{key:"ensureCapacity",value:function(t){if(!(t<=this.capacity)){var e=this.gl;this.capacity=this.roundUpToPowerOfTwo(Math.ceil(t*th));var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.texture&&e.deleteTexture(this.texture),this.framebuffer&&e.deleteFramebuffer(this.framebuffer),this.create()}}},{key:"bindAsRenderTarget",value:function(){var t=this.gl;t.bindFramebuffer(t.FRAMEBUFFER,this.framebuffer),t.viewport(0,0,this.textureWidth,this.textureHeight)}},{key:"bind",value:function(t){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"restore",value:function(){this.gl.getExtension("EXT_color_buffer_float"),this.create()}},{key:"kill",value:function(){var t=this.gl;this.texture&&(t.deleteTexture(this.texture),this.texture=null),this.framebuffer&&(t.deleteFramebuffer(this.framebuffer),this.framebuffer=null)}}])})(),So=(function(a){function n(t,e){return ne(this,n),fe(this,n,[t,2,e])}return ge(n,a),ae(n,[{key:"updateEdge",value:function(e,r,i,o,s,l){var u=arguments.length>6&&arguments[6]!==void 0?arguments[6]:0,d=arguments.length>7&&arguments[7]!==void 0?arguments[7]:0,h=arguments.length>8&&arguments[8]!==void 0?arguments[8]:0,f=this.indexMap.get(e);if(f===void 0)throw new Error('Edge "'.concat(e,'" not allocated in EdgeDataTexture'));var c=f*this.TEXELS_PER_ITEM*4;this.data[c+0]=r,this.data[c+1]=i,this.data[c+2]=o,this.data[c+3]=0,this.data[c+4]=s,this.data[c+5]=l,this.data[c+6]=u,this.data[c+7]=(d&15)<<4|h&15,this.markDirty(f)}}])})(Yn);var sf=$e(vt()),lf=$e(Pa());function ah(a,n){if(a==null)return{};var t={};for(var e in a)if({}.hasOwnProperty.call(a,e)){if(n.indexOf(e)!==-1)continue;t[e]=a[e]}return t}function Eo(a,n){if(a==null)return{};var t,e,r=ah(a,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(a);for(e=0;e<i.length;e++)t=i[e],n.indexOf(t)===-1&&{}.propertyIsEnumerable.call(a,t)&&(r[t]=a[t])}return r}var rh=["factor"],ih=["factor"],Ao=1.5,Ja={x:.5,y:.5,angle:0,ratio:1},Do=(function(a){function n(){var t;ne(this,n);for(var e=arguments.length,r=new Array(e),i=0;i<e;i++)r[i]=arguments[i];return t=fe(this,n,[].concat(r)),E(t,"minRatio",null),E(t,"maxRatio",null),E(t,"enabled",!0),E(t,"enabledZooming",!0),E(t,"enabledPanning",!0),E(t,"enabledRotation",!0),E(t,"constrainState",null),E(t,"state",H({},Ja)),E(t,"previousState",H({},Ja)),E(t,"nextFrame",null),E(t,"currentAnimationFrom",null),E(t,"currentAnimationTo",null),E(t,"animationCallback",null),t}return ge(n,a),ae(n,[{key:"x",get:function(){return this.state.x}},{key:"y",get:function(){return this.state.y}},{key:"angle",get:function(){return this.state.angle}},{key:"ratio",get:function(){return this.state.ratio}},{key:"getState",value:function(){return H({},this.state)}},{key:"getPreviousState",value:function(){return H({},this.previousState)}},{key:"getBoundedRatio",value:function(e){var r=e;return typeof this.minRatio=="number"&&(r=Math.max(r,this.minRatio)),typeof this.maxRatio=="number"&&(r=Math.min(r,this.maxRatio)),r}},{key:"validateState",value:function(e){var r=this.getState();return this.enabledPanning&&typeof e.x=="number"&&(r.x=e.x),this.enabledPanning&&typeof e.y=="number"&&(r.y=e.y),this.enabledZooming&&typeof e.ratio=="number"&&(r.ratio=this.getBoundedRatio(e.ratio)),this.enabledRotation&&typeof e.angle=="number"&&(r.angle=e.angle),this.constrainState?this.constrainState(r):r}},{key:"isAnimating",value:function(){return this.nextFrame!==null}},{key:"setState",value:function(e){if(!this.enabled)return this;var r=this.validateState(e);return Li(this.state,r)?this:(this.previousState=this.state,this.state=r,this.emit("updated",this.getState()),this)}},{key:"updateState",value:function(e){return this.setState(e(this.getState())),this}},{key:"animate",value:function(e,r){var i=this;return this.enabled?new Promise(function(o){return i.runAnimation(e,H(H({},Pi),r),o)}):Promise.resolve()}},{key:"cancelAnimation",value:function(){return this.nextFrame!==null&&(cancelAnimationFrame(this.nextFrame),this.nextFrame=null),this.resolveAnimation(),this.emitAnimationEnd(!1),this}},{key:"zoomIn",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=e.factor,i=r===void 0?Ao:r,o=Eo(e,rh);return this.animate({ratio:this.ratio/i},o)}},{key:"zoomOut",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=e.factor,i=r===void 0?Ao:r,o=Eo(e,ih);return this.animate({ratio:this.ratio*i},o)}},{key:"reset",value:function(e){return this.animate(Ja,e)}},{key:"runAnimation",value:function(e,r,i){var o=this,s=Fn(r.easing),l=Date.now(),u=this.getState(),d=this.validateState(e),h=function(){if(!o.enabled){o.cancelAnimation();return}var c=r.duration>0?(Date.now()-l)/r.duration:1;if(c>=1){o.nextFrame=null,o.setState(d),o.resolveAnimation(),o.emitAnimationEnd(!0);return}var g=s(c);o.setState({x:u.x+(d.x-u.x)*g,y:u.y+(d.y-u.y)*g,angle:u.angle+(d.angle-u.angle)*g,ratio:u.ratio+(d.ratio-u.ratio)*g}),o.nextFrame=requestAnimationFrame(h)};this.cancelAnimation(),this.currentAnimationFrom=u,this.currentAnimationTo=d,this.animationCallback=i,this.emit("animationStart",{from:u,to:d}),h()}},{key:"resolveAnimation",value:function(){var e=this.animationCallback;this.animationCallback=null,e&&e()}},{key:"emitAnimationEnd",value:function(e){var r=this.currentAnimationFrom,i=this.currentAnimationTo;!r||!i||(this.currentAnimationFrom=null,this.currentAnimationTo=null,this.emit("animationEnd",{from:r,to:i,completed:e}))}}],[{key:"from",value:function(e){var r=new n;return r.setState(e)}}])})(Kt);function Ae(a,n){var t=n.getBoundingClientRect();return{x:a.clientX-t.left,y:a.clientY-t.top}}function ze(a,n){var t=H(H({},Ae(a,n)),{},{sigmaDefaultPrevented:!1,preventSigmaDefault:function(){t.sigmaDefaultPrevented=!0},original:a});return t}function lt(a){var n="x"in a?a:H(H({},a.touches[0]||a.previousTouches[0]),{},{original:a.original,sigmaDefaultPrevented:a.sigmaDefaultPrevented,preventSigmaDefault:function(){a.sigmaDefaultPrevented=!0,n.sigmaDefaultPrevented=!0}});return n}function oh(a,n){var t=ze(a,n);return t.delta=Uo(a),t}var sh=2;function aa(a){for(var n=[],t=0,e=Math.min(a.length,sh);t<e;t++)n.push(a[t]);return n}function on(a,n,t){var e={touches:aa(a.touches).map(function(r){return Ae(r,t)}),previousTouches:n.map(function(r){return Ae(r,t)}),sigmaDefaultPrevented:!1,preventSigmaDefault:function(){e.sigmaDefaultPrevented=!0},original:a};return e}function Uo(a){if(typeof a.deltaY<"u")return a.deltaY*-3/360;if(typeof a.detail<"u")return a.detail/-9;throw new Error("Captor: could not extract delta from event.")}var Oo=(function(a){function n(t,e){var r;return ne(this,n),r=fe(this,n),r.container=t,r.renderer=e,r}return ge(n,a),ae(n)})(Kt),lh=["doubleClickTimeout","doubleClickZoomingDuration","doubleClickZoomingRatio","dragTimeout","draggedEventsTolerance","enableCameraMouseRotation","gestureTarget","inertiaDuration","inertiaRatio","zoomDuration","zoomingRatio"],uh=lh.reduce(function(a,n){return H(H({},a),{},E({},n,Wn[n]))},{});function sn(a){return a instanceof PointerEvent&&a.pointerType==="touch"}var dh=(function(a){function n(t,e){var r;return ne(this,n),r=fe(this,n,[t,e]),E(r,"enabled",!0),E(r,"draggedEvents",0),E(r,"downStartTime",null),E(r,"lastMouseX",null),E(r,"lastMouseY",null),E(r,"isMouseDown",!1),E(r,"isMoving",!1),E(r,"isPanningStage",!1),E(r,"movingTimeout",null),E(r,"startCameraState",null),E(r,"clicks",0),E(r,"doubleClickTimeout",null),E(r,"isRightMouseDown",!1),E(r,"startRotationAngle",null),E(r,"startCameraAngle",null),E(r,"currentWheelDirection",0),E(r,"lastWheelAnimationId",0),E(r,"settings",uh),r.handleRightClick=r.handleRightClick.bind(r),r.handleDown=r.handleDown.bind(r),r.handleUp=r.handleUp.bind(r),r.handleMove=r.handleMove.bind(r),r.handleWheel=r.handleWheel.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleEnter=r.handleEnter.bind(r),t.addEventListener("contextmenu",r.handleRightClick,{capture:!1}),t.addEventListener("pointerdown",r.handleDown,{capture:!1}),t.addEventListener("wheel",r.handleWheel,{capture:!1,passive:!1}),t.addEventListener("pointerleave",r.handleLeave,{capture:!1}),t.addEventListener("pointerenter",r.handleEnter,{capture:!1}),document.addEventListener("pointermove",r.handleMove,{capture:!1}),document.addEventListener("pointerup",r.handleUp,{capture:!1}),r}return ge(n,a),ae(n,[{key:"kill",value:function(){var e=this.container;e.removeEventListener("contextmenu",this.handleRightClick),e.removeEventListener("pointerdown",this.handleDown),e.removeEventListener("wheel",this.handleWheel),e.removeEventListener("pointerleave",this.handleLeave),e.removeEventListener("pointerenter",this.handleEnter),document.removeEventListener("pointermove",this.handleMove),document.removeEventListener("pointerup",this.handleUp)}},{key:"handleClick",value:function(e){var r=this;if(this.enabled){if(this.clicks++,this.clicks===2)return this.clicks=0,typeof this.doubleClickTimeout=="number"&&(clearTimeout(this.doubleClickTimeout),this.doubleClickTimeout=null),this.handleDoubleClick(e);setTimeout(function(){r.clicks=0,r.doubleClickTimeout=null},this.settings.doubleClickTimeout),this.draggedEvents<this.settings.draggedEventsTolerance&&this.emit("click",ze(e,this.container))}}},{key:"handleRightClick",value:function(e){this.enabled&&(this.settings.enableCameraMouseRotation&&e.preventDefault(),this.emit("rightClick",ze(e,this.container)))}},{key:"handleDoubleClick",value:function(e){if(this.enabled){e.preventDefault(),e.stopPropagation();var r=ze(e,this.container);if(this.emit("doubleClick",r),!r.sigmaDefaultPrevented){var i=this.renderer.getCamera(),o=i.getBoundedRatio(i.getState().ratio/this.settings.doubleClickZoomingRatio);i.animate(this.renderer.getViewportZoomedState(Ae(e,this.container),o),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}}},{key:"handleDown",value:function(e){if(!(!this.enabled||sn(e))){if(e.button===0){this.startCameraState=this.renderer.getCamera().getState();var r=Ae(e,this.container),i=r.x,o=r.y;this.lastMouseX=i,this.lastMouseY=o,this.draggedEvents=0,this.downStartTime=Date.now(),this.isMouseDown=!0}if(e.button===2&&this.settings.enableCameraMouseRotation){var s=Ae(e,this.container),l=s.x,u=s.y,d=this.container.offsetWidth/2,h=this.container.offsetHeight/2;this.startRotationAngle=Math.atan2(u-h,l-d),this.startCameraAngle=this.renderer.getCamera().getState().angle,this.isRightMouseDown=!0}this.emit("mousedown",ze(e,this.container))}}},{key:"handleUp",value:function(e){var r=this;if(!(!this.enabled||sn(e)||!this.isMouseDown&&!this.isRightMouseDown)){if(this.isRightMouseDown){this.isRightMouseDown=!1,this.startRotationAngle=null,this.startCameraAngle=null,this.emit("mouseup",ze(e,this.container));return}var i=this.renderer.getCamera();this.isMouseDown=!1,this.isPanningStage&&(this.isPanningStage=!1,this.renderer._setPanning(!1)),typeof this.movingTimeout=="number"&&(clearTimeout(this.movingTimeout),this.movingTimeout=null);var o=Ae(e,this.container),s=o.x,l=o.y,u=i.getState(),d=i.getPreviousState();this.isMoving?i.animate({x:u.x+this.settings.inertiaRatio*(u.x-d.x),y:u.y+this.settings.inertiaRatio*(u.y-d.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"}):(this.lastMouseX!==s||this.lastMouseY!==l)&&i.setState({x:u.x,y:u.y}),this.isMoving=!1,setTimeout(function(){var h=r.draggedEvents>0;r.draggedEvents=0;var f=r.renderer.getSetting("hideEdgesOnMove")||r.renderer.getSetting("hideLabelsOnMove");h&&f&&r.renderer.refresh()},0),this.emit("mouseup",ze(e,this.container)),(e.target===this.container||e.composedPath()[0]===this.container)&&this.handleClick(e)}}},{key:"handleMove",value:function(e){var r=this;if(!(!this.enabled||sn(e))){var i=ze(e,this.container);if(this.emit("mousemovebody",i),(e.target===this.container||e.composedPath()[0]===this.container)&&this.emit("mousemove",i),this.isMouseDown&&this.draggedEvents++,!i.sigmaDefaultPrevented){if(this.isMouseDown){this.isMoving=!0,typeof this.movingTimeout=="number"&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.movingTimeout=null,r.isMoving=!1},this.settings.dragTimeout);var o=this.renderer.getCamera(),s=Ae(e,this.container),l=s.x,u=s.y,d=this.renderer.viewportToFramedGraph({x:this.lastMouseX,y:this.lastMouseY}),h=this.renderer.viewportToFramedGraph({x:l,y:u}),f=d.x-h.x,c=d.y-h.y,g=o.getState(),b=g.x+f,m=g.y+c;this.isPanningStage||(this.isPanningStage=!0,this.renderer._setPanning(!0)),o.setState({x:b,y:m}),this.lastMouseX=l,this.lastMouseY=u,e.preventDefault(),e.stopPropagation()}if(this.isRightMouseDown){var p=Ae(e,this.container),v=p.x,y=p.y,T=this.container.offsetWidth/2,_=this.container.offsetHeight/2,S=Math.atan2(y-_,v-T),D=S-this.startRotationAngle,w=this.renderer.getCamera();w.setState({angle:this.startCameraAngle+D}),e.preventDefault(),e.stopPropagation()}}}}},{key:"handleLeave",value:function(e){sn(e)||this.emit("mouseleave",ze(e,this.container))}},{key:"handleEnter",value:function(e){sn(e)||this.emit("mouseenter",ze(e,this.container))}},{key:"handleWheel",value:function(e){var r=this,i=this.renderer.getCamera();if(!(!this.enabled||!i.enabledZooming)){var o=oh(e,this.container);if(this.emit("wheel",o),!o.sigmaDefaultPrevented){var s=this.settings.gestureTarget;if(s!=="page"){if(s==="shared"&&!e.ctrlKey&&!e.metaKey){this.renderer._showGestureHint("wheel");return}var l=Uo(e);if(l){e.preventDefault(),e.stopPropagation();var u=i.getState().ratio,d=l>0?1/this.settings.zoomingRatio:this.settings.zoomingRatio,h=i.getBoundedRatio(u*d),f=l>0?1:-1,c=Date.now();if(u!==h&&!(this.currentWheelDirection===f&&this.lastWheelTriggerTime&&c-this.lastWheelTriggerTime<this.settings.zoomDuration/5)){var g=++this.lastWheelAnimationId;i.animate(this.renderer.getViewportZoomedState(Ae(e,this.container),h),{easing:"quadraticOut",duration:this.settings.zoomDuration}).then(function(){r.lastWheelAnimationId===g&&(r.currentWheelDirection=0)}),this.currentWheelDirection=f,this.lastWheelTriggerTime=c}}}}}}},{key:"setSettings",value:function(e){this.settings=e}}])})(Oo),hh=["dragTimeout","gestureTarget","inertiaDuration","inertiaRatio","doubleClickTimeout","doubleClickZoomingRatio","doubleClickZoomingDuration","tapMoveTolerance"],ch=hh.reduce(function(a,n){return H(H({},a),{},E({},n,Wn[n]))},{}),fh=(function(a){function n(t,e){var r;return ne(this,n),r=fe(this,n,[t,e]),E(r,"enabled",!0),E(r,"isMoving",!1),E(r,"hasMoved",!1),E(r,"isPanningStage",!1),E(r,"isZoomingStage",!1),E(r,"touchMode",0),E(r,"startTouchesPositions",[]),E(r,"lastTouches",[]),E(r,"lastTap",null),E(r,"settings",ch),r.handleStart=r.handleStart.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleMove=r.handleMove.bind(r),t.addEventListener("touchstart",r.handleStart,{capture:!1}),t.addEventListener("touchcancel",r.handleLeave,{capture:!1}),document.addEventListener("touchend",r.handleLeave,{capture:!1,passive:!1}),document.addEventListener("touchmove",r.handleMove,{capture:!1,passive:!1}),r}return ge(n,a),ae(n,[{key:"kill",value:function(){var e=this.container;e.removeEventListener("touchstart",this.handleStart),e.removeEventListener("touchcancel",this.handleLeave),document.removeEventListener("touchend",this.handleLeave),document.removeEventListener("touchmove",this.handleMove)}},{key:"getDimensions",value:function(){return{width:this.container.offsetWidth,height:this.container.offsetHeight}}},{key:"doesCapture",value:function(e){var r=this.settings.gestureTarget;return r==="graph"?!0:r==="page"?!1:e>=2}},{key:"syncStageFlags",value:function(){var e=this.touchMode===1&&this.hasMoved,r=this.touchMode===2;e!==this.isPanningStage&&(this.isPanningStage=e,this.renderer._setPanning(e)),r!==this.isZoomingStage&&(this.isZoomingStage=r,this.renderer._setZooming(r))}},{key:"handleStart",value:function(e){var r=this;if(this.enabled){var i=aa(e.touches);if(this.touchMode=i.length,this.startCameraState=this.renderer.getCamera().getState(),this.startTouchesPositions=i.map(function(c){return Ae(c,r.container)}),this.touchMode===2){var o=oe(this.startTouchesPositions,2),s=o[0],l=s.x,u=s.y,d=o[1],h=d.x,f=d.y;this.startTouchesAngle=Math.atan2(f-u,h-l),this.startTouchesDistance=Math.sqrt(Math.pow(h-l,2)+Math.pow(f-u,2))}this.syncStageFlags(),this.emit("touchdown",on(e,this.lastTouches,this.container)),this.lastTouches=i,this.lastTouchesPositions=this.startTouchesPositions,e.cancelable&&(this.doesCapture(e.touches.length)||this.renderer._hasNodeDrag())&&e.preventDefault()}}},{key:"handleLeave",value:function(e){if(!(!this.enabled||!this.startTouchesPositions.length)){switch(e.cancelable&&this.doesCapture(this.touchMode)&&e.preventDefault(),this.movingTimeout&&(this.isMoving=!1,clearTimeout(this.movingTimeout)),this.touchMode){case 2:if(e.touches.length===1){this.handleStart(e);break}case 1:if(this.isMoving){var r=this.renderer.getCamera(),i=r.getState(),o=r.getPreviousState();r.animate({x:i.x+this.settings.inertiaRatio*(i.x-o.x),y:i.y+this.settings.inertiaRatio*(i.y-o.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"})}this.hasMoved=!1,this.isMoving=!1,this.touchMode=0;break}if(this.syncStageFlags(),this.emit("touchup",on(e,this.lastTouches,this.container)),!e.touches.length){var s=Ae(this.lastTouches[0],this.container),l=this.startTouchesPositions[0],u=Math.pow(s.x-l.x,2)+Math.pow(s.y-l.y,2);if(!e.touches.length&&u<Math.pow(this.settings.tapMoveTolerance,2))if(this.lastTap&&Date.now()-this.lastTap.time<this.settings.doubleClickTimeout){var d=on(e,this.lastTouches,this.container);if(this.emit("doubletap",d),this.lastTap=null,!d.sigmaDefaultPrevented&&this.settings.gestureTarget!=="page"){var h=this.renderer.getCamera(),f=h.getBoundedRatio(h.getState().ratio/this.settings.doubleClickZoomingRatio);h.animate(this.renderer.getViewportZoomedState(s,f),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}else{var c=on(e,this.lastTouches,this.container);this.emit("tap",c),this.lastTap={time:Date.now(),position:c.touches[0]||c.previousTouches[0]}}}this.lastTouches=aa(e.touches),this.startTouchesPositions=[]}}},{key:"handleMove",value:function(e){var r=this;if(!(!this.enabled||!this.startTouchesPositions.length)){var i=this.doesCapture(e.touches.length);i&&e.preventDefault();var o=aa(e.touches),s=o.map(function(A){return Ae(A,r.container)}),l=this.lastTouches;this.lastTouches=o,this.lastTouchesPositions=s;var u=on(e,l,this.container);if(this.emit("touchmove",u),!u.sigmaDefaultPrevented&&(this.hasMoved||(this.hasMoved=s.some(function(A,q){var V=r.startTouchesPositions[q];return V&&(A.x!==V.x||A.y!==V.y)})),!!this.hasMoved)){if(!i){this.settings.gestureTarget==="shared"&&this.renderer._showGestureHint("touch");return}this.isMoving=!0,this.syncStageFlags(),this.movingTimeout&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.isMoving=!1},this.settings.dragTimeout);var d=this.renderer.getCamera(),h=this.startCameraState,f=this.renderer.getSetting("stagePadding");switch(this.touchMode){case 1:{var c=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0]),g=c.x,b=c.y,m=this.renderer.viewportToFramedGraph(s[0]),p=m.x,v=m.y;d.setState({x:h.x+g-p,y:h.y+b-v});break}case 2:{var y={x:.5,y:.5,angle:0,ratio:1},T=s[0],_=T.x,S=T.y,D=s[1],w=D.x,k=D.y,L=Math.atan2(k-S,w-_)-this.startTouchesAngle,N=Math.hypot(k-S,w-_)/this.startTouchesDistance,R=d.getBoundedRatio(h.ratio/N);y.ratio=R,y.angle=h.angle+L;var z=this.getDimensions(),C=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0],{cameraState:h}),I=Math.min(z.width,z.height)-2*f,B=I/z.width,F=I/z.height,x=R/I,P=_-I/2/B,W=S-I/2/F,G=pa({x:P,y:W},y.angle);P=G.x,W=G.y,y.x=C.x-P*x,y.y=C.y+W*x,d.setState(y);break}}}}}},{key:"setSettings",value:function(e){this.settings=e}}])})(Oo),gh=(function(){function a(n,t,e,r){ne(this,a),E(this,"pendingNode",null),E(this,"session",null),this.graph=n,this.viewportToGraph=t,this.setNodesState=e,this.emit=r}return ae(a,[{key:"start",value:function(t,e,r,i,o){var s=r(t),l=!1;if(this.emit("nodeDragStart",{node:t,allDraggedNodes:s,event:e,preventSigmaDefault:function(){l=!0}}),l)return!1;var u=new Map,d=U(s),h;try{for(d.s();!(h=d.n()).done;){var f=h.value;u.set(f,{x:this.graph.getNodeAttribute(f,i),y:this.graph.getNodeAttribute(f,o)})}}catch(c){d.e(c)}finally{d.f()}return this.session={node:t,allNodes:s,startPosition:this.viewportToGraph(e),startNodePositions:u,xAttr:i,yAttr:o},this.setNodesState(s,{isDragged:!0}),!0}},{key:"applyMove",value:function(t,e){var r=this.session,i=r.allNodes,o=r.startNodePositions,s=r.startPosition,l=r.xAttr,u=r.yAttr,d=this.viewportToGraph(t),h={x:d.x-s.x,y:d.y-s.y},f=U(i),c;try{for(f.s();!(c=f.n()).done;){var g=c.value,b=o.get(g);if(!(!b||!this.graph.hasNode(g))){var m={x:b.x+h.x,y:b.y+h.y};e?this.graph.mergeNodeAttributes(g,e(m,g)):(this.graph.setNodeAttribute(g,l,m.x),this.graph.setNodeAttribute(g,u,m.y))}}}catch(p){f.e(p)}finally{f.f()}}},{key:"end",value:function(){if(this.pendingNode=null,!this.session)return null;var t=this.session,e=t.node,r=t.allNodes;return this.setNodesState(r,{isDragged:!1}),this.session=null,{node:e,allNodes:r}}},{key:"removeNode",value:function(t){t===this.pendingNode&&(this.pendingNode=null),this.session&&(this.session.node===t?(this.setNodesState(this.session.allNodes,{isDragged:!1}),this.session=null):this.session.allNodes.includes(t)&&(this.session.allNodes=this.session.allNodes.filter(function(e){return e!==t}),this.session.startNodePositions.delete(t)))}},{key:"clear",value:function(){this.pendingNode=null,this.session=null}}])})();function wo(a,n){return a<n?"".concat(a,"\0").concat(n):"".concat(n,"\0").concat(a)}var ph=(function(){function a(n,t){ne(this,a),E(this,"groups",new Map),E(this,"edgeToGroupKey",new Map),this.graph=n,this.onGroupChanged=t}return ae(a,[{key:"getGroupKey",value:function(t){return wo(this.graph.source(t),this.graph.target(t))}},{key:"sortGroup",value:function(t,e){var r=this,i=e.split("\0")[0];t.sort(function(o,s){var l=r.graph.source(o)===i||!r.graph.isDirected(o)?0:1,u=r.graph.source(s)===i||!r.graph.isDirected(s)?0:1;return l-u})}},{key:"register",value:function(t){var e=this.getGroupKey(t),r=this.groups.get(e);r||(r=[],this.groups.set(e,r)),r.includes(t)||r.push(t),this.edgeToGroupKey.set(t,e),r.length>1&&this.sortGroup(r,e),this.onGroupChanged(r,r.length)}},{key:"unregister",value:function(t){var e=this.edgeToGroupKey.get(t);if(e){this.edgeToGroupKey.delete(t);var r=this.groups.get(e);if(r){var i=r.indexOf(t);i!==-1&&r.splice(i,1),r.length===0?this.groups.delete(e):this.onGroupChanged(r,r.length)}}}},{key:"getGroup",value:function(t){var e,r=this.edgeToGroupKey.get(t);return r?(e=this.groups.get(r))!==null&&e!==void 0?e:[]:[]}},{key:"getSiblings",value:function(t){return this.getGroup(t).filter(function(e){return e!==t})}},{key:"rebuild",value:function(){var t=this;this.groups.clear(),this.edgeToGroupKey.clear(),this.graph.forEachEdge(function(l,u,d,h){var f=wo(d,h),c=t.groups.get(f);c||(c=[],t.groups.set(f,c)),c.push(l),t.edgeToGroupKey.set(l,f)});var e=U(this.groups),r;try{for(e.s();!(r=e.n()).done;){var i=oe(r.value,2),o=i[0],s=i[1];s.length>1&&this.sortGroup(s,o),this.onGroupChanged(s,s.length)}}catch(l){e.e(l)}finally{e.f()}}},{key:"clear",value:function(){this.groups.clear(),this.edgeToGroupKey.clear()}}])})();function Ro(a,n){var t;if(a===!1||a==="extend"||a==="separate")return a;var e=a[n];return e!==void 0?e:(t=a.default)!==null&&t!==void 0?t:!1}function Lo(a){return a===!1?!1:a==="extend"||a==="separate"?!0:Object.values(a).some(function(n){return n==="extend"||n==="separate"})}function Co(a){return a==="separate"?!0:a===!1||a==="extend"?!1:a.default==="separate"?!0:Object.values(a).includes("separate")}var _e={node:{parent:null,eventSuffix:"Node",payloadKey:"node",isEnabled:function(){return!0},writesPickingThisFrame:function(){return!0},setHover:function(n,t,e){return n.setNodeState(t,{isHovered:e})},getCursor:function(n,t){var e;return(e=n.nodeDataCache[t])===null||e===void 0?void 0:e.cursor},isHitValid:function(n,t){var e;return((e=n.nodeDataCache[t])===null||e===void 0?void 0:e.visibility)!=="hidden"}},edge:{parent:null,eventSuffix:"Edge",payloadKey:"edge",isEnabled:function(n){return!!n.settings.enableEdgeEvents},writesPickingThisFrame:function(n){return!!n.settings.enableEdgeEvents},setHover:function(n,t,e){return n.setEdgeState(t,{isHovered:e})},getCursor:function(n,t){var e;return(e=n.edgeDataCache[t])===null||e===void 0?void 0:e.cursor}},nodeLabel:{parent:"node",eventSuffix:"NodeLabel",payloadKey:"node",isEnabled:function(n){return Co(n.settings.nodeLabelEvents)},writesPickingThisFrame:function(n){return Lo(n.settings.nodeLabelEvents)},setHover:function(n,t,e){return n.setNodeState(t,{isLabelHovered:e})},getCursor:function(n,t){var e;return(e=n.nodeDataCache[t])===null||e===void 0?void 0:e.labelCursor},resolveForVerb:function(n,t,e){var r=Ro(e.settings.nodeLabelEvents,t);return r===!1?null:r==="extend"?{kind:"node",key:n.key}:n}},edgeLabel:{parent:"edge",eventSuffix:"EdgeLabel",payloadKey:"edge",isEnabled:function(n){return Co(n.settings.edgeLabelEvents)},writesPickingThisFrame:function(n){return Lo(n.settings.edgeLabelEvents)},setHover:function(n,t,e){return n.setEdgeState(t,{isLabelHovered:e})},getCursor:function(n,t){var e;return(e=n.edgeDataCache[t])===null||e===void 0?void 0:e.labelCursor},resolveForVerb:function(n,t,e){var r=Ro(e.settings.edgeLabelEvents,t);return r===!1?null:r==="extend"?{kind:"edge",key:n.key}:n}}},ht=["node","edge","nodeLabel","edgeLabel"];function vh(){return{lookup:[null],idsByKind:{node:new Map,edge:new Map,nodeLabel:new Map,edgeLabel:new Map},nextId:1}}function dt(a,n,t){var e;return(e=a.idsByKind[n].get(t))!==null&&e!==void 0?e:0}function er(a,n){var t=U(ht),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;if(r===n||_e[r].parent===n){var i=U(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(f){i.e(f)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(f){t.e(f)}finally{t.f()}var l=1,u=U(ht),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;if(h===n)break;l+=a.idsByKind[h].size}}catch(f){u.e(f)}finally{u.f()}a.nextId=l}function ko(a,n,t){var e=a.nextId++;return a.idsByKind[n].set(t,e),a.lookup[e]={kind:n,key:t},e}function mh(a,n){var t=U(ht),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;if(_e[r].parent!==null){var i=U(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(_){i.e(_)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(_){t.e(_)}finally{t.f()}a.nextId=1;var l=U(ht),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;if(_e[d].parent===null){var h=U(a.idsByKind[d].values()),f;try{for(h.s();!(f=h.n()).done;){var c=f.value;c>=a.nextId&&(a.nextId=c+1)}}catch(_){h.e(_)}finally{h.f()}}}}catch(_){l.e(_)}finally{l.f()}var g=U(ht),b;try{for(g.s();!(b=g.n()).done;){var m=b.value,p=_e[m];if(!(p.parent===null||!p.isEnabled(n))){var v=U(a.idsByKind[p.parent].keys()),y;try{for(v.s();!(y=v.n()).done;){var T=y.value;a.idsByKind[m].set(T,a.nextId),a.lookup[a.nextId]={kind:m,key:T},a.nextId++}}catch(_){v.e(_)}finally{v.f()}}}}catch(_){g.e(_)}finally{g.f()}}function bh(a,n){var t=U(ht),e;try{for(t.s();!(e=t.n()).done;){var r=e.value,i=a[r],o=n[r];if(i!==o){if(i.size!==o.size)return!1;var s=U(i),l;try{for(s.s();!(l=s.n()).done;){var u=oe(l.value,2),d=u[0],h=u[1];if(o.get(d)!==h)return!1}}catch(f){s.e(f)}finally{s.f()}}}}catch(f){t.e(f)}finally{t.f()}return!0}function Po(a,n,t){_e[n.kind].setHover(a,n.key,t)}function yh(a,n){return _e[n.kind].getCursor(a,n.key)}function ln(a,n){return"".concat(n).concat(_e[a].eventSuffix)}function un(a,n){return H(H({},n),{},E({},_e[a.kind].payloadKey,a.key))}function xh(a,n,t){var e=_e[n.kind].resolveForVerb;return e?e(n,t,a):n}function _h(a,n){var t=_e[n.kind].isHitValid;return t?t(a,n.key):!0}function tr(a){return ht.filter(function(n){return n===a||_e[n].parent===a})}function Ho(a,n,t){return n&&(n=xh(a,n,t)),n&&!_h(a,n)&&(n=null),n}function nr(a,n,t){var e=t==="wheel"?a.stateManager.hovered:a.getHitAtPosition(n);return Ho(a,e,t)}function Th(a,n){return!a||!n?a===n:a.kind===n.kind&&a.key===n.key}function $o(a,n,t){var e={event:t,preventSigmaDefault:function(){return t.preventSigmaDefault()}},r=a.dragManager.session,i=r?{kind:"node",key:r.node}:Ho(a,n,"enter"),o=a.stateManager,s=o.hovered;Th(s,i)||(s&&(Po(a,s,!1),a.emit(ln(s.kind,"leave"),un(s,e))),o.setHovered(i),i&&(Po(a,i,!0),a.emit(ln(i.kind,"enter"),un(i,e))),a.updateContainerCursor())}function Sh(a,n,t,e){e.handleResize=function(){return a.scheduleRefresh()},window.addEventListener("resize",e.handleResize),e.handleMove=function(i){a.hoverResolver.pointerMoved(lt(i))},e.handleMoveBody=function(i){var o=lt(i),s=a.dragManager;if(s.pendingNode&&!s.session){var l=a.nodeStyleAnalysis,u=l.xAttribute,d=l.yAttribute,h=a.settings;s.start(s.pendingNode,o,h.getDraggedNodes,u||"x",d||"y"),s.pendingNode=null}s.session&&(s.applyMove(o,a.settings.dragPositionToAttributes),a.emit("nodeDrag",{node:s.session.node,allDraggedNodes:s.session.allNodes,event:o}),o.preventSigmaDefault()),a.emit("moveBody",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})},e.handleLeave=function(i){var o=lt(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}};a.hoverResolver.pointerLeft(),$o(a,null,o),a.emit("leaveStage",s)},e.handleEnter=function(i){var o=lt(i);a.emit("enterStage",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})};var r=function(o){return function(s){var l=lt(s),u={event:l,preventSigmaDefault:function(){return l.preventSigmaDefault()}},d=nr(a,l,o);if(d){a.emit(ln(d.kind,o),un(d,u));return}a.emit("".concat(o,"Stage"),u)}};e.handleClick=r("click"),e.handleRightClick=r("rightClick"),e.handleDoubleClick=r("doubleClick"),e.handleWheel=r("wheel"),e.handleDown=function(i){var o=lt(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=nr(a,o,"down");if(l){l.kind==="node"&&a.settings.enableNodeDrag&&(a.dragManager.pendingNode=l.key),a.emit(ln(l.kind,"down"),un(l,s));return}a.emit("downStage",s)},e.handleUp=function(i){var o=lt(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=a.dragManager.end();l&&a.emit("nodeDragEnd",H({node:l.node,allDraggedNodes:l.allNodes},s));var u=nr(a,o,"up");if(u){a.emit(ln(u.kind,"up"),un(u,s));return}a.emit("upStage",s)},n.on("mousemove",e.handleMove),n.on("mousemovebody",e.handleMoveBody),n.on("click",e.handleClick),n.on("rightClick",e.handleRightClick),n.on("doubleClick",e.handleDoubleClick),n.on("wheel",e.handleWheel),n.on("mousedown",e.handleDown),n.on("mouseup",e.handleUp),n.on("mouseleave",e.handleLeave),n.on("mouseenter",e.handleEnter),t.on("touchdown",e.handleDown),t.on("touchdown",e.handleMove),t.on("touchup",e.handleUp),t.on("touchmove",e.handleMove),t.on("tap",e.handleClick),t.on("doubletap",e.handleDoubleClick),t.on("touchmove",e.handleMoveBody)}function Eh(a,n){var t=a.graph,e=new Set(["x","y","zIndex","type"]);n.eachNodeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=!o||o.some(function(l){return e.has(l)});a.refresh({partialGraph:{nodes:t.nodes()},skipIndexation:!s,schedule:!0})},n.eachEdgeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=o&&["zIndex","type"].some(function(l){return o?.includes(l)});a.refresh({partialGraph:{edges:t.edges()},skipIndexation:!s,schedule:!0})},n.addNodeGraphUpdate=function(r){a.addNode(r.key),a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.updateNodeGraphUpdate=function(r){a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.dropNodeGraphUpdate=function(r){a.removeNode(r.key),a.refresh({schedule:!0})},n.addEdgeGraphUpdate=function(r){var i=r.key;a.edgeGroups.register(i),a.addEdge(i);var o=a.edgeGroups.getSiblings(i),s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;a.addEdge(u)}}catch(d){s.e(d)}finally{s.f()}a.refresh({partialGraph:{edges:[i].concat(J(o))},schedule:!0})},n.updateEdgeGraphUpdate=function(r){a.refresh({partialGraph:{edges:[r.key]},skipIndexation:!1,schedule:!0})},n.dropEdgeGraphUpdate=function(r){var i=r.key,o=a.edgeGroups.getSiblings(i);a.edgeGroups.unregister(i),a.removeEdge(i);var s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;a.addEdge(u)}}catch(d){s.e(d)}finally{s.f()}a.refresh({schedule:!0})},n.clearEdgesGraphUpdate=function(){a.clearEdgeState(),a.clearEdgeIndices(),a.refresh({schedule:!0})},n.clearGraphUpdate=function(){a.clearEdgeState(),a.clearNodeState(),a.clearEdgeIndices(),a.clearNodeIndices(),a.refresh({schedule:!0})},t.on("nodeAdded",n.addNodeGraphUpdate),t.on("nodeDropped",n.dropNodeGraphUpdate),t.on("nodeAttributesUpdated",n.updateNodeGraphUpdate),t.on("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),t.on("edgeAdded",n.addEdgeGraphUpdate),t.on("edgeDropped",n.dropEdgeGraphUpdate),t.on("edgeAttributesUpdated",n.updateEdgeGraphUpdate),t.on("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),t.on("edgesCleared",n.clearEdgesGraphUpdate),t.on("cleared",n.clearGraphUpdate)}function Ah(a,n){a.removeListener("nodeAdded",n.addNodeGraphUpdate),a.removeListener("nodeDropped",n.dropNodeGraphUpdate),a.removeListener("nodeAttributesUpdated",n.updateNodeGraphUpdate),a.removeListener("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),a.removeListener("edgeAdded",n.addEdgeGraphUpdate),a.removeListener("edgeDropped",n.dropEdgeGraphUpdate),a.removeListener("edgeAttributesUpdated",n.updateEdgeGraphUpdate),a.removeListener("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),a.removeListener("edgesCleared",n.clearEdgesGraphUpdate),a.removeListener("cleared",n.clearGraphUpdate)}var Dh={x:.5,y:.5,angle:0,ratio:1},wh=8,Rh=.001;function Lh(a){var n=a.coords,t=a.nodeData,e=a.dimensions,r=a.stagePadding,i=a.zoomToSizeRatioFunction,o=a.itemSizesReference,s=a.fitLabels,l=a.nodeLabelBox,u=Object.keys(n);if(!u.length)return a.extent;for(var d=i(1)||1,h=o==="positions",f=e.width,c=e.height,g=a.extent,b=0;b<wh;b++){for(var m=Bn(g),p={width:g.x[1]-g.x[0]||1,height:g.y[1]-g.y[0]||1},v=je(Dh,e,p,r),y=Fo(v,m({x:0,y:0}),f,c),T=Fo(v,m({x:1,y:0}),f,c),_=Math.hypot(T.x-y.x,T.y-y.y)||1,S=1/0,D=-1/0,w=1/0,k=-1/0,L=0,N=u.length;L<N;L++){var R=u[L],z=t[R],C=z.size/d*(h?_:1),I=-C,B=C,F=-C,x=C;if(s){var P=l(z,C);P&&(P.minX<I&&(I=P.minX),P.maxX>B&&(B=P.maxX),P.minY<F&&(F=P.minY),P.maxY>x&&(x=P.maxY))}var W=n[R],G=W.x,A=W.y;S=Math.min(S,G+I/_),D=Math.max(D,G+B/_),w=Math.min(w,A-x/_),k=Math.max(k,A-F/_)}var q={x:[S,D],y:[w,k]},V=Math.max(q.x[1]-q.x[0],q.y[1]-q.y[0])||1,Y=Math.max(Math.abs(q.x[0]-g.x[0]),Math.abs(q.x[1]-g.x[1]),Math.abs(q.y[0]-g.y[0]),Math.abs(q.y[1]-g.y[1]));if(g=q,Y/V<Rh)break}return g}function Fo(a,n,t,e){var r=Tt(a,n);return{x:(1+r.x)*t/2,y:(1-r.y)*e/2}}var Ch=typeof navigator<"u"&&/Mac|iP(hone|ad|od)/.test(navigator.platform),kh=1500,Ph=`.sigma-gesture-hint {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 1em;
  background: #00000066;
  color: #ffffff;
  font-size: 1em;
  pointer-events: none;
  transition: opacity 0.1s;
}`;function Io(a){var n,t,e=Rt("style");e.textContent=a,document.head.appendChild(e);var r=((n=(t=e.sheet)===null||t===void 0?void 0:t.cssRules.length)!==null&&n!==void 0?n:0)>0;return e.remove(),r}var ut=null;function Fh(){return ut===null&&(ut=Ph,Io("@scope {}")&&(ut=`@scope {
`.concat(ut,`
}`)),Io("@layer sigma-gesture-hint {}")&&(ut=`@layer sigma-gesture-hint {
`.concat(ut,`
}`))),ut}var Ih=(function(){function a(n){ne(this,a),E(this,"hideTimeout",null),this.styleElement=Rt("style"),this.styleElement.textContent=Fh(),n.appendChild(this.styleElement),this.element=Rt("div",{opacity:"0"},{class:"sigma-gesture-hint"}),n.appendChild(this.element)}return ae(a,[{key:"show",value:function(t,e){var r=this,i=t==="touch"?e.sharedGestureTouchMessage:Ch?e.sharedGestureAppleWheelMessage:e.sharedGestureWheelMessage;i&&(this.element.textContent=i,this.element.style.opacity="1",typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.hideTimeout=window.setTimeout(function(){r.hideTimeout=null,r.element.style.opacity="0"},kh))}},{key:"kill",value:function(){typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.styleElement.remove(),this.element.remove()}}])})(),Gh=(function(){function a(n){ne(this,a),E(this,"readBuffer",null),E(this,"fence",null),E(this,"result",new Uint8Array(4)),E(this,"generation",0),E(this,"readGeneration",0),E(this,"lastEvent",null),E(this,"active",!1),E(this,"dirty",!1),E(this,"rafId",null),E(this,"killed",!1),this.options=n}return ae(a,[{key:"pointerMoved",value:function(t){this.lastEvent=t,this.active=!0,!this.options.isRenderPending()&&this.request()}},{key:"pointerLeft",value:function(){this.active=!1,this.dirty=!1}},{key:"frameRendered",value:function(){this.request()}},{key:"invalidate",value:function(){this.generation++}},{key:"request",value:function(){if(!(this.killed||!this.active||!this.lastEvent)){if(this.fence){this.dirty=!0;return}this.startRead()}}},{key:"startRead",value:function(){var t=this.options,e=t.gl,r=t.getFrameBuffer,i=t.getPixelRatio,o=t.getDownSizingRatio;this.readGeneration=this.generation;var s=this.lastEvent,l=va(e,s.x,s.y,i(),o()),u=oe(l,2),d=u[0],h=u[1];this.readBuffer||(this.readBuffer=e.createBuffer()),e.bindFramebuffer(e.FRAMEBUFFER,r()),e.bindBuffer(e.PIXEL_PACK_BUFFER,this.readBuffer),e.bufferData(e.PIXEL_PACK_BUFFER,4,e.STREAM_READ),e.readPixels(d,h,1,1,e.RGBA,e.UNSIGNED_BYTE,0),e.bindBuffer(e.PIXEL_PACK_BUFFER,null),e.bindFramebuffer(e.FRAMEBUFFER,null),this.fence=e.fenceSync(e.SYNC_GPU_COMMANDS_COMPLETE,0),e.flush(),this.schedulePoll(s)}},{key:"schedulePoll",value:function(t){var e=this;this.rafId=requestAnimationFrame(function(){e.rafId=null,e.poll(t)})}},{key:"poll",value:function(t){var e=this.options,r=e.gl,i=e.onIndex;if(!(this.killed||!this.fence)){var o=r.clientWaitSync(this.fence,0,0);if(o===r.TIMEOUT_EXPIRED){this.schedulePoll(t);return}if(r.deleteSync(this.fence),this.fence=null,o!==r.WAIT_FAILED&&this.readGeneration===this.generation){r.bindBuffer(r.PIXEL_PACK_BUFFER,this.readBuffer),r.getBufferSubData(r.PIXEL_PACK_BUFFER,0,this.result),r.bindBuffer(r.PIXEL_PACK_BUFFER,null);var s=oe(this.result,4),l=s[0],u=s[1],d=s[2],h=s[3];this.active&&i(ma(l,u,d,h),t)}this.dirty&&(this.dirty=!1,this.active&&this.lastEvent&&!this.options.isRenderPending()&&this.startRead())}}},{key:"reset",value:function(){this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null,this.fence=null,this.readBuffer=null,this.dirty=!1}},{key:"kill",value:function(){this.killed=!0,this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null;var t=this.options.gl;this.fence&&t.deleteSync(this.fence),this.fence=null,this.readBuffer&&t.deleteBuffer(this.readBuffer),this.readBuffer=null}}])})(),Go=(function(){function a(n,t){ne(this,a),this.key=n,this.size=t}return ae(a,null,[{key:"compare",value:function(t,e){return t.size>e.size?-1:t.size<e.size||t.key>e.key?1:-1}}])})(),na=(function(){function a(){ne(this,a),E(this,"width",0),E(this,"height",0),E(this,"cellSize",0),E(this,"columns",0),E(this,"rows",0),E(this,"cells",{})}return ae(a,[{key:"resizeAndClear",value:function(t,e){this.width=t.width,this.height=t.height,this.cellSize=e,this.columns=Math.ceil(t.width/e),this.rows=Math.ceil(t.height/e),this.cells={}}},{key:"getIndex",value:function(t){var e=Math.floor(t.x/this.cellSize),r=Math.floor(t.y/this.cellSize);return r*this.columns+e}},{key:"add",value:function(t,e,r){var i=new Go(t,e),o=this.getIndex(r),s=this.cells[o];s||(s=[],this.cells[o]=s),s.push(i)}},{key:"organize",value:function(){for(var t in this.cells){var e=this.cells[t];e.sort(Go.compare)}}},{key:"getLabelsToDisplay",value:function(t,e,r){var i=this.cellSize*this.cellSize,o=i/t/t,s=o*e/i,l=Math.ceil(s),u=[];if(r)for(var d=Math.max(0,Math.floor(r.x1/this.cellSize)),h=Math.min(this.columns-1,Math.floor(r.x2/this.cellSize)),f=Math.max(0,Math.floor(r.y1/this.cellSize)),c=Math.min(this.rows-1,Math.floor(r.y2/this.cellSize)),g=f;g<=c;g++)for(var b=d;b<=h;b++){var m=g*this.columns+b,p=this.cells[m];if(p)for(var v=0;v<Math.min(l,p.length);v++)u.push(p[v].key)}else for(var y in this.cells)for(var T=this.cells[y],_=0;_<Math.min(l,T.length);_++)u.push(T[_].key);return u}}])})();function Nh(a){var n=a.graph,t=a.hoveredNode,e=a.highlightedNodes,r=a.displayedNodeLabels,i=new Set,o=new Set(e);t&&o.add(t);var s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;n.hasNode(u)&&n.forEachEdge(u,function(c){i.add(c)})}}catch(c){s.e(c)}finally{s.f()}var d=U(r),h;try{var f=function(){var g=h.value;if(o.has(g))return 1;n.forEachEdge(g,function(b,m,p,v){var y=p===g?v:p;r.has(y)&&i.add(b)})};for(d.s();!(h=d.n()).done;)f()}catch(c){d.e(c)}finally{d.f()}return Array.from(i)}var Nt=150,zt=50,zh={both:0,node:1,label:2},Mh={over:0,above:1,below:2,auto:3},No=12,Bh=3,Wh=(function(){function a(n){ne(this,a),E(this,"labelGrid",new na),E(this,"edgeAnchorGrid",new na),E(this,"displayedNodeLabels",new Set),E(this,"displayedEdgeLabels",new Set),E(this,"edgeLabelCandidates",[]),E(this,"renderedNodeLabels",new Set),E(this,"labelSizeCache",new Map),E(this,"framePassPoints",new Float32Array(0)),E(this,"labelsDirty",!0),this.internals=n}return ae(a,[{key:"resetFrame",value:function(){this.displayedNodeLabels=new Set,this.renderedNodeLabels=new Set,this.labelSizeCache.clear()}},{key:"clearEdgeLabels",value:function(){this.displayedEdgeLabels=new Set}},{key:"resetLabelGrid",value:function(){this.labelGrid=new na,this.edgeAnchorGrid=new na}},{key:"processWebGLLabels",value:function(t){for(var e,r=this.internals,i=r.labelProgram,o=r.primitives,s=r.nodeDataCache,l=(o==null||(e=o.nodes)===null||e===void 0||(e=e.label)===null||e===void 0||(e=e.font)===null||e===void 0?void 0:e.family)||"sans-serif",u=new Map,d=0,h=t.length;d<h;d++){var f=t[d],c=s[f];if(!(c.visibility==="hidden"||!c.label)){var g=c.labelFont||l,b=u.get(g);b?b.push(c.label):u.set(g,[c.label])}}var m=U(u),p;try{for(m.s();!(p=m.n()).done;){var v=oe(p.value,2),y=v[0],T=v[1],_=On(y),S=_.family,D=_.weight,w=_.style,k=i.registerFont(S,D,w);i.ensureGlyphsReady(T,k)}}catch(L){m.e(L)}finally{m.f()}}},{key:"measureNodeLabel",value:function(t){var e,r;if(!t.label)return{width:0,height:0,textHeight:0};var i=this.internals,o=i.labelProgram,s=i.primitives,l=(e=t.labelSize)!==null&&e!==void 0?e:14,u=t.labelFont||(s==null||(r=s.nodes)===null||r===void 0||(r=r.label)===null||r===void 0||(r=r.font)===null||r===void 0?void 0:r.family)||"sans-serif",d="".concat(t.label,"|").concat(l,"|").concat(u);if(this.labelSizeCache.has(d))return this.labelSizeCache.get(d);var h=On(u),f=h.family,c=h.weight,g=h.style,b=o.registerFont(f,c,g),m=o.measureLabel(t.label,l,b);return this.labelSizeCache.set(d,m),m}},{key:"nodeLabelBox",value:function(t,e){var r,i,o,s,l;if(t.visibility==="hidden"||t.labelVisibility==="hidden")return null;var u=this.measureNodeLabel(t),d=u.width,h=u.height,f=u.textHeight;if(!d)return null;var c=(r=(i=this.internals.primitives)===null||i===void 0||(i=i.nodes)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:kt,g=e+c,b=t.labelBackgroundColor?(o=t.labelBackgroundPadding)!==null&&o!==void 0?o:Oa:0,m=d/2,p=h/2,v=m+b,y=p+b,T=f/2,_=0,S=0;switch((s=t.labelPosition)!==null&&s!==void 0?s:"right"){case"left":_=-(g+m);break;case"above":S=-(g+T);break;case"below":S=g+T;break;case"over":break;default:_=g+m}var D=(l=t.labelAngle)!==null&&l!==void 0?l:0,w=pa({x:_,y:S},D),k=w.x,L=w.y,N=Math.abs(Math.cos(D)),R=Math.abs(Math.sin(D)),z=N*v+R*y,C=R*v+N*y;return{minX:k-z,maxX:k+z,minY:L-C,maxY:L+C}}},{key:"computeDisplayedNodeLabels",value:function(){var t=this.internals.getCameraState(),e=this.internals.getDimensions(),r=e.width,i=e.height,o=this.internals.viewportToFramedGraph({x:-Nt,y:-zt}),s=this.internals.viewportToFramedGraph({x:r+Nt,y:-zt}),l=this.internals.viewportToFramedGraph({x:-Nt,y:i+zt}),u=this.internals.viewportToFramedGraph({x:r+Nt,y:i+zt}),d=Math.min(o.x,s.x,l.x,u.x),h=Math.max(o.x,s.x,l.x,u.x),f=Math.min(o.y,s.y,l.y,u.y),c=Math.max(o.y,s.y,l.y,u.y),g=je({x:.5,y:.5,ratio:1,angle:0},{width:r,height:i},this.internals.getGraphDimensions(),this.internals.getStagePadding()),b=function(P){var W=Tt(g,P);return{x:(1+W.x)*r/2,y:(1-W.y)*i/2}},m=b({x:d,y:f}),p=b({x:h,y:f}),v=b({x:d,y:c}),y=b({x:h,y:c}),T={x1:Math.min(m.x,p.x,v.x,y.x),y1:Math.min(m.y,p.y,v.y,y.y),x2:Math.max(m.x,p.x,v.x,y.x),y2:Math.max(m.y,p.y,v.y,y.y)},_=this.internals,S=_.settings,D=_.nodeDataCache,w=_.nodesWithForcedLabels,k=this.labelGrid.getLabelsToDisplay(t.ratio,S.labelDensity,T);ka(k,w);for(var L=0,N=k.length;L<N;L++){var R=k[L],z=D[R];if(!this.displayedNodeLabels.has(R)&&!(z.visibility==="hidden"||z.labelVisibility==="hidden")&&z.label&&!(z.x<d||z.x>h||z.y<f||z.y>c)){var C=this.internals.framedGraphToViewport(z),I=C.x,B=C.y,F=this.internals.scaleSize(z.size);!qe(z)&&F<S.labelRenderedSizeThreshold||I<-Nt-F||I>r+Nt+F||B<-zt-F||B>i+zt+F||this.displayedNodeLabels.add(R)}}}},{key:"buildFramePassPoints",value:function(){var t=this.internals,e=t.nodeDataCache,r=t.nodeDataTexture;if(!r)return{data:this.framePassPoints,count:0};var i=this.displayedNodeLabels.size*3;this.framePassPoints.length<i&&(this.framePassPoints=new Float32Array(i));var o=this.framePassPoints,s=0,l=U(this.displayedNodeLabels),u;try{for(l.s();!(u=l.n()).done;){var d,h,f=u.value,c=e[f];if(c){var g=r.getIndex(f);g<0||(o[s*3]=g,o[s*3+1]=(d=Ct[c.labelPosition||Oe.labelPosition])!==null&&d!==void 0?d:0,o[s*3+2]=(h=c.labelAngle)!==null&&h!==void 0?h:0,s++)}}}catch(b){l.e(b)}finally{l.f()}return{data:o,count:s}}},{key:"renderWebGLLabels",value:function(t,e){var r=this.internals,i=r.nodeDataCache,o=r.labelProgram,s=r.primitives,l=r.nodeDataTexture,u=[],d=U(this.displayedNodeLabels),h;try{for(d.s();!(h=d.n()).done;){var f=h.value;if(!this.renderedNodeLabels.has(f)){var c=i[f];e&&c.labelDepth!==e||(this.renderedNodeLabels.add(f),u.push(f))}}}catch(se){d.e(se)}finally{d.f()}if(u.length!==0){if(this.labelsDirty){for(var g,b,m,p=0,v=0,y=u.length;v<y;v++){var T=i[u[v]];p+=T.label.length}o.reallocate(p);for(var _=14,S=(g=s==null||(b=s.nodes)===null||b===void 0||(b=b.label)===null||b===void 0?void 0:b.margin)!==null&&g!==void 0?g:kt,D=Oe.labelPosition,w=(s==null||(m=s.nodes)===null||m===void 0||(m=m.label)===null||m===void 0||(m=m.font)===null||m===void 0?void 0:m.family)||"sans-serif",k=new Map,L=0,N=0,R=u.length;N<R;N++){var z,C,I,B,F=u[N],x=i[F],P=x.labelFont||w,W=k.get(P);if(W===void 0){var G=On(P),A=G.family,q=G.weight,V=G.style;W=o.registerFont(A,q,V),k.set(P,W)}var Y={text:x.label,x:x.x,y:x.y,size:(z=x.labelSize)!==null&&z!==void 0?z:_,color:x.labelColor,nodeSize:x.size,margin:S,position:(C=x.labelPosition)!==null&&C!==void 0?C:D,hidden:!1,forceLabel:qe(x),type:"default",zIndex:(I=x.zIndex)!==null&&I!==void 0?I:0,parentType:"node",parentKey:F,fontKey:W,labelAngle:(B=x.labelAngle)!==null&&B!==void 0?B:0,nodeIndex:l.getIndex(F)},pe=o.processLabel(F,L,Y);L+=pe}o.invalidateBuffers()}o.render(t)}}},{key:"renderBackdrops",value:function(t,e){var r=this.internals,i=r.backdropProgram,o=r.nodeDataCache,s=r.nodesWithBackdrop,l=r.attachmentManager,u=r.pixelRatio,d=r.nodeDataTexture,h=[],f=U(s),c;try{for(f.s();!(c=f.n()).done;){var g,b=c.value,m=o[b];if(!(!m||m.visibility==="hidden")&&!(e&&m.depth!==e)){var p=(g=d?.getIndex(b))!==null&&g!==void 0?g:-1;p<0||h.push({key:b,nodeIndex:p})}}}catch(K){f.e(K)}finally{f.f()}if(h.length!==0){i.reallocate(h.length);for(var v=0;v<h.length;v++){var y,T,_,S,D,w,k,L,N=h[v],R=N.key,z=N.nodeIndex,C=o[R],I=this.displayedNodeLabels.has(R),B=I?this.measureNodeLabel(C):{width:0,height:0,textHeight:0},F=B.textHeight,x=B.width,P=B.height,W=0,G=0;if(I&&C.labelAttachment&&l){var A=l.getEntry(R,C.labelAttachment);if(A){var q=C.labelAttachmentPlacement||"below";if(q==="below"||q==="above"){var V=A.height/u;P+=V+Ze,x=Math.max(x,A.width/u),G=q==="below"?(V+Ze)/2:-(V+Ze)/2}else{var Y=A.width/u;x+=Y+Ze,P=Math.max(P,A.height/u),W=q==="right"?(Y+Ze)/2:-(Y+Ze)/2}}}var pe=C.backdropColor?Xe(C.backdropColor):[255,255,255,255],se=C.backdropShadowColor?Xe(C.backdropShadowColor):[0,0,0,128],Z=pe.map(function(K){return K/255}),ue=se.map(function(K){return K/255}),ve=(y=C.backdropShadowBlur)!==null&&y!==void 0?y:12,De=(T=C.backdropPadding)!==null&&T!==void 0?T:6,Te=C.backdropBorderColor?Xe(C.backdropBorderColor):[0,0,0,0],Mt=Te.map(function(K){return K/255}),Me=(_=C.backdropBorderWidth)!==null&&_!==void 0?_:0,O=(S=C.backdropCornerRadius)!==null&&S!==void 0?S:0,X=(D=C.backdropLabelPadding)!==null&&D!==void 0?D:-1,ee=X<0?De:X,he=(w=zh[(k=C.backdropArea)!==null&&k!==void 0?k:"both"])!==null&&w!==void 0?w:0,Q={key:R,nodeIndex:z,label:C.label,labelWidth:x,labelHeight:P,textHeight:F,type:"default",position:C.labelPosition||Oe.labelPosition,labelAngle:(L=C.labelAngle)!==null&&L!==void 0?L:0,backdropColor:Z,backdropShadowColor:ue,backdropShadowBlur:ve,backdropPadding:De,backdropBorderColor:Mt,backdropBorderWidth:Me,backdropCornerRadius:O,backdropLabelPadding:ee,backdropArea:he,labelBoxOffset:[W,G]};i.processBackdrop(v,Q)}i.invalidateBuffers(),i.render(t)}}},{key:"renderLabelBackgrounds",value:function(t,e){var r=this.internals,i=r.labelBackgroundProgram,o=r.nodeDataCache,s=r.nodeDataTexture,l=_e.nodeLabel.writesPickingThisFrame(this.internals),u=[],d=U(this.displayedNodeLabels),h;try{for(d.s();!(h=d.n()).done;){var f=h.value,c=o[f];!c||c.visibility==="hidden"||e&&c.labelDepth!==e||!l&&!c.labelBackgroundColor||u.push(f)}}catch(z){d.e(z)}finally{d.f()}if(u.length!==0){i.reallocate(u.length);for(var g=0;g<u.length;g++){var b,m,p,v,y=u[g],T=o[y],_=(b=s?.getIndex(y))!==null&&b!==void 0?b:-1;if(!(_<0)){var S=this.measureNodeLabel(T),D=S.width,w=S.height,k=S.textHeight,L=dt(this.internals.pickingState,"nodeLabel",y)||dt(this.internals.pickingState,"node",y),N=T.labelBackgroundColor?ke(T.labelBackgroundColor):ke("transparent"),R={nodeIndex:_,id:St(L),color:N,labelWidth:D,labelHeight:w,textHeight:k,positionMode:(m=Ct[T.labelPosition||Oe.labelPosition])!==null&&m!==void 0?m:0,labelAngle:(p=T.labelAngle)!==null&&p!==void 0?p:0,padding:(v=T.labelBackgroundPadding)!==null&&v!==void 0?v:Oa};i.processLabelBackground(g,R)}}i.invalidateBuffers(),i.render(t)}}},{key:"cacheAttachments",value:function(t){var e=this.internals,r=e.attachmentManager,i=e.pixelRatio,o=e.nodeDataCache,s=e.nodesWithBackdrop,l=e.graph;if(r){var u=U(s),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;if(this.displayedNodeLabels.has(h)){var f=o[h];if(!(!f||f.visibility==="hidden")&&!(t&&f.depth!==t)&&f.labelAttachment){var c=l.getNodeAttributes(h),g=this.measureNodeLabel(f),b=g.width,m=g.height,p={node:h,attributes:c,pixelRatio:i,labelWidth:b,labelHeight:m};r.renderAttachment(h,f.labelAttachment,p)}}}}catch(v){u.e(v)}finally{u.f()}r.regenerateAtlas()}}},{key:"renderAttachments",value:function(t,e){var r=this.internals,i=r.attachmentManager,o=r.attachmentProgram,s=r.nodeDataCache,l=r.nodesWithBackdrop,u=r.pixelRatio,d=r.nodeDataTexture;if(!(!i||!o)){var h=0;o.reallocateAttachments(l.size);var f=U(l),c;try{for(f.s();!(c=f.n()).done;){var g,b,m,p,v=c.value;if(this.displayedNodeLabels.has(v)){var y=s[v];if(!(!y||y.visibility==="hidden")&&!(e&&y.labelDepth!==e)&&y.labelAttachment){var T=i.getEntry(v,y.labelAttachment);if(T){var _=(g=d?.getIndex(v))!==null&&g!==void 0?g:-1;if(!(_<0)){var S=this.measureNodeLabel(y),D=S.width,w=S.height,k=S.textHeight,L=(b=Ji[y.labelAttachmentPlacement||"below"])!==null&&b!==void 0?b:0;o.processAttachment(h,{nodeIndex:_,atlasX:T.x,atlasY:T.y,atlasW:T.width,atlasH:T.height,attachWidth:T.width/u,attachHeight:T.height/u,positionMode:(m=Ct[y.labelPosition||Oe.labelPosition])!==null&&m!==void 0?m:0,attachmentPlacement:L,labelWidth:D,labelHeight:w,textHeight:k,labelAngle:(p=y.labelAngle)!==null&&p!==void 0?p:0}),h++}}}}}}catch(N){f.e(N)}finally{f.f()}h!==0&&(o.reallocateAttachments(h),i.bindTexture(an),o.invalidateBuffers(),o.render(t))}}},{key:"computeDisplayedEdgeLabels",value:function(){var t=this.internals,e=t.graph,r=t.stateManager,i=t.settings,o=t.edgesWithForcedLabels,s=r.getHighlightedNodes(),l=i.edgeLabelAnchors==="allNodes"?new Set(this.edgeAnchorGrid.getLabelsToDisplay(this.internals.getCameraState().ratio,i.labelDensity)):this.displayedNodeLabels,u=r.hovered,d=Nh({graph:e,hoveredNode:u?.kind==="node"?u.key:null,displayedNodeLabels:l,highlightedNodes:s});ka(d,o),this.edgeLabelCandidates=d,this.displayedEdgeLabels=new Set}},{key:"filterEdgeLabelsForDepth",value:function(t){for(var e=this.internals,r=e.graph,i=e.nodeDataCache,o=e.edgeDataCache,s=[],l=new Set,u=0,d=this.edgeLabelCandidates.length;u<d;u++){var h=this.edgeLabelCandidates[u];if(!l.has(h)){l.add(h);var f=r.extremities(h),c=i[f[0]],g=i[f[1]],b=o[h];!b||!c||!g||b.visibility==="hidden"||b.labelVisibility==="hidden"||c.visibility==="hidden"||g.visibility==="hidden"||t&&b.labelDepth!==t||b.label&&s.push(h)}}return s}},{key:"renderEdgeLabels",value:function(t,e){var r=this.internals,i=r.graph,o=r.nodeDataCache,s=r.edgeDataCache,l=r.primitives,u=r.edgeLabelProgram,d=r.nodeDataTexture,h=r.edgeDataTexture;if(u){var f=this.filterEdgeLabelsForDepth(e),c=U(f),g;try{for(c.s();!(g=c.n()).done;){var b=g.value;this.displayedEdgeLabels.add(b)}}catch(Y){c.e(Y)}finally{c.f()}if(f.length!==0){if(this.labelsDirty){var m,p,v=0,y=U(f),T;try{for(y.s();!(T=y.n()).done;){var _=T.value;v+=s[_].label.length}}catch(Y){y.e(Y)}finally{y.f()}u.reallocate(v);var S=(m=l==null||(p=l.edges)===null||p===void 0||(p=p.label)===null||p===void 0?void 0:p.margin)!==null&&m!==void 0?m:5,D="over",w=0,k=U(f),L;try{for(k.s();!(L=k.n()).done;){var N,R,z=L.value,C=i.extremities(z),I=C[0],B=C[1],F=o[I],x=o[B],P=s[z],W=d.getIndex(I),G=d.getIndex(B),A=h.getIndex(z),q={text:P.label,x:(F.x+x.x)/2,y:(F.y+x.y)/2,size:No,color:P.labelColor,nodeSize:0,nodeIndex:-1,margin:S,position:(N=P.labelPosition)!==null&&N!==void 0?N:D,hidden:!1,forceLabel:qe(P),type:"default",zIndex:(R=P.zIndex)!==null&&R!==void 0?R:0,parentType:"edge",parentKey:z,fontKey:"",labelAngle:0,sourceX:F.x,sourceY:F.y,targetX:x.x,targetY:x.y,sourceSize:F.size,targetSize:x.size,sourceShape:F.shape||"circle",targetShape:x.shape||"circle",edgeSize:P.size,offset:0,edgeAttributes:P,sourceNodeIndex:W,targetNodeIndex:G,edgeIndex:A},V=u.processEdgeLabel(z,w,q);w+=V}}catch(Y){k.e(Y)}finally{k.f()}u.invalidateBuffers()}u.render(t)}}}},{key:"renderEdgeLabelBackgrounds",value:function(t,e){var r,i,o=this.internals,s=o.edgeLabelBackgroundProgram,l=o.edgeLabelProgram,u=o.edgeDataCache,d=o.primitives,h=o.edgeDataTexture;if(!(!h||!l||!s)){var f=_e.edgeLabel.writesPickingThisFrame(this.internals),c=(r=d==null||(i=d.edges)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:5,g="over",b=this.filterEdgeLabelsForDepth(e),m=[],p=U(b),v;try{for(p.s();!(v=p.n()).done;){var y=v.value;!f&&!u[y].labelBackgroundColor||m.push(y)}}catch(F){p.e(F)}finally{p.f()}if(m.length!==0){s.reallocate(m.length);for(var T=0;T<m.length;T++){var _,S,D,w=m[T],k=u[w],L=k.label,N=l.measureLabelAtlasWidth(L),R=(_=k.labelPosition)!==null&&_!==void 0?_:g,z=typeof R=="string"&&(S=Mh[R])!==null&&S!==void 0?S:0,C=dt(this.internals.pickingState,"edgeLabel",w)||dt(this.internals.pickingState,"edge",w),I=k.labelBackgroundColor?ke(k.labelBackgroundColor):ke("transparent"),B={edgeIndex:h.getIndex(w),baseFontSize:No,totalTextWidth:N,positionMode:z,margin:c,padding:(D=k.labelBackgroundPadding)!==null&&D!==void 0?D:Bh,color:I,id:St(C),edgeAttributes:k};s.processEdgeLabelBackground(T,w,B)}s.invalidateBuffers(),s.render(t)}}}}])})(),Uh=(function(){function a(n,t,e,r){ne(this,a),E(this,"nodeStates",new Map),E(this,"edgeStates",new Map),E(this,"hovered",null),E(this,"dirtyNodes",new Set),E(this,"dirtyEdges",new Set),E(this,"graphStateChanged",!1),E(this,"graphStateFlagsDirty",!1),this.scheduleRefresh=n,this.customNodeStateDefaults=t,this.customEdgeStateDefaults=e,this.customGraphStateDefaults=r,this.graphState=Aa(r)}return ae(a,[{key:"getNodeState",value:function(t){t=""+t;var e=this.nodeStates.get(t);return e||(e=Si(this.customNodeStateDefaults),this.nodeStates.set(t,e)),e}},{key:"getEdgeState",value:function(t){t=""+t;var e=this.edgeStates.get(t);return e||(e=Ei(this.customEdgeStateDefaults),this.edgeStates.set(t,e)),e}},{key:"getGraphState",value:function(){return this.flushGraphStateFlags(),this.graphState}},{key:"getHighlightedNodes",value:function(){var t=new Set,e=U(this.nodeStates),r;try{for(e.s();!(r=e.n()).done;){var i=oe(r.value,2),o=i[0],s=i[1];s.isHighlighted&&t.add(o)}}catch(l){e.e(l)}finally{e.f()}return t}},{key:"setNodeState",value:function(t,e){t=""+t;var r=this.getNodeState(t);if(wt(r,e)){var i=H(H({},r),e);this.nodeStates.set(t,i),this.dirtyNodes.add(t),this.updateHoveredNodeTracking(t,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setEdgeState",value:function(t,e){t=""+t;var r=this.getEdgeState(t);if(wt(r,e)){var i=H(H({},r),e);this.edgeStates.set(t,i),this.dirtyEdges.add(t),this.updateHoveredEdgeTracking(t,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setGraphState",value:function(t){if(wt(this.graphState,t)){var e=H(H({},this.graphState),t);e.isIdle=!e.isPanning&&!e.isZooming&&!e.isDragging,this.graphState=e,this.graphStateChanged=!0,this.scheduleRefresh()}}},{key:"setNodesState",value:function(t,e){var r=!1,i=U(t),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getNodeState(s);if(wt(l,e)){var u=H(H({},l),e);this.nodeStates.set(s,u),this.dirtyNodes.add(s),this.updateHoveredNodeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"setEdgesState",value:function(t,e){var r=!1,i=U(t),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getEdgeState(s);if(wt(l,e)){var u=H(H({},l),e);this.edgeStates.set(s,u),this.dirtyEdges.add(s),this.updateHoveredEdgeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"removeNode",value:function(t){this.nodeStates.delete(t),this.dirtyNodes.delete(t),this.clearHoveredFor("node",t)}},{key:"removeEdge",value:function(t){this.edgeStates.delete(t),this.dirtyEdges.delete(t),this.clearHoveredFor("edge",t)}},{key:"pruneNodes",value:function(t){var e=U(this.nodeStates.keys()),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;t(i)||this.removeNode(i)}}catch(o){e.e(o)}finally{e.f()}}},{key:"pruneEdges",value:function(t){var e=U(this.edgeStates.keys()),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;t(i)||this.removeEdge(i)}}catch(o){e.e(o)}finally{e.f()}}},{key:"clearNodes",value:function(){this.nodeStates.clear(),this.dirtyNodes.clear(),this.clearHoveredForKinds(tr("node"))}},{key:"clearEdges",value:function(){this.edgeStates.clear(),this.dirtyEdges.clear(),this.clearHoveredForKinds(tr("edge"))}},{key:"resetGraphState",value:function(){this.graphState=Aa(this.customGraphStateDefaults),this.graphStateChanged=!1,this.graphStateFlagsDirty=!1}},{key:"clearDirtyTracking",value:function(){this.dirtyNodes.clear(),this.dirtyEdges.clear(),this.graphStateChanged=!1}},{key:"setHovered",value:function(t){this.hovered=t}},{key:"clearHoveredFor",value:function(t,e){this.hovered&&this.hovered.key===e&&tr(t).includes(this.hovered.kind)&&(this.hovered=null)}},{key:"clearHoveredForKinds",value:function(t){this.hovered&&t.includes(this.hovered.kind)&&(this.hovered=null)}},{key:"updateGraphStateFromNodes",value:function(){var t,e=!1,r=!1,i=!1,o=U(this.nodeStates),s;try{for(o.s();!(s=o.n()).done;){var l=oe(s.value,2),u=l[1];if(u.isHovered&&(e=!0),u.isHighlighted&&(r=!0),u.isDragged&&(i=!0),e&&r&&i)break}}catch(h){o.e(h)}finally{o.f()}!e&&((t=this.hovered)===null||t===void 0?void 0:t.kind)==="edge"&&(e=!0);var d=!this.graphState.isPanning&&!this.graphState.isZooming&&!i;(this.graphState.hasHovered!==e||this.graphState.hasHighlighted!==r||this.graphState.isDragging!==i||this.graphState.isIdle!==d)&&(this.graphStateChanged=!0),this.graphState=H(H({},this.graphState),{},{hasHovered:e,hasHighlighted:r,isDragging:i,isIdle:d})}},{key:"updateGraphStateFromEdges",value:function(){var t,e=((t=this.hovered)===null||t===void 0?void 0:t.kind)==="node";if(!e){var r=U(this.edgeStates),i;try{for(r.s();!(i=r.n()).done;){var o=oe(i.value,2),s=o[1];if(s.isHovered){e=!0;break}}}catch(l){r.e(l)}finally{r.f()}}this.graphState.hasHovered!==e&&(this.graphStateChanged=!0),this.graphState=H(H({},this.graphState),{},{hasHovered:e})}},{key:"flushGraphStateFlags",value:function(){this.graphStateFlagsDirty&&(this.updateGraphStateFromNodes(),this.updateGraphStateFromEdges(),this.graphStateFlagsDirty=!1)}},{key:"updateHoveredNodeTracking",value:function(t,e,r){var i;if(e.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="node"&&this.hovered.key!==t){var s=this.hovered.key,l=this.getNodeState(s);this.nodeStates.set(s,H(H({},l),{},{isHovered:!1})),this.dirtyNodes.add(s)}this.hovered={kind:"node",key:t}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="node"&&this.hovered.key===t&&(this.hovered=null)}},{key:"updateHoveredEdgeTracking",value:function(t,e,r){var i;if(e.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="edge"&&this.hovered.key!==t){var s=this.hovered.key,l=this.getEdgeState(s);this.edgeStates.set(s,H(H({},l),{},{isHovered:!1})),this.dirtyEdges.add(s)}this.hovered={kind:"edge",key:t}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="edge"&&this.hovered.key===t&&(this.hovered=null)}}])})(),zo=1,Mo=2,Bo=3,Wo=4,Oh=(function(a){function n(t,e){var r,i,o,s,l,u=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};ne(this,n),l=fe(this,n),E(l,"nodeReducer",null),E(l,"edgeReducer",null),E(l,"stageCanvas",null),E(l,"mouseLayer",null),E(l,"gestureHint",null),E(l,"extraElements",{}),E(l,"webGLContext",null),E(l,"pickingFrameBuffer",null),E(l,"pickingTexture",null),E(l,"pickingDepthBuffer",null),E(l,"activeListeners",{}),E(l,"nodeVariableEntries",[]),E(l,"edgeVariableEntries",[]),E(l,"edgePathsByName",new Map),E(l,"nodeProgramIndex",{}),E(l,"edgeProgramIndex",{}),E(l,"edgeTextureIndexCache",{}),E(l,"nodeGraphCoords",{}),E(l,"nodeExtent",{x:[0,1],y:[0,1]}),E(l,"matrix",Ce()),E(l,"invMatrix",Ce()),E(l,"correctionRatio",1),E(l,"frameId",0),E(l,"customBBox",null),E(l,"gpuTimerExt",void 0),E(l,"activeGpuTimerQuery",null),E(l,"pendingGpuTimerQueries",[]),E(l,"normalizationFunction",Bn({x:[0,1],y:[0,1]})),E(l,"graphToViewportRatio",1),E(l,"pickingState",vh()),E(l,"prevNodeVisibilities",{}),E(l,"width",0),E(l,"height",0),E(l,"autoRescaleFrozen",!1),E(l,"stylesDeclaration",null),E(l,"resolvedStageStyle",{}),E(l,"renderFrame",null),E(l,"contextLost",!1),E(l,"pendingProcess","full"),E(l,"needToRefreshState",!1),E(l,"checkEdgesEventsFrame",null),E(l,"edgeStyleAnalysis",{dependency:"static",xAttribute:null,yAttribute:null}),E(l,"depthLayers",J(Pn)),E(l,"customLayerPrograms",new Map),E(l,"nodeShapeSlug",null),E(l,"sdfAtlas",null),E(l,"depthRanges",{nodes:{},edges:{}}),E(l,"nodeBaseDepth",{}),E(l,"edgeBaseDepth",{});var d=u.primitives,h=u.styles,f=u.settings,c=f===void 0?{}:f,g=u.nodeReducer,b=u.edgeReducer,m=u.customNodeState,p=u.customEdgeState,v=u.customGraphState;l.stateManager=new Uh(function(){return l.scheduleStateRefresh()},m,p,v);var y=d??Ca;l.stylesDeclaration=h?{nodes:(r=h.nodes)!==null&&r!==void 0?r:In.nodes,edges:(i=h.edges)!==null&&i!==void 0?i:In.edges,stage:h.stage}:In,l.nodeReducer=g??null,l.edgeReducer=b??null;var T=Gn(l.stylesDeclaration.nodes);l.nodeReducer&&(T.dependency="graph-state"),l.edgeStyleAnalysis=Gn(l.stylesDeclaration.edges),l.edgeReducer&&(l.edgeStyleAnalysis.dependency="graph-state"),l.stylesDeclaration.stage&&(l.resolvedStageStyle=La(l.stylesDeclaration.stage,l.stateManager.graphState));var _=Ii(c);if(Un(_),_.enableNodeDrag){var S=T.xAttribute,D=T.yAttribute;if((!S||!D)&&!_.dragPositionToAttributes)throw new Error('Sigma: `enableNodeDrag` is true but position attribute names could not be inferred from styles. Either use attribute bindings for x/y in your node styles (e.g. `x: { attribute: "x" }`), or provide a `dragPositionToAttributes` setting.')}if(Fi(t),!(e instanceof HTMLElement))throw new Error("Sigma: container should be an html element.");l.container=e,l.edgeGroups=new ph(t,function(x,P){for(var W=0;W<x.length;W++){var G=l.stateManager.getEdgeState(x[W]);G.parallelIndex=W,G.parallelCount=P}});var w=new gh(t,l.viewportToGraph.bind(l),l.setNodesState.bind(l),function(x,P){return l.emit(x,P)});if(l.depthLayers=(o=y.depthLayers)!==null&&o!==void 0?o:J(Pn),!(h!=null&&h.nodes)&&!Cn.every(function(x){return l.depthLayers.includes(x)}))throw new Error("Sigma: depthLayers must include ".concat(Cn.join(", ")," for the built-in node styles."));if(!(h!=null&&h.edges)&&!kn.every(function(x){return l.depthLayers.includes(x)}))throw new Error("Sigma: depthLayers must include ".concat(kn.join(", ")," for the built-in edge styles."));l.itemBuckets={nodes:new Za(l.depthLayers),edges:new Za(l.depthLayers)},l.initWebGLContext(),l.mouseLayer=Rt("div",{position:"absolute",touchAction:"none",userSelect:"none"},{class:"sigma-mouse"}),l.container.appendChild(l.mouseLayer),l.resolvedStageStyle.background&&(l.container.style.backgroundColor=l.resolvedStageStyle.background),l.resolvedStageStyle.cursor&&(l.container.style.cursor=l.resolvedStageStyle.cursor);var k=new To(l.webGLContext),L=new Qa(l.webGLContext,{channels:1}),N=new So(l.webGLContext),R=new Qa(l.webGLContext,{channels:4}),z=l.webGLContext,C=new Gh({gl:z,getFrameBuffer:function(){return l.pickingFrameBuffer},getPixelRatio:function(){return l.internals.pixelRatio},getDownSizingRatio:function(){return l.internals.settings.pickingDownSizingRatio},isRenderPending:function(){return l.renderFrame!==null},onIndex:function(P,W){var G;return $o(l.internals,(G=l.pickingState.lookup[P])!==null&&G!==void 0?G:null,W)}}),I=l.initPrograms(y,_),B=y==null||(s=y.nodes)===null||s===void 0?void 0:s.labelAttachments,F=null;return B&&Object.keys(B).length>0&&(F=new _o(z,B,function(){return l.scheduleRender()})),l.internals=H(H({nodeDataCache:{},edgeDataCache:{},nodesWithForcedLabels:new Set,nodesWithBackdrop:new Set,edgesWithForcedLabels:new Set,settings:_,primitives:y,pixelRatio:Yt(),graph:t,stateManager:l.stateManager,dragManager:w,hoverResolver:C,nodeStyleAnalysis:T,pickingState:l.pickingState},I),{},{attachmentManager:F,nodeDataTexture:k,nodeFrameTexture:L,edgeDataTexture:N,edgeFrameTexture:R,getDimensions:function(){return l.getDimensions()},getGraphDimensions:function(){return l.getGraphDimensions()},getStagePadding:function(){return l.getStagePadding()},getCameraState:function(){return l.camera.getState()},getHitAtPosition:function(P){return l.getHitAtPosition(P)},setNodeState:function(P,W){return l.setNodeState(P,W)},setEdgeState:function(P,W){return l.setEdgeState(P,W)},updateContainerCursor:function(){return l.updateContainerCursor()},scheduleRefresh:function(){return l.scheduleRefresh()},viewportToFramedGraph:function(P){return l.viewportToFramedGraph(P)},viewportToGraph:function(P){return l.viewportToGraph(P)},framedGraphToViewport:function(P){return l.framedGraphToViewport(P)},scaleSize:function(P){return l.scaleSize(P)},emit:function(P,W){return l.emit(P,W)}}),l.labelRenderer=new Wh(l.internals),l.resize(),l.initializeWebGLLabels(),l.camera=new Do,l.bindCameraHandlers(),l.mouseCaptor=new dh(l.mouseLayer,l),l.mouseCaptor.setSettings(l.internals.settings),l.touchCaptor=new fh(l.mouseLayer,l),l.touchCaptor.setSettings(l.internals.settings),l.bindEventHandlers(),l.bindGraphHandlers(),l.handleSettingsUpdate(),l.refresh(),l}return ge(n,a),ae(n,[{key:"initializeWebGLLabels",value:function(){this.sdfAtlas=new it,this.sdfAtlas.registerFont({family:"sans-serif",weight:"normal",style:"normal"})}},{key:"resetWebGLTexture",value:function(){var e=this.webGLContext;if(!this.pickingFrameBuffer)return this;var r=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),i=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);e.bindFramebuffer(e.FRAMEBUFFER,this.pickingFrameBuffer),this.pickingTexture&&e.deleteTexture(this.pickingTexture);var o=e.createTexture();o&&(e.bindTexture(e.TEXTURE_2D,o),e.texImage2D(e.TEXTURE_2D,0,e.RGBA,r,i,0,e.RGBA,e.UNSIGNED_BYTE,null),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.framebufferTexture2D(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,o,0),this.pickingTexture=o),this.pickingDepthBuffer&&e.deleteRenderbuffer(this.pickingDepthBuffer);var s=e.createRenderbuffer();return s&&(e.bindRenderbuffer(e.RENDERBUFFER,s),e.renderbufferStorage(e.RENDERBUFFER,e.DEPTH_COMPONENT16,r,i),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.DEPTH_ATTACHMENT,e.RENDERBUFFER,s),this.pickingDepthBuffer=s),e.bindFramebuffer(e.FRAMEBUFFER,null),this}},{key:"bindCameraHandlers",value:function(){var e=this;return this.activeListeners.camera=function(){e.refreshMatrices(),e.labelRenderer.labelsDirty=!0,e.scheduleRender()},this.activeListeners.cameraAnimationStart=function(r){var i=r.from,o=r.to;i.ratio!==o.ratio&&e.stateManager.setGraphState({isZooming:!0})},this.activeListeners.cameraAnimationEnd=function(){e.stateManager.setGraphState({isZooming:!1})},this.camera.on("updated",this.activeListeners.camera),this.camera.on("animationStart",this.activeListeners.cameraAnimationStart),this.camera.on("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"unbindCameraHandlers",value:function(){return this.camera.removeListener("updated",this.activeListeners.camera),this.camera.removeListener("animationStart",this.activeListeners.cameraAnimationStart),this.camera.removeListener("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"getHitAtPosition",value:function(e){var r;if(this.contextLost)return null;var i=this.webGLContext;i.bindFramebuffer(i.FRAMEBUFFER,this.pickingFrameBuffer);var o=oi(i,this.pickingFrameBuffer,e.x,e.y,this.internals.pixelRatio,this.internals.settings.pickingDownSizingRatio),s=ma.apply(void 0,J(o));return(r=this.pickingState.lookup[s])!==null&&r!==void 0?r:null}},{key:"bindEventHandlers",value:function(){return Sh(this.internals,this.mouseCaptor,this.touchCaptor,this.activeListeners),this}},{key:"bindGraphHandlers",value:function(){return Eh({graph:this.internals.graph,edgeGroups:this.edgeGroups,addNode:this.addNode.bind(this),updateNode:this.updateNode.bind(this),removeNode:this.removeNode.bind(this),addEdge:this.addEdge.bind(this),updateEdge:this.updateEdge.bind(this),removeEdge:this.removeEdge.bind(this),clearEdgeState:this.clearEdgeState.bind(this),clearNodeState:this.clearNodeState.bind(this),clearEdgeIndices:this.clearEdgeIndices.bind(this),clearNodeIndices:this.clearNodeIndices.bind(this),refresh:this.refresh.bind(this)},this.activeListeners),this}},{key:"unbindGraphHandlers",value:function(){Ah(this.internals.graph,this.activeListeners)}},{key:"getNodeShapeId",value:function(e){return this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&e.shape&&e.shape in this.internals.nodeShapeMap?this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[e.shape]]:tn(e.shape||"circle")}},{key:"processNodes",value:function(){var e=this,r=this.internals.graph,i=this.internals.settings,o=this.getDimensions(),s=i.autoRescale,l=i.autoRescaleContent,u=this.nodeExtent;if(s===!1){var d=o.width,h=o.height;u={x:[-d/2,d/2],y:[-h/2,h/2]}}else(s!=="once"||!this.autoRescaleFrozen)&&(u=this.computeNodeExtent(),l!=="positions"&&!this.customBBox&&(u=Lh({extent:u,coords:this.nodeGraphCoords,nodeData:this.internals.nodeDataCache,dimensions:o,stagePadding:this.getStagePadding(),zoomToSizeRatioFunction:i.zoomToSizeRatioFunction,itemSizesReference:i.itemSizesReference,fitLabels:l==="labels",nodeLabelBox:function(G,A){return e.labelRenderer.nodeLabelBox(G,A)}})),s==="once"&&(this.autoRescaleFrozen=!0));this.nodeExtent=u,this.normalizationFunction=Bn(this.customBBox||this.nodeExtent);var f=new Do,c=je(f.getState(),o,this.getGraphDimensions(),this.getStagePadding());this.labelRenderer.labelGrid.resizeAndClear(o,i.labelGridCellSize),this.labelRenderer.edgeAnchorGrid.resizeAndClear(o,i.labelGridCellSize);var g=i.renderEdgeLabels&&i.edgeLabelAnchors==="allNodes",b=!1;er(this.pickingState,"node");for(var m=r.nodes(),p=0,v=m.length;p<v;p++){var y=m[p],T=this.internals.nodeDataCache[y],_=this.nodeGraphCoords[y];T.x=_.x,T.y=_.y,this.normalizationFunction.applyTo(T),T.visibility!==this.prevNodeVisibilities[y]&&(b=!0),typeof T.label=="string"&&T.visibility!=="hidden"&&T.labelVisibility!=="hidden"&&this.labelRenderer.labelGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:c})),g&&T.visibility!=="hidden"&&this.labelRenderer.edgeAnchorGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:c}))}this.labelRenderer.labelGrid.organize(),this.labelRenderer.edgeAnchorGrid.organize(),this.nodeProgram.reallocate(m.length);var S=0;this.depthRanges.nodes={},this.nodeBaseDepth={};var D=this.internals.nodeDataCache,w=U(this.depthLayers),k;try{for(w.s();!(k=w.n()).done;){var L=k.value,N=this.itemBuckets.nodes.getSorted(L,function(W){return D[W].zIndex});if(N.length!==0){this.depthRanges.nodes[L]=[{offset:S,count:N.length}];var R=U(N),z;try{for(R.s();!(z=R.n()).done;){var C=z.value;this.nodeBaseDepth[C]=L,this.nodeProgram.allocateNode(C),ko(this.pickingState,"node",C),this.addNodeToProgram(C,S++)}}catch(W){R.e(W)}finally{R.f()}}}}catch(W){w.e(W)}finally{w.f()}this.nodeProgram.invalidateBuffers();for(var I=0,B=m.length;I<B;I++)this.prevNodeVisibilities[m[I]]=this.internals.nodeDataCache[m[I]].visibility;this.labelRenderer.processWebGLLabels(m);var F=U(this.customLayerPrograms.values()),x;try{for(F.s();!(x=F.n()).done;){var P=x.value.program;P.cacheData&&P.cacheData()}}catch(W){F.e(W)}finally{F.f()}return b}},{key:"processEdges",value:function(){var e=this.internals.graph,r=e.edges();this.edgeProgram.reallocate(r.length);var i=0;er(this.pickingState,"edge"),this.depthRanges.edges={},this.edgeBaseDepth={};var o=this.internals.edgeDataCache,s=U(this.depthLayers),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=this.itemBuckets.edges.getSorted(u,function(g){return o[g].zIndex});if(d.length!==0){this.depthRanges.edges[u]=[{offset:i,count:d.length}];var h=U(d),f;try{for(h.s();!(f=h.n()).done;){var c=f.value;this.edgeBaseDepth[c]=u,ko(this.pickingState,"edge",c),this.addEdgeToProgram(c,i++)}}catch(g){h.e(g)}finally{h.f()}}}}catch(g){s.e(g)}finally{s.f()}this.edgeProgram.invalidateBuffers()}},{key:"updateNodeDepthRanges",value:function(e,r,i){var o=this.nodeProgramIndex[e];o!==void 0&&(Ia(this.depthRanges.nodes,r,o),Ga(this.depthRanges.nodes,i,o))}},{key:"updateEdgeDepthRanges",value:function(e,r,i){var o=this.edgeProgramIndex[e];o!==void 0&&(Ia(this.depthRanges.edges,r,o),Ga(this.depthRanges.edges,i,o))}},{key:"handleSettingsUpdate",value:function(){var e=this,r=this.internals.settings;return this.camera.minRatio=r.minCameraRatio,this.camera.maxRatio=r.maxCameraRatio,this.camera.enabledZooming=r.enableCameraZooming,this.camera.enabledPanning=r.enableCameraPanning,this.camera.enabledRotation=r.enableCameraRotation,r.cameraPanBoundaries?this.camera.constrainState=function(i){return e.cleanCameraState(i,r.cameraPanBoundaries&&ie(r.cameraPanBoundaries)==="object"?r.cameraPanBoundaries:{})}:this.camera.constrainState=null,this.camera.setState(this.camera.getState()),this.mouseLayer.style.touchAction=r.gestureTarget==="graph"?"none":r.gestureTarget==="shared"?"pan-x pan-y":"auto",r.gestureTarget!=="shared"&&this.gestureHint&&(this.gestureHint.kill(),this.gestureHint=null),this.mouseCaptor.setSettings(this.internals.settings),this.touchCaptor.setSettings(this.internals.settings),this}},{key:"cleanCameraState",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=r.tolerance,o=i===void 0?0:i,s=r.boundaries,l=H({},e),u=s||this.nodeExtent,d=oe(u.x,2),h=d[0],f=d[1],c=oe(u.y,2),g=c[0],b=c[1],m=[this.graphToViewport({x:h,y:g},{cameraState:e}),this.graphToViewport({x:f,y:g},{cameraState:e}),this.graphToViewport({x:h,y:b},{cameraState:e}),this.graphToViewport({x:f,y:b},{cameraState:e})],p=1/0,v=-1/0,y=1/0,T=-1/0;m.forEach(function(C){var I=C.x,B=C.y;p=Math.min(p,I),v=Math.max(v,I),y=Math.min(y,B),T=Math.max(T,B)});var _=v-p,S=T-y,D=this.getDimensions(),w=D.width,k=D.height,L=0,N=0;if(_>=w?v<w-o?L=v-(w-o):p>o&&(L=p-o):v>w+o?L=v-(w+o):p<-o&&(L=p+o),S>=k?T<k-o?N=T-(k-o):y>o&&(N=y-o):T>k+o?N=T-(k+o):y<-o&&(N=y+o),L||N){var R=this.viewportToFramedGraph({x:0,y:0},{cameraState:e}),z=this.viewportToFramedGraph({x:L,y:N},{cameraState:e});L=z.x-R.x,N=z.y-R.y,l.x+=L,l.y+=N}return l}},{key:"refreshMatrices",value:function(){var e=this.camera.getState(),r=this.getDimensions(),i=this.getGraphDimensions(),o=this.getStagePadding();this.matrix=je(e,r,i,o),this.invMatrix=je(e,r,i,o,!0),this.correctionRatio=ni(this.matrix,e,r),this.graphToViewportRatio=this.getGraphToViewportRatio()}},{key:"processData",value:function(){var e;this.emit("beforeProcess"),(e=this.internals.attachmentManager)===null||e===void 0||e.clear();var r=H({},this.pickingState.idsByKind),i=this.processNodes();(this.pendingProcess==="full"||i)&&this.processEdges(),mh(this.pickingState,this.internals),bh(r,this.pickingState.idsByKind)||this.internals.hoverResolver.invalidate(),this.pendingProcess="none",this.emit("afterProcess")}},{key:"render",value:function(){var e=this;if(this.contextLost)return this;this.emit("beforeRender");var r=function(){return e.internals.hoverResolver.frameRendered(),e.emit("afterRender"),e};if(this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null),this.resize(),(this.pendingProcess!=="none"||this.needToRefreshState)&&(this.labelRenderer.labelsDirty=!0),this.pendingProcess!=="none"&&this.processData(),this.needToRefreshState&&this.refreshState(),this.needToRefreshState=!1,this.stateManager.clearDirtyTracking(),this.pendingProcess!=="none"&&this.processData(),this.clear(),this.resetWebGLTexture(),!this.internals.graph.order)return r();var i=this.mouseCaptor,o=this.camera.isAnimating()||i.isMoving||i.draggedEvents||i.currentWheelDirection;if(this.refreshMatrices(),this.frameId++,this.internals.settings.renderEdgeLabels&&!this.internals.edgeLabelProgram){var s=this.createEdgeLabelPrograms(),l=s.labelProgram,u=s.labelBackgroundProgram;this.internals.edgeLabelProgram=l,this.internals.edgeLabelBackgroundProgram=u}var d=this.internals.settings.DEBUG_logRenderStats?this.getDebugPrograms():null;d&&d.forEach(function(Y){return Y.resetDebugStats()}),this.internals.settings.DEBUG_gpuTimerQueries&&this.beginGpuTimerQuery(),this.internals.nodeFrameTexture.ensureCapacity(this.internals.nodeDataTexture.getCapacity()),this.internals.edgeFrameTexture.ensureCapacity(this.internals.edgeDataTexture.getCapacity());var h=this.getRenderParams(),f=_e.edge.writesPickingThisFrame(this.internals)?h:H(H({},h),{},{pickingFrameBuffer:null});this.labelRenderer.resetFrame();var c=this.webGLContext,g=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),b=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);if(c.bindFramebuffer(c.FRAMEBUFFER,this.pickingFrameBuffer),c.viewport(0,0,g,b),c.clear(c.COLOR_BUFFER_BIT|c.DEPTH_BUFFER_BIT),c.bindFramebuffer(c.FRAMEBUFFER,null),c.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),c.clear(c.COLOR_BUFFER_BIT|c.DEPTH_BUFFER_BIT),this.internals.nodeDataTexture.upload(),this.internals.edgeDataTexture.upload(),this.nodeProgram.uploadLayerTexture(),this.edgeProgram.uploadAttributeTexture(),this.emit("afterTexturesUpload"),this.internals.nodeDataTexture.bind(Bo),this.internals.edgeDataTexture.bind(Wo),this.edgeFramePass.run(h,this.internals.edgeFrameTexture,this.internals.edgeDataTexture.getHighWaterMark(),this.edgeProgram.getAttributeTexture(),this.nodeProgram.getAttributeTexture()),this.internals.edgeFrameTexture.bind(zo),this.internals.settings.renderLabels){this.labelRenderer.computeDisplayedNodeLabels();var m=this.labelRenderer.buildFramePassPoints(),p=m.data,v=m.count;this.nodeFramePass.run(p,v,this.internals.nodeFrameTexture,h,this.nodeProgram.getAttributeTexture()),this.internals.nodeFrameTexture.bind(Mo)}this.internals.settings.renderEdgeLabels&&this.labelRenderer.computeDisplayedEdgeLabels();var y=U(this.customLayerPrograms.values()),T;try{for(y.s();!(T=y.n()).done;){var _=T.value.program;_.preRender&&_.preRender(h)}}catch(Y){y.e(Y)}finally{y.f()}c.bindFramebuffer(c.FRAMEBUFFER,null),c.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),c.blendFunc(c.ONE,c.ONE_MINUS_SRC_ALPHA);var S=!this.internals.settings.hideLabelsOnMove||!o,D=U(this.depthLayers),w;try{for(D.s();!(w=D.n()).done;){var k=w.value,L=U(this.customLayerPrograms.values()),N;try{for(L.s();!(N=L.n()).done;){var R=N.value;R.depth===k&&R.program.render(h)}}catch(Y){L.e(Y)}finally{L.f()}var z=this.depthRanges.edges[k];if(z&&(!this.internals.settings.hideEdgesOnMove||!o)){var C=U(z),I;try{for(C.s();!(I=C.n()).done;){var B=I.value,F=B.offset,x=B.count;x>0&&this.edgeProgram.render(f,F,x)}}catch(Y){C.e(Y)}finally{C.f()}}this.internals.settings.renderEdgeLabels&&S&&(this.labelRenderer.renderEdgeLabelBackgrounds(_e.edgeLabel.writesPickingThisFrame(this.internals)?h:H(H({},h),{},{pickingFrameBuffer:null}),k),this.labelRenderer.renderEdgeLabels(h,k)),this.labelRenderer.cacheAttachments(k),this.labelRenderer.renderBackdrops(H(H({},h),{},{pickingFrameBuffer:null}),k);var P=this.depthRanges.nodes[k];if(P){var W=U(P),G;try{for(W.s();!(G=W.n()).done;){var A=G.value,q=A.offset,V=A.count;V>0&&this.nodeProgram.render(h,q,V)}}catch(Y){W.e(Y)}finally{W.f()}}S&&(this.labelRenderer.renderAttachments(H(H({},h),{},{pickingFrameBuffer:null}),k),this.labelRenderer.renderLabelBackgrounds(_e.nodeLabel.writesPickingThisFrame(this.internals)?h:H(H({},h),{},{pickingFrameBuffer:null}),k),this.internals.settings.renderLabels&&this.labelRenderer.renderWebGLLabels(h,k))}}catch(Y){D.e(Y)}finally{D.f()}return this.labelRenderer.labelsDirty=!1,this.internals.settings.DEBUG_displayPickingLayer&&(c.bindFramebuffer(c.READ_FRAMEBUFFER,this.pickingFrameBuffer),c.bindFramebuffer(c.DRAW_FRAMEBUFFER,null),c.blitFramebuffer(0,0,g,b,0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio,c.COLOR_BUFFER_BIT,c.NEAREST)),this.internals.settings.DEBUG_gpuTimerQueries&&this.endGpuTimerQuery(),this.pollGpuTimerQueries(),d&&this.logRenderStats(d),r()}},{key:"getDebugPrograms",value:function(){return[this.nodeProgram,this.edgeProgram,this.internals.labelProgram,this.internals.edgeLabelProgram,this.internals.edgeLabelBackgroundProgram,this.internals.backdropProgram,this.internals.labelBackgroundProgram,this.internals.attachmentProgram].filter(function(e){return e!==null})}},{key:"logRenderStats",value:function(e){var r={},i=U(e),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;r[s.constructor.name]=H({},s.debugStats)}}catch(l){i.e(l)}finally{i.f()}console.log("[sigma] DEBUG_logRenderStats: frame #".concat(this.frameId)),console.table(r)}},{key:"getGpuTimerExtension",value:function(){return this.gpuTimerExt===void 0&&(this.gpuTimerExt=this.webGLContext.getExtension("EXT_disjoint_timer_query_webgl2"),this.gpuTimerExt||console.warn("Sigma: DEBUG_gpuTimerQueries is enabled, but this browser/driver doesn't support EXT_disjoint_timer_query_webgl2.")),this.gpuTimerExt}},{key:"beginGpuTimerQuery",value:function(){var e=this.getGpuTimerExtension();if(e){var r=this.webGLContext,i=r.createQuery();i&&(r.beginQuery(e.TIME_ELAPSED_EXT,i),this.activeGpuTimerQuery=i)}}},{key:"endGpuTimerQuery",value:function(){!this.gpuTimerExt||!this.activeGpuTimerQuery||(this.webGLContext.endQuery(this.gpuTimerExt.TIME_ELAPSED_EXT),this.pendingGpuTimerQueries.push({query:this.activeGpuTimerQuery,frameId:this.frameId}),this.activeGpuTimerQuery=null)}},{key:"pollGpuTimerQueries",value:function(){if(this.pendingGpuTimerQueries.length!==0){var e=this.webGLContext,r=this.gpuTimerExt;if(!r){this.pendingGpuTimerQueries.forEach(function(h){var f=h.query;return e.deleteQuery(f)}),this.pendingGpuTimerQueries=[];return}var i=e.getParameter(r.GPU_DISJOINT_EXT),o=[],s=U(this.pendingGpuTimerQueries),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if(!e.getQueryParameter(u.query,e.QUERY_RESULT_AVAILABLE)){o.push(u);continue}if(!i){var d=e.getQueryParameter(u.query,e.QUERY_RESULT);console.log("[sigma] DEBUG_gpuTimerQueries: frame #".concat(u.frameId," GPU time = ").concat((d/1e6).toFixed(2),"ms"))}e.deleteQuery(u.query)}}catch(h){s.e(h)}finally{s.f()}this.pendingGpuTimerQueries=o}}},{key:"postEvaluateNode",value:function(e,r,i){var o=e;o.x===void 0&&(o.x=r.x),o.y===void 0&&(o.y=r.y),e.highlighted=i.isHighlighted;for(var s=0,l=this.nodeVariableEntries.length;s<l;s++){var u,d,h=oe(this.nodeVariableEntries[s],2),f=h[0],c=h[1];o[f]=(u=(d=o[f])!==null&&d!==void 0?d:r[f])!==null&&u!==void 0?u:c.default}}},{key:"addNode",value:function(e){var r=this.internals.graph.getNodeAttributes(e),i=this.stateManager.getNodeState(e),o=this.internals.nodeDataCache[e]||{};if(Nn(this.stylesDeclaration.nodes,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateNode(o,r,i),this.nodeReducer){var s=this.nodeReducer(e,o,r,i,this.stateManager.graphState,this.internals.graph);o=H(H({},o),s)}if(typeof o.x!="number"||typeof o.y!="number")throw new Error('Sigma: could not find a valid position (x, y) for node "'.concat(e,'". ')+"Provide coordinates via node attributes, styles, or a nodeReducer.");this.internals.nodeShapeMap?(!o.shape||!(o.shape in this.internals.nodeShapeMap))&&(o.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(o.shape=this.nodeShapeSlug),this.internals.nodeDataCache[e]=o,this.nodeGraphCoords[e]={x:o.x,y:o.y},rt(this.internals.nodesWithForcedLabels,e,qe(o)),rt(this.internals.nodesWithBackdrop,e,Fa(o)),this.itemBuckets.nodes.set(e,o.depth)}},{key:"updateNode",value:function(e){this.addNode(e);var r=this.internals.nodeDataCache[e];this.normalizationFunction.applyTo(r)}},{key:"removeNode",value:function(e){this.itemBuckets.nodes.remove(e),delete this.internals.nodeDataCache[e],delete this.nodeGraphCoords[e],delete this.nodeProgramIndex[e],this.internals.dragManager.removeNode(e),this.stateManager.removeNode(e),this.internals.nodesWithForcedLabels.delete(e),this.internals.nodesWithBackdrop.delete(e)}},{key:"postEvaluateEdge",value:function(e,r){for(var i=e,o=0,s=this.edgeVariableEntries.length;o<s;o++){var l,u,d=oe(this.edgeVariableEntries[o],2),h=d[0],f=d[1];i[h]=(l=(u=i[h])!==null&&u!==void 0?u:r[h])!==null&&l!==void 0?l:f.default}}},{key:"applyEdgeSpread",value:function(e,r,i){var o;if(!(i.parallelCount<=1)){var s=this.internals.graph.source(e),l=this.internals.graph.target(e),u=s===l,d=u?r.selfLoopPath||r.path:r.parallelPath||r.path,h=d?this.edgePathsByName.get(d):void 0;if(h!=null&&h.spread){var f=(o=r.parallelSpread)!==null&&o!==void 0?o:.25,c=h.spread.compute(i.parallelIndex,i.parallelCount,f);!u&&this.internals.graph.isDirected(e)&&s>l&&(c=-c),r[h.spread.variable]=c}}}},{key:"addEdge",value:function(e){var r=this.internals.graph.getEdgeAttributes(e),i=this.stateManager.getEdgeState(e),o={};if(zn(this.stylesDeclaration.edges,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateEdge(o,r),this.edgeReducer){var s=this.edgeReducer(e,o,r,i,this.stateManager.graphState,this.internals.graph);o=H(H({},o),s)}this.applyEdgeSpread(e,o,i),this.internals.edgeDataCache[e]=o,rt(this.internals.edgesWithForcedLabels,e,qe(o)),this.itemBuckets.edges.set(e,o.depth)}},{key:"updateEdge",value:function(e){this.addEdge(e)}},{key:"removeEdge",value:function(e){this.itemBuckets.edges.remove(e),delete this.internals.edgeDataCache[e],delete this.edgeProgramIndex[e],delete this.edgeTextureIndexCache[e],this.internals.edgeDataTexture.free(e),this.stateManager.removeEdge(e),this.internals.edgesWithForcedLabels.delete(e)}},{key:"clearNodeIndices",value:function(){this.labelRenderer.resetLabelGrid(),this.autoRescaleFrozen||(this.nodeExtent={x:[0,1],y:[0,1]}),this.internals.nodeDataCache={},this.nodeGraphCoords={},this.edgeProgramIndex={},this.internals.nodesWithForcedLabels.clear(),this.internals.nodesWithBackdrop.clear(),this.prevNodeVisibilities={},this.itemBuckets.nodes.clearAll(),this.depthRanges.nodes={},this.nodeBaseDepth={}}},{key:"clearEdgeIndices",value:function(){this.internals.edgeDataCache={},this.edgeProgramIndex={},this.edgeTextureIndexCache={},this.internals.edgesWithForcedLabels.clear(),er(this.pickingState,"edge"),this.itemBuckets.edges.clearAll(),this.depthRanges.edges={},this.edgeBaseDepth={},this.edgeGroups.clear()}},{key:"clearIndices",value:function(){this.clearEdgeIndices(),this.clearNodeIndices()}},{key:"clearNodeState",value:function(){this.labelRenderer.resetFrame(),this.internals.nodesWithBackdrop.clear(),this.internals.dragManager.clear(),this.autoRescaleFrozen=!1,this.stateManager.clearNodes()}},{key:"clearEdgeState",value:function(){this.labelRenderer.clearEdgeLabels(),this.stateManager.clearEdges()}},{key:"clearState",value:function(){this.clearEdgeState(),this.clearNodeState(),this.stateManager.resetGraphState()}},{key:"addNodeToProgram",value:function(e,r){var i,o=this.internals.nodeDataCache[e];this.internals.nodeDataTexture.allocate(e),(i=this.internals.nodeDataTexture).updateNode.apply(i,[e,o.x,o.y,o.size,this.getNodeShapeId(o)].concat(J(Mn(o)),[o.color]));var s=this.internals.nodeDataTexture.getIndex(e);this.nodeProgram.process(dt(this.pickingState,"node",e),r,o,s,e),this.nodeProgramIndex[e]=r}},{key:"addEdgeToProgram",value:function(e,r){var i,o,s=this.internals.edgeDataCache[e],l=this.internals.graph.source(e),u=this.internals.graph.target(e),d=this.internals.edgeDataTexture.allocate(e);this.edgeTextureIndexCache[e]=d;var h=l===u,f=!h&&((i=(o=this.stateManager.getEdgeState(e))===null||o===void 0?void 0:o.parallelCount)!==null&&i!==void 0?i:1)>1,c=this.edgeProgram.resolveEdgeIds(s,h,f),g=c.pathId,b=c.headId,m=c.tailId,p=c.headLengthRatio,v=c.tailLengthRatio;this.internals.edgeDataTexture.updateEdge(e,this.internals.nodeDataTexture.getIndex(l),this.internals.nodeDataTexture.getIndex(u),s.size,p,v,g,b,m),this.edgeProgram.process(dt(this.pickingState,"edge",e),r,this.internals.nodeDataCache[l],this.internals.nodeDataCache[u],s,d),this.edgeProgramIndex[e]=r}},{key:"getRenderParams",value:function(){return{frameId:this.frameId,matrix:this.matrix,invMatrix:this.invMatrix,width:this.width,height:this.height,pixelRatio:this.internals.pixelRatio,zoomRatio:this.camera.ratio,cameraAngle:this.camera.angle,sizeRatio:1/this.scaleSize(),correctionRatio:this.correctionRatio,downSizingRatio:this.internals.settings.pickingDownSizingRatio,minEdgeThickness:this.internals.settings.minEdgeThickness,antiAliasingFeather:this.internals.settings.antiAliasingFeather,nodePickingPadding:this.internals.settings.nodePickingPadding,edgePickingPadding:this.internals.settings.edgePickingPadding,labelPickingPadding:this.internals.settings.labelPickingPadding,nodeDataTextureUnit:Bo,nodeDataTextureWidth:this.internals.nodeDataTexture.getTextureWidth(),nodeFrameTextureUnit:Mo,nodeFrameTextureWidth:this.internals.nodeFrameTexture.getTextureWidth(),edgeDataTextureUnit:Wo,edgeDataTextureWidth:this.internals.edgeDataTexture.getTextureWidth(),edgeFrameTextureUnit:zo,edgeFrameTextureWidth:this.internals.edgeFrameTexture.getTextureWidth(),pickingFrameBuffer:this.pickingFrameBuffer,labelPixelSnapping:this.internals.settings.labelPixelSnapping?1:0}}},{key:"getStagePadding",value:function(){var e=this.internals.settings,r=e.stagePadding,i=e.autoRescale;return i&&r||0}},{key:"getLayerElement",value:function(e){if(e==="mouse")return this.mouseLayer;var r=this.extraElements[e];if(!r)throw new Error('Sigma: layer "'.concat(e,'" does not exist'));return r}},{key:"initWebGLContext",value:function(){var e=this,r=this.createWebGLContext("stage");this.stageCanvas=this.extraElements.stage,this.webGLContext=r,this.stageCanvas.addEventListener("webglcontextlost",function(i){e.webGLContext&&(i.preventDefault(),e.contextLost=!0,e.renderFrame&&(cancelAnimationFrame(e.renderFrame),e.renderFrame=null),e.gpuTimerExt=void 0,e.activeGpuTimerQuery=null,e.pendingGpuTimerQueries=[],e.emit("webglContextLost"))}),this.stageCanvas.addEventListener("webglcontextrestored",function(){requestAnimationFrame(function(){e.webGLContext&&!e.webGLContext.isContextLost()&&e.restoreWebGLContext()})}),this.initPickingFramebuffer()}},{key:"initPickingFramebuffer",value:function(){var e=this.webGLContext,r=e.createFramebuffer();if(!r)throw new Error("Sigma: cannot create picking frame buffer");e.bindFramebuffer(e.FRAMEBUFFER,r);var i=e.createTexture();if(!i)throw new Error("Sigma: cannot create picking texture");e.bindTexture(e.TEXTURE_2D,i),e.texImage2D(e.TEXTURE_2D,0,e.RGBA,1,1,0,e.RGBA,e.UNSIGNED_BYTE,null),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE),e.framebufferTexture2D(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,i,0);var o=e.createRenderbuffer();if(!o)throw new Error("Sigma: cannot create picking depth buffer");if(e.bindRenderbuffer(e.RENDERBUFFER,o),e.renderbufferStorage(e.RENDERBUFFER,e.DEPTH_COMPONENT16,1,1),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.DEPTH_ATTACHMENT,e.RENDERBUFFER,o),e.checkFramebufferStatus(e.FRAMEBUFFER)!==e.FRAMEBUFFER_COMPLETE)throw new Error("Sigma: picking framebuffer is not complete");e.bindFramebuffer(e.FRAMEBUFFER,null),this.pickingFrameBuffer=r,this.pickingTexture=i,this.pickingDepthBuffer=o}},{key:"initPrograms",value:function(e,r){var i=this,o=this.webGLContext,s=po(o,this.pickingFrameBuffer,i,e?.nodes,r.antialiasNodes),l=s.nodeProgram,u=s.labelProgram,d=s.backdropProgram,h=s.labelBackgroundProgram,f=s.attachmentProgram,c=s.framePass,g=s.shapeSlug,b=s.shapeNameToIndex,m=s.shapeGlobalIds,p=s.variables;this.nodeProgram=l,this.nodeFramePass=c,this.nodeVariableEntries=Object.entries(p),g&&(this.nodeShapeSlug=g);var v=vo(o,this.pickingFrameBuffer,i,e?.edges,r.antialiasEdges,e?.nodes),y=v.edgeProgram,T=v.createLabelPrograms,_=v.framePass,S=v.variables,D=v.paths;return this.createEdgeLabelPrograms=T,this.edgeProgram=y,this.edgeFramePass=_,this.edgeVariableEntries=Object.entries(S),this.edgePathsByName=new Map(D.map(function(w){return[w.name,w]})),{labelProgram:u,edgeLabelProgram:null,edgeLabelBackgroundProgram:null,backdropProgram:d,labelBackgroundProgram:h,attachmentProgram:f,nodeShapeMap:b??null,nodeGlobalShapeIds:m??null}}},{key:"restoreWebGLContext",value:function(){var e,r,i,o,s,l=this.internals;this.initPickingFramebuffer(),(e=l.nodeDataTexture)===null||e===void 0||e.restore(),(r=l.nodeFrameTexture)===null||r===void 0||r.restore(),(i=l.edgeDataTexture)===null||i===void 0||i.restore(),(o=l.edgeFrameTexture)===null||o===void 0||o.restore(),(s=l.attachmentManager)===null||s===void 0||s.restore(),l.hoverResolver.reset(),Object.assign(l,this.initPrograms(l.primitives,l.settings));var u=U(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;h.program.kill(),h.program=h.factory(this.webGLContext)}}catch(f){u.e(f)}finally{u.f()}this.contextLost=!1,this.emit("webglContextRestored"),this.refresh()}},{key:"createLayer",value:function(e,r){var i=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};if(this.extraElements[e])throw new Error('Sigma: a layer named "'.concat(e,'" already exists'));var o=Rt(r,{position:"absolute"},{class:"sigma-".concat(e)});return i.style&&Object.assign(o.style,i.style),this.extraElements[e]=o,"beforeLayer"in i&&i.beforeLayer?this.getLayerElement(i.beforeLayer).before(o):"afterLayer"in i&&i.afterLayer?this.getLayerElement(i.afterLayer).after(o):this.container.appendChild(o),o}},{key:"createCanvas",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.createLayer(e,"canvas",r)}},{key:"createWebGLContext",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=this.createCanvas(e,r);r.hidden&&i.remove();var o=i.getContext("webgl2",H({preserveDrawingBuffer:!1,antialias:!1,depth:!0},r));if(!o)throw new Error("Sigma: WebGL 2 is not supported by your browser. Please use a modern browser (Chrome 56+, Firefox 51+, Safari 15+, Edge 79+).");return o.blendFunc(o.ONE,o.ONE_MINUS_SRC_ALPHA),o}},{key:"killLayer",value:function(e){if(e==="stage"||e==="mouse")throw new Error('Sigma: cannot kill built-in layer "'.concat(e,'"'));var r=this.extraElements[e];if(!r)throw new Error("Sigma: cannot kill layer ".concat(e,", which does not exist"));return r.remove(),delete this.extraElements[e],this}},{key:"getWebGLContext",value:function(){if(!this.webGLContext)throw new Error("Sigma: WebGL context is not available");return this.webGLContext}},{key:"getNodeDataTexture",value:function(){if(!this.internals.nodeDataTexture)throw new Error("Sigma: node data texture is not available");return this.internals.nodeDataTexture}},{key:"getNormalizationFunction",value:function(){return this.normalizationFunction}},{key:"addCustomLayerProgram",value:function(e,r,i){var o;if(!this.depthLayers.includes(r))throw new Error('Sigma: cannot add custom layer program at depth "'.concat(r,'", ')+"it must be declared in primitives.depthLayers. Current layers: ".concat(this.depthLayers.join(", ")));(o=this.customLayerPrograms.get(e))===null||o===void 0||o.program.kill();var s=i(this.webGLContext);return this.customLayerPrograms.set(e,{depth:r,factory:i,program:s}),s.cacheData&&s.cacheData(),this.scheduleRender(),this}},{key:"removeCustomLayerProgram",value:function(e){var r=this.customLayerPrograms.get(e);return r&&(r.program.kill(),this.customLayerPrograms.delete(e),this.scheduleRender()),this}},{key:"getCamera",value:function(){return this.camera}},{key:"setCamera",value:function(e){return this.camera.cancelAnimation(),this.unbindCameraHandlers(),this.camera=e,this.bindCameraHandlers(),this.scheduleRender(),this}},{key:"getContainer",value:function(){return this.container}},{key:"getGraph",value:function(){return this.internals.graph}},{key:"setGraph",value:function(e){return e===this.internals.graph?this:(this.stateManager.pruneNodes(function(r){return e.hasNode(r)}),this.stateManager.pruneEdges(function(r){return e.hasEdge(r)}),this.unbindGraphHandlers(),this.checkEdgesEventsFrame!==null&&(cancelAnimationFrame(this.checkEdgesEventsFrame),this.checkEdgesEventsFrame=null),this.internals.graph=e,this.autoRescaleFrozen=!1,this.bindGraphHandlers(),this.refresh(),this)}},{key:"getMouseCaptor",value:function(){return this.mouseCaptor}},{key:"getTouchCaptor",value:function(){return this.touchCaptor}},{key:"getDimensions",value:function(){return{width:this.width,height:this.height}}},{key:"getGraphDimensions",value:function(){var e=this.customBBox||this.nodeExtent;return{width:e.x[1]-e.x[0]||1,height:e.y[1]-e.y[0]||1}}},{key:"getNodeDisplayData",value:function(e){var r=this.internals.nodeDataCache[e];return r?Object.assign({},r):void 0}},{key:"getEdgeDisplayData",value:function(e){var r=this.internals.edgeDataCache[e];return r?Object.assign({},r):void 0}},{key:"getNodeState",value:function(e){return this.stateManager.getNodeState(e)}},{key:"getEdgeState",value:function(e){return this.stateManager.getEdgeState(e)}},{key:"getGraphState",value:function(){return this.stateManager.getGraphState()}},{key:"setNodeState",value:function(e,r){return this.stateManager.setNodeState(e,r),this}},{key:"setEdgeState",value:function(e,r){return this.stateManager.setEdgeState(e,r),this}},{key:"setGraphState",value:function(e){return this.stateManager.setGraphState(e),this}},{key:"_setPanning",value:function(e){this.stateManager.setGraphState({isPanning:e})}},{key:"_setZooming",value:function(e){this.stateManager.setGraphState({isZooming:e})}},{key:"_hasNodeDrag",value:function(){var e=this.internals.dragManager;return!!(e.pendingNode||e.session)}},{key:"_showGestureHint",value:function(e){var r=this.internals.settings;r.gestureTarget==="shared"&&(this.gestureHint=this.gestureHint||new Ih(this.container),this.gestureHint.show(e,r))}},{key:"setNodesState",value:function(e,r){return this.stateManager.setNodesState(e,r),this}},{key:"setEdgesState",value:function(e,r){return this.stateManager.setEdgesState(e,r),this}},{key:"updateContainerCursor",value:function(){var e=this.stateManager.hovered,r=this.resolvedStageStyle.cursor||"";this.container.style.cursor=e&&yh(this.internals,e)||r}},{key:"refreshStageStyle",value:function(){this.resolvedStageStyle=La(this.stylesDeclaration.stage,this.stateManager.graphState),this.resolvedStageStyle.background!==void 0&&(this.container.style.backgroundColor=this.resolvedStageStyle.background),this.updateContainerCursor()}},{key:"getNodeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedNodeLabels)}},{key:"getEdgeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedEdgeLabels)}},{key:"getSettings",value:function(){return H({},this.internals.settings)}},{key:"getSetting",value:function(e){return this.internals.settings[e]}},{key:"getStyles",value:function(){return H({},this.stylesDeclaration)}},{key:"getPrimitives",value:function(){return H({},this.internals.primitives)}},{key:"setSetting",value:function(e,r){return this.internals.settings[e]=r,Un(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"updateSetting",value:function(e,r){return this.setSetting(e,r(this.internals.settings[e])),this}},{key:"setSettings",value:function(e){return this.internals.settings=H(H({},this.internals.settings),e),Un(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"resize",value:function(e){var r=this.width,i=this.height;if(this.width=this.container.offsetWidth,this.height=this.container.offsetHeight,this.internals.pixelRatio=Yt(),this.width===0)if(this.internals.settings.allowInvalidContainer)this.width=1;else throw new Error("Sigma: Container has no width. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(this.height===0)if(this.internals.settings.allowInvalidContainer)this.height=1;else throw new Error("Sigma: Container has no height. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(!e&&r===this.width&&i===this.height)return this;this.labelRenderer.labelsDirty=!0;for(var o=0,s=[this.mouseLayer].concat(J(Object.values(this.extraElements)));o<s.length;o++){var l=s[o];l.style.width=this.width+"px",l.style.height=this.height+"px"}return this.webGLContext&&(this.stageCanvas.setAttribute("width",this.width*this.internals.pixelRatio+"px"),this.stageCanvas.setAttribute("height",this.height*this.internals.pixelRatio+"px"),this.webGLContext.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio)),this.emit("resize"),this}},{key:"clear",value:function(){return this.emit("beforeClear"),this.webGLContext.bindFramebuffer(WebGLRenderingContext.FRAMEBUFFER,null),this.webGLContext.clear(WebGLRenderingContext.COLOR_BUFFER_BIT),this.emit("afterClear"),this}},{key:"scheduleStateRefresh",value:function(){this.needToRefreshState=!0,this.scheduleRender()}},{key:"refreshState",value:function(){var e=this,r;this.stateManager.flushGraphStateFlags();var i=this.stateManager.graphStateChanged&&this.internals.nodeStyleAnalysis.dependency==="graph-state",o=this.stateManager.graphStateChanged&&this.edgeStyleAnalysis.dependency==="graph-state",s=!1;if(i)this.internals.graph.forEachNode(function(g){e.refreshNodeState(g)&&(s=!0)});else if(this.internals.nodeStyleAnalysis.dependency!=="static"){var l=U(this.stateManager.dirtyNodes),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;this.refreshNodeState(d)&&(s=!0)}}catch(g){l.e(g)}finally{l.f()}}if(o)this.internals.graph.forEachEdge(function(g){e.refreshEdgeState(g)&&(s=!0)});else if(this.edgeStyleAnalysis.dependency!=="static"){var h=U(this.stateManager.dirtyEdges),f;try{for(h.s();!(f=h.n()).done;){var c=f.value;this.refreshEdgeState(c)&&(s=!0)}}catch(g){h.e(g)}finally{h.f()}}this.stateManager.graphStateChanged&&(r=this.stylesDeclaration)!==null&&r!==void 0&&r.stage&&this.refreshStageStyle(),this.stateManager.clearDirtyTracking(),s&&(this.pendingProcess="full")}},{key:"refreshNodeState",value:function(e){var r=this.internals.nodeDataCache[e];if(!r||this.nodeReducer){var i,o,s,l,u=(i=this.internals.nodeDataCache[e])===null||i===void 0?void 0:i.depth,d=(o=this.internals.nodeDataCache[e])===null||o===void 0?void 0:o.zIndex,h=(s=this.internals.nodeDataCache[e])===null||s===void 0?void 0:s.labelAttachment;this.updateNode(e);var f=this.internals.nodeDataCache[e];this.internals.attachmentManager&&f.labelAttachment!==h&&this.internals.attachmentManager.invalidateNode(e);var c;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&f.shape&&f.shape in this.internals.nodeShapeMap?c=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[f.shape]]:c=tn(f.shape||"circle"),(l=this.internals.nodeDataTexture).updateNode.apply(l,[e,f.x,f.y,f.size,c].concat(J(Mn(f)),[f.color])),u&&f.depth!==u&&this.updateNodeDepthRanges(e,u,f.depth);var g=this.nodeProgramIndex[e];return g!==void 0&&(this.addNodeToProgram(e,g),this.nodeProgram.invalidateBuffers()),d!==void 0&&f.zIndex!==d}var b=this.internals.graph.getNodeAttributes(e),m=this.stateManager.getNodeState(e),p=r.size,v=r.shape,y=r.depth,T=r.zIndex,_=r.labelAttachment,S=r.rotationAlignment,D=r.labelRotationAlignment,w=r.color;Nn(this.stylesDeclaration.nodes,b,m,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateNode(r,b,m);var k=this.nodeGraphCoords[e],L=r.x!==k.x||r.y!==k.y;if(L&&(k.x=r.x,k.y=r.y),this.normalizationFunction.applyTo(r),this.internals.nodeShapeMap?(!r.shape||!(r.shape in this.internals.nodeShapeMap))&&(r.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(r.shape=this.nodeShapeSlug),this.internals.attachmentManager&&r.labelAttachment!==_&&this.internals.attachmentManager.invalidateNode(e),rt(this.internals.nodesWithForcedLabels,e,qe(r)),rt(this.internals.nodesWithBackdrop,e,Fa(r)),L||r.size!==p||r.shape!==v||r.rotationAlignment!==S||r.labelRotationAlignment!==D||r.color!==w){var N,R;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&r.shape&&r.shape in this.internals.nodeShapeMap?R=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[r.shape]]:R=tn(r.shape||"circle"),(N=this.internals.nodeDataTexture).updateNode.apply(N,[e,r.x,r.y,r.size,R].concat(J(Mn(r)),[r.color]))}this.itemBuckets.nodes.set(e,r.depth),r.depth!==y&&this.updateNodeDepthRanges(e,y,r.depth);var z=this.nodeProgramIndex[e];return z!==void 0&&(this.addNodeToProgram(e,z),this.nodeProgram.invalidateBuffers()),r.zIndex!==T}},{key:"refreshEdgeState",value:function(e){var r=this.internals.edgeDataCache[e];if(!r||this.edgeReducer){var i=r?.depth,o=r?.zIndex;this.updateEdge(e);var s=this.internals.edgeDataCache[e];i&&s.depth!==i&&this.updateEdgeDepthRanges(e,i,s.depth);var l=this.edgeProgramIndex[e];return l!==void 0&&(this.addEdgeToProgram(e,l),this.edgeProgram.invalidateBuffers()),o!==void 0&&s.zIndex!==o}var u=this.internals.graph.getEdgeAttributes(e),d=this.stateManager.getEdgeState(e),h=r.depth,f=r.zIndex,c=r.size,g=r.path,b=r.selfLoopPath,m=r.parallelPath,p=r.head,v=r.tail;zn(this.stylesDeclaration.edges,u,d,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateEdge(r,u),this.applyEdgeSpread(e,r,d),rt(this.internals.edgesWithForcedLabels,e,qe(r)),this.itemBuckets.edges.set(e,r.depth),r.depth!==h&&this.updateEdgeDepthRanges(e,h,r.depth);var y=this.edgeProgramIndex[e];if(y!==void 0){var T=r.size!==c||r.path!==g||r.selfLoopPath!==b||r.parallelPath!==m||r.head!==p||r.tail!==v;if(T)this.addEdgeToProgram(e,y),this.edgeProgram.invalidateBuffers();else{var _=this.internals.graph.source(e),S=this.internals.graph.target(e),D=this.internals.nodeDataCache[_],w=this.internals.nodeDataCache[S],k=this.edgeTextureIndexCache[e];this.edgeProgram.process(dt(this.pickingState,"edge",e),y,D,w,r,k),this.edgeProgram.invalidateBuffers()}}return r.zIndex!==f}},{key:"refresh",value:function(e){var r=this,i=e?.skipIndexation!==void 0?e?.skipIndexation:!1,o=e?.schedule!==void 0?e.schedule:!1,s=!e||!e.partialGraph;if(s)this.clearEdgeIndices(),this.clearNodeIndices(),this.internals.graph.forEachNode(function(D){return r.addNode(D)}),this.edgeGroups.rebuild(),this.internals.graph.forEachEdge(function(D){return r.addEdge(D)}),this.pendingProcess="full";else{for(var l,u,d=((l=e.partialGraph)===null||l===void 0?void 0:l.nodes)||[],h=0,f=d?.length||0;h<f;h++){var c,g,b=""+d[h],m=(c=this.internals.nodeDataCache[b])===null||c===void 0?void 0:c.labelAttachment;if(this.updateNode(b),this.internals.attachmentManager&&((g=this.internals.nodeDataCache[b])!==null&&g!==void 0&&g.labelAttachment||m)&&this.internals.attachmentManager.invalidateNode(b),i){var p=this.nodeProgramIndex[b];if(p===void 0)throw new Error('Sigma: node "'.concat(b,`" can't be repaint`));this.addNodeToProgram(b,p)}}i&&d.length>0&&(this.nodeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0);for(var v=(e==null||(u=e.partialGraph)===null||u===void 0?void 0:u.edges)||[],y=0,T=v.length;y<T;y++){var _=""+v[y];if(this.updateEdge(_),i){var S=this.edgeProgramIndex[_];if(S===void 0)throw new Error('Sigma: edge "'.concat(_,`" can't be repaint`));this.addEdgeToProgram(_,S)}}i&&v.length>0&&(this.edgeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0),!i&&this.pendingProcess!=="full"&&(this.pendingProcess=v.length>0?"full":"nodes")}return o?this.scheduleRender():this.render(),this}},{key:"scheduleRender",value:function(){var e=this;return this.renderFrame||(this.renderFrame=requestAnimationFrame(function(){e.render()})),this}},{key:"scheduleRefresh",value:function(e){return this.refresh(H(H({},e),{},{schedule:!0}))}},{key:"getViewportZoomedState",value:function(e,r){var i=this.camera.getState(),o=i.ratio,s=i.angle,l=i.x,u=i.y,d=this.internals.settings,h=d.minCameraRatio,f=d.maxCameraRatio;typeof f=="number"&&(r=Math.min(r,f)),typeof h=="number"&&(r=Math.max(r,h));var c=r/o,g={x:this.width/2,y:this.height/2},b=this.viewportToFramedGraph(e),m=this.viewportToFramedGraph(g);return{angle:s,x:(b.x-m.x)*(1-c)+l,y:(b.y-m.y)*(1-c)+u,ratio:r}}},{key:"viewRectangle",value:function(){var e=this.viewportToFramedGraph({x:0,y:0}),r=this.viewportToFramedGraph({x:this.width,y:0}),i=this.viewportToFramedGraph({x:0,y:this.height});return{x1:e.x,y1:e.y,x2:r.x,y2:r.y,height:r.y-i.y}}},{key:"framedGraphToViewport",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?je(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding()):this.matrix),s=Tt(o,e);return{x:(1+s.x)*this.width/2,y:(1-s.y)*this.height/2}}},{key:"viewportToFramedGraph",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?je(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding(),!0):this.invMatrix),s=Tt(o,{x:e.x/this.width*2-1,y:1-e.y/this.height*2});return isNaN(s.x)&&(s.x=0),isNaN(s.y)&&(s.y=0),s}},{key:"viewportToGraph",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.normalizationFunction.inverse(this.viewportToFramedGraph(e,r))}},{key:"graphToViewport",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.framedGraphToViewport(this.normalizationFunction(e),r)}},{key:"graphToFramedGraph",value:function(e){return this.normalizationFunction(e)}},{key:"framedGraphToGraph",value:function(e){return this.normalizationFunction.inverse(e)}},{key:"getGraphToViewportRatio",value:function(){var e={x:0,y:0},r={x:1,y:1},i=Math.sqrt(Math.pow(e.x-r.x,2)+Math.pow(e.y-r.y,2)),o=this.graphToViewport(e),s=this.graphToViewport(r),l=Math.sqrt(Math.pow(o.x-s.x,2)+Math.pow(o.y-s.y,2));return l/i}},{key:"computeNodeExtent",value:function(){var e=this.nodeGraphCoords,r=Object.keys(e);if(!r.length)return{x:[0,1],y:[0,1]};for(var i=1/0,o=-1/0,s=1/0,l=-1/0,u=0,d=r.length;u<d;u++){var h=e[r[u]],f=h.x,c=h.y;f<i&&(i=f),f>o&&(o=f),c<s&&(s=c),c>l&&(l=c)}return{x:[i,o],y:[s,l]}}},{key:"getBBox",value:function(){return this.nodeExtent}},{key:"getCustomBBox",value:function(){return this.customBBox}},{key:"setCustomBBox",value:function(e){return this.customBBox=e,this.scheduleRender(),this}},{key:"kill",value:function(){var e,r,i,o,s;this.emit("kill"),this.removeAllListeners(),this.camera.cancelAnimation(),this.unbindCameraHandlers(),window.removeEventListener("resize",this.activeListeners.handleResize),this.mouseCaptor.kill(),this.touchCaptor.kill(),this.internals.hoverResolver.kill(),this.unbindGraphHandlers(),this.clearIndices(),this.clearState(),this.internals.nodeDataCache={},this.internals.edgeDataCache={},this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null);for(var l=this.container;l.firstChild;)l.removeChild(l.firstChild);this.nodeProgram.kill(),this.nodeFramePass.kill(),this.edgeProgram.kill(),this.edgeFramePass.kill(),this.internals.labelProgram.kill(),(e=this.internals.edgeLabelProgram)===null||e===void 0||e.kill(),(r=this.internals.edgeLabelBackgroundProgram)===null||r===void 0||r.kill(),this.internals.backdropProgram.kill(),this.internals.labelBackgroundProgram.kill(),(i=this.internals.attachmentProgram)===null||i===void 0||i.kill(),(o=this.internals.attachmentManager)===null||o===void 0||o.kill(),this.internals.attachmentProgram=null,this.internals.attachmentManager=null;var u=U(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var h=d.value.program;h.kill()}}catch(g){u.e(g)}finally{u.f()}if(this.customLayerPrograms.clear(),this.sdfAtlas&&(this.sdfAtlas=null),this.internals.nodeDataTexture&&(this.internals.nodeDataTexture.kill(),this.internals.nodeDataTexture=null),this.internals.nodeFrameTexture&&(this.internals.nodeFrameTexture.kill(),this.internals.nodeFrameTexture=null),this.internals.edgeDataTexture&&(this.internals.edgeDataTexture.kill(),this.internals.edgeDataTexture=null),this.internals.edgeFrameTexture&&(this.internals.edgeFrameTexture.kill(),this.internals.edgeFrameTexture=null),this.webGLContext){var f;(f=this.webGLContext.getExtension("WEBGL_lose_context"))===null||f===void 0||f.loseContext(),this.webGLContext=null}(s=this.gestureHint)===null||s===void 0||s.kill(),this.gestureHint=null,this.mouseLayer.remove();for(var c in this.extraElements)this.extraElements[c].remove();this.extraElements={}}},{key:"scaleSize",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:1,r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:this.camera.ratio;return e/this.internals.settings.zoomToSizeRatioFunction(r)*(this.getSetting("itemSizesReference")==="positions"?r*this.graphToViewportRatio:1)}},{key:"getStageCanvas",value:function(){return this.stageCanvas}},{key:"getMouseLayer",value:function(){return this.mouseLayer}}])})(Kt),ar=Oh;var gf=$e(vt());var rr=7,Vo=4,Hh=13,$h=1.6,Vh="Segoe UI Variable Text, Segoe UI, system-ui, sans-serif",jh=`#version 300 es
void main(){gl_Position=vec4(float((gl_VertexID<<1)&2)*2.0-1.0,float(gl_VertexID&2)*2.0-1.0,0.0,1.0);}`,Xh=`#version 300 es
precision mediump float;uniform vec4 u_tint;out vec4 o_color;void main(){o_color=u_tint;}`;function ir(a){let n=a.trim().toLowerCase();if(n.startsWith("#")){let s=n.slice(1),l=s.length<=4?[...s].map(d=>d+d).join(""):s,u=d=>parseInt(l.slice(d,d+2),16)/255;return[u(0),u(2),u(4),l.length>=8?u(6):1]}let t=(n.match(/[\d.]+%?/g)??[]).map(s=>s.endsWith("%")?parseFloat(s)/100:parseFloat(s)),[e=0,r=0,i=0,o=1]=t;return[e/255,r/255,i/255,o]}function jo(a){return Math.min(Vo,Math.max(0,Math.ceil(Math.log2(1/Math.max(a[3],2**-Vo)))))}function Xo(a,n,t){return e=>{let r=e.createProgram();for(let[s,l]of[[e.VERTEX_SHADER,jh],[e.FRAGMENT_SHADER,Xh]]){let u=e.createShader(s);e.shaderSource(u,l),e.compileShader(u),e.attachShader(r,u),e.deleteShader(u)}e.linkProgram(r);let i=e.getUniformLocation(r,"u_tint"),o=e.createVertexArray();return{render(s){if(t()){e.bindFramebuffer(e.FRAMEBUFFER,null),e.viewport(0,0,s.width*s.pixelRatio,s.height*s.pixelRatio),e.enable(e.BLEND),e.useProgram(r),e.bindVertexArray(o),e.blendFuncSeparate(e.ZERO,e.ONE,e.DST_ALPHA,e.ONE),e.uniform4f(i,0,0,0,1);for(let l=0;l<n;l++)e.drawArrays(e.TRIANGLES,0,3);e.blendFuncSeparate(e.DST_ALPHA,e.ZERO,e.DST_ALPHA,e.ZERO),e.uniform4f(i,a[0]*a[3],a[1]*a[3],a[2]*a[3],a[3]),e.drawArrays(e.TRIANGLES,0,3),e.bindVertexArray(null),e.blendFunc(e.ONE,e.ONE_MINUS_SRC_ALPHA)}},kill(){e.deleteProgram(r),e.deleteVertexArray(o)}}}}function qo(a){return{labelColor:a.label,labelFont:Vh,labelSize:Hh}}function Ko(){return{labelRenderedSizeThreshold:rr,labelDensity:.35,labelGridCellSize:110,minEdgeThickness:.8,zoomToSizeRatioFunction:()=>1}}function qh(a){let n=[Dt.nodes,{...qo(a),label:{attribute:"tag"},cursor:"pointer",depth:"nodes"},{whenState:"named",then:{labelVisibility:"visible"}},{whenState:"mute",then:{labelVisibility:"hidden"}},{whenState:"off",then:{visibility:"hidden"}}],t=[Dt.edges,{color:a.edge,depth:"edges"},{whenState:"off",then:{visibility:"hidden"}}];return{nodes:n,edges:t}}function Kh(a){let n=[Dt.nodes,{...qo(a),labelVisibility:"visible",backdropVisibility:"visible",backdropColor:a.back,backdropBorderColor:a.backLine,backdropBorderWidth:1,backdropCornerRadius:6,backdropShadowColor:a.shadow,depth:"nodes"},{whenData:"pin",then:{depth:"pin"}},{whenState:"hush",then:{labelVisibility:"hidden",backdropVisibility:"hidden"}}],t=[Dt.edges,{color:a.edgeOn,size:$h,depth:"edges"}];return{nodes:n,edges:t}}function Yh(a){return{primitives:{depthLayers:["edges","nodes","veil"]},customNodeState:{...Kr},customEdgeState:{...Yr},styles:qh(a),settings:Ko()}}function Zh(a){return{primitives:{depthLayers:["edges","nodes","pin"]},customNodeState:{...Zr},styles:Kh(a),settings:Ko()}}function Yo(a,n,t){return new ar(a,n,Yh(t))}function or(a,n,t){return new ar(a,n,Zh(t))}var Qe=a=>getComputedStyle(document.documentElement).getPropertyValue(a).trim(),Qh=10,Jh=14,ec=200,tc=2e3,nc=120,ac=160;function rc(a){let n=!1;return()=>{n||(n=!0,requestAnimationFrame(()=>{n=!1,a()}))}}async function Rf(a,n){let t={label:Qe("--g-label"),dim:Qe("--g-dim"),edge:Qe("--g-edge"),edgeOn:Qe("--g-edge-on"),back:Qe("--g-back"),backLine:Qe("--g-back-line"),shadow:Qe("--g-shadow")};Wt(a,re("div",{class:"empty",style:"margin:24px"},ce.graph.loading));let e=await sr("graph",...n?[n]:[]),{g:r,colors:i,names:o}=Br(e,Qe("--g-other")),s=n??"all",l=Mr(r,Nr(s)),u=re("div",{class:"canvas"}),d=re("div",{class:"canvas",style:"pointer-events:none"}),h=re("span",{class:"info"},ce.graph.nodes(r.order,r.size)),f=re("aside",{}),c=new Set,g=!1,b=r.nodes(),m=new Set([...b].sort((O,X)=>r.getNodeAttribute(X,"deg")-r.getNodeAttribute(O,"deg")).slice(0,Jh)),p=new Set(m);r.forEachNode((O,X)=>{X.size>=rr&&p.add(O)});let v=[...p];for(let O of v)r.setNodeAttribute(O,"tag",r.getNodeAttribute(O,"label"));let y=null,T=null,_=r.mapNodes((O,X)=>X.t).filter(O=>O>0),[S,D]=_.length?[Math.min(..._),Math.max(..._)]:[0,0],w=D,k=O=>!c.has(O.kind)&&(g||O.deg>0)&&(O.t===0||O.t<=w),L=!1,N=0,R=null,z=!1,C=0,I=O=>{clearTimeout(C),O&&(C=window.setTimeout(()=>I(!1),ac)),!(O===z||!R||O&&De.order<=nc)&&(z=O,R.setNodesState(De.nodes(),{hush:O}))},B=()=>{let O=Te(y??T,y===null?null:T)>0;O!==L&&ue.setNodesState(v,{mute:O}),L=O,z=!1,O&&(R??=or(De,d,t),R.setCustomBBox(ue.getBBox()),R.getCamera().setState(ue.getCamera().getState())),ue.scheduleRender()},F=rc(()=>{let O=ve.sift(k);O.length&&(L&&B(),O.some(X=>p.has(X))&&(clearTimeout(N),N=window.setTimeout(()=>ue.refresh({partialGraph:{nodes:[]},schedule:!0}),ec)))}),x=O=>re("span",{class:"swatch",style:`background:${r.getNodeAttribute(O,"color")}`}),P=O=>re("a",{href:"#",onclick:X=>{X.preventDefault(),Y(O)}},x(O),r.getNodeAttribute(O,"label")),W=O=>{let X=r.getNodeAttributes(O);X.kind==="note"?oa(`#/n/${O.slice(1)}`):X.kind==="project"&&oa(`#/p/${ur(X.group)}`)},G=re("div",{class:"legend"},Ir.map(O=>re("label",{},re("input",{type:"checkbox",checked:!0,onchange:X=>{X.target.checked?c.delete(O):c.add(O),F()}}),ce.graph.kinds[O])),re("label",{},re("input",{type:"checkbox",checked:g,onchange:O=>{g=O.target.checked,F()}}),ce.graph.orphans)),A=()=>{let O=r.filterNodes((ee,he)=>he.kind==="note").sort((ee,he)=>r.getNodeAttribute(he,"deg")-r.getNodeAttribute(ee,"deg")).slice(0,Qh),X=[...i.keys()].map(ee=>{let he=r.findNode((K,M)=>M.cluster===ee&&M.label===o.get(ee))??r.findNode((K,M)=>M.cluster===ee),Q=r.filterNodes((K,M)=>M.cluster===ee).length;return re("a",{href:"#",onclick:K=>{K.preventDefault(),he&&Y(he)}},re("span",{class:"swatch",style:`background:${i.get(ee)}`}),o.get(ee)??ee,re("span",{class:"faint"},` \xB7 ${Q}`))});Wt(f,re("h2",{style:"margin-top:0"},ce.graph.show),G,X.length?re("h2",{},ce.graph.groups):null,...X,re("h2",{},ce.graph.hubs),...O.map(P))},q=O=>{let X=r.getNodeAttributes(O),ee=r.neighbors(O).sort((he,Q)=>r.getNodeAttribute(Q,"deg")-r.getNodeAttribute(he,"deg"));Wt(f,re("div",{class:"detail"},re("button",{class:"back",onclick:()=>Y(null)},`\u2190 ${ce.graph.back}`),re("h3",{},X.label),re("div",{class:"meta"},[ce.graph.kinds[X.kind],X.t?ce.ago(lr(X.t)):"",i.has(X.cluster)?o.get(X.cluster):""].filter(Boolean).join(" \xB7 ")),re("div",{class:"actions"},X.kind==="session"?null:re("button",{class:"primary",onclick:()=>W(O)},ce.graph.open),re("button",{onclick:()=>V(O)},ce.graph.center)),re("h2",{},ce.graph.connections,re("span",{class:"n"},ee.length)),ee.length?ee.map(P):re("div",{class:"faint"},ce.graph.alone)))},V=O=>{let X=ue.getNodeDisplayData(O);X&&ue.getCamera().animate({x:X.x,y:X.y,ratio:.3},{duration:350})},Y=O=>{y=O,B(),O?(q(O),V(O)):A()},pe=re("input",{type:"search",class:"find",placeholder:ce.graph.find,onkeydown:O=>{if(O.key!=="Enter")return;let X=O.target.value.trim().toLowerCase();if(!X)return Y(null);let ee=r.filterNodes((he,Q)=>Q.label.toLowerCase().includes(X));ee.sort((he,Q)=>+(r.getNodeAttribute(Q,"kind")==="project")-+(r.getNodeAttribute(he,"kind")==="project")||r.getNodeAttribute(Q,"deg")-r.getNodeAttribute(he,"deg")),ee.length&&Y(ee[0])}}),se=re("span",{},ce.graph.all),Z=re("input",{type:"range",min:0,max:1e3,value:1e3,"aria-label":ce.graph.until,oninput:O=>{let X=Number(O.target.value)/1e3;w=S+(D-S)*X,se.textContent=X>=1?ce.graph.all:`${ce.graph.until} ${new Date(w*1e3).toLocaleDateString("en-US",{year:"numeric",month:"short",day:"numeric"})}`,F()}});Wt(a,re("div",{class:"graph"},re("div",{class:"stage"},u,d,re("div",{class:"tools"},pe,h),re("div",{class:"hint-box"},ce.graph.focus),D>S?re("div",{class:"time"},Z,se):null),f)),A();let ue=Yo(r,u,t),ve=Jr(r,ue),{top:De,spot:Te}=Qr(r,ve.gone);ue.addCustomLayerProgram("veil","veil",Xo(ir(t.dim),jo(ir(t.edge)),()=>L)),ue.setNodesState([...m],{named:!0}),ve.sift(k),ue.getCamera().on("updated",O=>{L&&(R?.getCamera().setState(O),I(!0))});let Mt=qr(()=>R??=or(De,d,t),tc);ue.on("enterNode",({node:O})=>{T=O,B()}),ue.on("leaveNode",()=>{T=null,B()}),ue.on("clickNode",({node:O})=>Y(O===y?null:O)),ue.on("clickStage",()=>Y(null)),ue.on("doubleClickNode",({node:O})=>W(O));let Me=()=>{};return l&&(h.textContent=ce.graph.layout,Me=Xr(r,jr(r.order,l),()=>{ue.refresh({partialGraph:{nodes:b}}),L&&B()},()=>{zr(s,r),h.textContent=ce.graph.nodes(r.order,r.size)})),()=>{clearTimeout(N),clearTimeout(C),Mt(),Me(),R?.kill(),ue.kill()}}export{Rf as graph};
