import{a as et,b as lt,d as Rr,g as re,j as on,k as Cr,l as Ra,m as Lr,n as me}from"./chunk-4W7R3RY3.js";var sn=et((Tc,Ca)=>{"use strict";var zt=typeof Reflect=="object"?Reflect:null,kr=zt&&typeof zt.apply=="function"?zt.apply:function(n,e,t){return Function.prototype.apply.call(n,e,t)},Nn;zt&&typeof zt.ownKeys=="function"?Nn=zt.ownKeys:Object.getOwnPropertySymbols?Nn=function(n){return Object.getOwnPropertyNames(n).concat(Object.getOwnPropertySymbols(n))}:Nn=function(n){return Object.getOwnPropertyNames(n)};function fs(a){console&&console.warn&&console.warn(a)}var Fr=Number.isNaN||function(n){return n!==n};function ge(){ge.init.call(this)}Ca.exports=ge;Ca.exports.once=ms;ge.EventEmitter=ge;ge.prototype._events=void 0;ge.prototype._eventsCount=0;ge.prototype._maxListeners=void 0;var Pr=10;function Mn(a){if(typeof a!="function")throw new TypeError('The "listener" argument must be of type Function. Received type '+typeof a)}Object.defineProperty(ge,"defaultMaxListeners",{enumerable:!0,get:function(){return Pr},set:function(a){if(typeof a!="number"||a<0||Fr(a))throw new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received '+a+".");Pr=a}});ge.init=function(){(this._events===void 0||this._events===Object.getPrototypeOf(this)._events)&&(this._events=Object.create(null),this._eventsCount=0),this._maxListeners=this._maxListeners||void 0};ge.prototype.setMaxListeners=function(n){if(typeof n!="number"||n<0||Fr(n))throw new RangeError('The value of "n" is out of range. It must be a non-negative number. Received '+n+".");return this._maxListeners=n,this};function Ir(a){return a._maxListeners===void 0?ge.defaultMaxListeners:a._maxListeners}ge.prototype.getMaxListeners=function(){return Ir(this)};ge.prototype.emit=function(n){for(var e=[],t=1;t<arguments.length;t++)e.push(arguments[t]);var r=n==="error",i=this._events;if(i!==void 0)r=r&&i.error===void 0;else if(!r)return!1;if(r){var o;if(e.length>0&&(o=e[0]),o instanceof Error)throw o;var s=new Error("Unhandled error."+(o?" ("+o.message+")":""));throw s.context=o,s}var l=i[n];if(l===void 0)return!1;if(typeof l=="function")kr(l,this,e);else for(var u=l.length,d=Wr(l,u),t=0;t<u;++t)kr(d[t],this,e);return!0};function Gr(a,n,e,t){var r,i,o;if(Mn(e),i=a._events,i===void 0?(i=a._events=Object.create(null),a._eventsCount=0):(i.newListener!==void 0&&(a.emit("newListener",n,e.listener?e.listener:e),i=a._events),o=i[n]),o===void 0)o=i[n]=e,++a._eventsCount;else if(typeof o=="function"?o=i[n]=t?[e,o]:[o,e]:t?o.unshift(e):o.push(e),r=Ir(a),r>0&&o.length>r&&!o.warned){o.warned=!0;var s=new Error("Possible EventEmitter memory leak detected. "+o.length+" "+String(n)+" listeners added. Use emitter.setMaxListeners() to increase limit");s.name="MaxListenersExceededWarning",s.emitter=a,s.type=n,s.count=o.length,fs(s)}return a}ge.prototype.addListener=function(n,e){return Gr(this,n,e,!1)};ge.prototype.on=ge.prototype.addListener;ge.prototype.prependListener=function(n,e){return Gr(this,n,e,!0)};function gs(){if(!this.fired)return this.target.removeListener(this.type,this.wrapFn),this.fired=!0,arguments.length===0?this.listener.call(this.target):this.listener.apply(this.target,arguments)}function zr(a,n,e){var t={fired:!1,wrapFn:void 0,target:a,type:n,listener:e},r=gs.bind(t);return r.listener=e,t.wrapFn=r,r}ge.prototype.once=function(n,e){return Mn(e),this.on(n,zr(this,n,e)),this};ge.prototype.prependOnceListener=function(n,e){return Mn(e),this.prependListener(n,zr(this,n,e)),this};ge.prototype.removeListener=function(n,e){var t,r,i,o,s;if(Mn(e),r=this._events,r===void 0)return this;if(t=r[n],t===void 0)return this;if(t===e||t.listener===e)--this._eventsCount===0?this._events=Object.create(null):(delete r[n],r.removeListener&&this.emit("removeListener",n,t.listener||e));else if(typeof t!="function"){for(i=-1,o=t.length-1;o>=0;o--)if(t[o]===e||t[o].listener===e){s=t[o].listener,i=o;break}if(i<0)return this;i===0?t.shift():vs(t,i),t.length===1&&(r[n]=t[0]),r.removeListener!==void 0&&this.emit("removeListener",n,s||e)}return this};ge.prototype.off=ge.prototype.removeListener;ge.prototype.removeAllListeners=function(n){var e,t,r;if(t=this._events,t===void 0)return this;if(t.removeListener===void 0)return arguments.length===0?(this._events=Object.create(null),this._eventsCount=0):t[n]!==void 0&&(--this._eventsCount===0?this._events=Object.create(null):delete t[n]),this;if(arguments.length===0){var i=Object.keys(t),o;for(r=0;r<i.length;++r)o=i[r],o!=="removeListener"&&this.removeAllListeners(o);return this.removeAllListeners("removeListener"),this._events=Object.create(null),this._eventsCount=0,this}if(e=t[n],typeof e=="function")this.removeListener(n,e);else if(e!==void 0)for(r=e.length-1;r>=0;r--)this.removeListener(n,e[r]);return this};function Nr(a,n,e){var t=a._events;if(t===void 0)return[];var r=t[n];return r===void 0?[]:typeof r=="function"?e?[r.listener||r]:[r]:e?ps(r):Wr(r,r.length)}ge.prototype.listeners=function(n){return Nr(this,n,!0)};ge.prototype.rawListeners=function(n){return Nr(this,n,!1)};ge.listenerCount=function(a,n){return typeof a.listenerCount=="function"?a.listenerCount(n):Mr.call(a,n)};ge.prototype.listenerCount=Mr;function Mr(a){var n=this._events;if(n!==void 0){var e=n[a];if(typeof e=="function")return 1;if(e!==void 0)return e.length}return 0}ge.prototype.eventNames=function(){return this._eventsCount>0?Nn(this._events):[]};function Wr(a,n){for(var e=new Array(n),t=0;t<n;++t)e[t]=a[t];return e}function vs(a,n){for(;n+1<a.length;n++)a[n]=a[n+1];a.pop()}function ps(a){for(var n=new Array(a.length),e=0;e<n.length;++e)n[e]=a[e].listener||a[e];return n}function ms(a,n){return new Promise(function(e,t){function r(o){a.removeListener(n,i),t(o)}function i(){typeof a.removeListener=="function"&&a.removeListener("error",r),e([].slice.call(arguments))}Br(a,n,i,{once:!0}),n!=="error"&&bs(a,r,{once:!0})})}function bs(a,n,e){typeof a.on=="function"&&Br(a,"error",n,e)}function Br(a,n,e,t){if(typeof a.on=="function")t.once?a.once(n,e):a.on(n,e);else if(typeof a.addEventListener=="function")a.addEventListener(n,function r(i){t.once&&a.removeEventListener(n,r),e(i)});else throw new TypeError('The "emitter" argument must be of type EventEmitter. Received type '+typeof a)}});var hn=et((Sc,ei)=>{ei.exports=function(n){return n!==null&&typeof n=="object"&&typeof n.addUndirectedEdgeWithKey=="function"&&typeof n.dropNode=="function"&&typeof n.multi=="boolean"}});var Ga=et(jn=>{function Sl(a){return typeof a!="number"||isNaN(a)?1:a}function Al(a,n){var e={},t=function(o){return typeof o>"u"?n:o};typeof n=="function"&&(t=n);var r=function(o){return t(o[a])},i=function(){return t(void 0)};return typeof a=="string"?(e.fromAttributes=r,e.fromGraph=function(o,s){return r(o.getNodeAttributes(s))},e.fromEntry=function(o,s){return r(s)}):typeof a=="function"?(e.fromAttributes=function(){throw new Error("graphology-utils/getters/createNodeValueGetter: irrelevant usage.")},e.fromGraph=function(o,s){return t(a(s,o.getNodeAttributes(s)))},e.fromEntry=function(o,s){return t(a(o,s))}):(e.fromAttributes=i,e.fromGraph=i,e.fromEntry=i),e}function ti(a,n){var e={},t=function(o){return typeof o>"u"?n:o};typeof n=="function"&&(t=n);var r=function(o){return t(o[a])},i=function(){return t(void 0)};return typeof a=="string"?(e.fromAttributes=r,e.fromGraph=function(o,s){return r(o.getEdgeAttributes(s))},e.fromEntry=function(o,s){return r(s)},e.fromPartialEntry=e.fromEntry,e.fromMinimalEntry=e.fromEntry):typeof a=="function"?(e.fromAttributes=function(){throw new Error("graphology-utils/getters/createEdgeValueGetter: irrelevant usage.")},e.fromGraph=function(o,s){var l=o.extremities(s);return t(a(s,o.getEdgeAttributes(s),l[0],l[1],o.getNodeAttributes(l[0]),o.getNodeAttributes(l[1]),o.isUndirected(s)))},e.fromEntry=function(o,s,l,u,d,h,c){return t(a(o,s,l,u,d,h,c))},e.fromPartialEntry=function(o,s,l,u){return t(a(o,s,l,u))},e.fromMinimalEntry=function(o,s){return t(a(o,s))}):(e.fromAttributes=i,e.fromGraph=i,e.fromEntry=i,e.fromMinimalEntry=i),e}jn.createNodeValueGetter=Al;jn.createEdgeValueGetter=ti;jn.createEdgeWeightGetter=function(a){return ti(a,Sl)}});var oi=et((Dc,ii)=>{var Ee=0,be=1,he=2,ce=3,at=4,rt=5,fe=6,ni=7,Xn=8,ai=9,Dl=0,wl=1,Rl=2,we=0,Oe=1,Ie=2,Tt=3,ut=4,xe=5,Ge=6,Ye=7,Ze=8,ri=3,qe=10,Cl=3,ke=9,za=10;ii.exports=function(n,e,t){var r,i,o,s,l,u,d,h,c,f,g=e.length,b=t.length,m=n.adjustSizes,v=n.barnesHutTheta*n.barnesHutTheta,p,y,T,_,S,D,A,E=[];for(o=0;o<g;o+=qe)e[o+at]=e[o+he],e[o+rt]=e[o+ce],e[o+he]=0,e[o+ce]=0;if(n.outboundAttractionDistribution){for(p=0,o=0;o<g;o+=qe)p+=e[o+fe];p/=g/qe}if(n.barnesHutOptimize){var L=1/0,z=-1/0,C=1/0,N=-1/0,k,F,M;for(o=0;o<g;o+=qe)L=Math.min(L,e[o+Ee]),z=Math.max(z,e[o+Ee]),C=Math.min(C,e[o+be]),N=Math.max(N,e[o+be]);var I=z-L,x=N-C;for(I>x?(C-=(I-x)/2,N=C+I):(L-=(x-I)/2,z=L+x),E[0+we]=-1,E[0+Oe]=(L+z)/2,E[0+Ie]=(C+N)/2,E[0+Tt]=Math.max(z-L,N-C),E[0+ut]=-1,E[0+xe]=-1,E[0+Ge]=0,E[0+Ye]=0,E[0+Ze]=0,r=1,o=0;o<g;o+=qe)for(i=0,M=ri;;)if(E[i+xe]>=0){e[o+Ee]<E[i+Oe]?e[o+be]<E[i+Ie]?k=E[i+xe]:k=E[i+xe]+ke:e[o+be]<E[i+Ie]?k=E[i+xe]+ke*2:k=E[i+xe]+ke*3,E[i+Ye]=(E[i+Ye]*E[i+Ge]+e[o+Ee]*e[o+fe])/(E[i+Ge]+e[o+fe]),E[i+Ze]=(E[i+Ze]*E[i+Ge]+e[o+be]*e[o+fe])/(E[i+Ge]+e[o+fe]),E[i+Ge]+=e[o+fe],i=k;continue}else if(E[i+we]<0){E[i+we]=o;break}else{if(E[i+xe]=r*ke,h=E[i+Tt]/2,c=E[i+xe],E[c+we]=-1,E[c+Oe]=E[i+Oe]-h,E[c+Ie]=E[i+Ie]-h,E[c+Tt]=h,E[c+ut]=c+ke,E[c+xe]=-1,E[c+Ge]=0,E[c+Ye]=0,E[c+Ze]=0,c+=ke,E[c+we]=-1,E[c+Oe]=E[i+Oe]-h,E[c+Ie]=E[i+Ie]+h,E[c+Tt]=h,E[c+ut]=c+ke,E[c+xe]=-1,E[c+Ge]=0,E[c+Ye]=0,E[c+Ze]=0,c+=ke,E[c+we]=-1,E[c+Oe]=E[i+Oe]+h,E[c+Ie]=E[i+Ie]-h,E[c+Tt]=h,E[c+ut]=c+ke,E[c+xe]=-1,E[c+Ge]=0,E[c+Ye]=0,E[c+Ze]=0,c+=ke,E[c+we]=-1,E[c+Oe]=E[i+Oe]+h,E[c+Ie]=E[i+Ie]+h,E[c+Tt]=h,E[c+ut]=E[i+ut],E[c+xe]=-1,E[c+Ge]=0,E[c+Ye]=0,E[c+Ze]=0,r+=4,e[E[i+we]+Ee]<E[i+Oe]?e[E[i+we]+be]<E[i+Ie]?k=E[i+xe]:k=E[i+xe]+ke:e[E[i+we]+be]<E[i+Ie]?k=E[i+xe]+ke*2:k=E[i+xe]+ke*3,E[i+Ge]=e[E[i+we]+fe],E[i+Ye]=e[E[i+we]+Ee],E[i+Ze]=e[E[i+we]+be],E[k+we]=E[i+we],E[i+we]=-1,e[o+Ee]<E[i+Oe]?e[o+be]<E[i+Ie]?F=E[i+xe]:F=E[i+xe]+ke:e[o+be]<E[i+Ie]?F=E[i+xe]+ke*2:F=E[i+xe]+ke*3,k===F)if(M--){i=k;continue}else{M=ri;break}E[F+we]=o;break}}if(n.barnesHutOptimize)for(y=n.scalingRatio,o=0;o<g;o+=qe)for(i=0;;)if(E[i+xe]>=0)if(D=Math.pow(e[o+Ee]-E[i+Ye],2)+Math.pow(e[o+be]-E[i+Ze],2),f=E[i+Tt],4*f*f/D<v){if(T=e[o+Ee]-E[i+Ye],_=e[o+be]-E[i+Ze],m===!0?D>0?(A=y*e[o+fe]*E[i+Ge]/D,e[o+he]+=T*A,e[o+ce]+=_*A):D<0&&(A=-y*e[o+fe]*E[i+Ge]/Math.sqrt(D),e[o+he]+=T*A,e[o+ce]+=_*A):D>0&&(A=y*e[o+fe]*E[i+Ge]/D,e[o+he]+=T*A,e[o+ce]+=_*A),i=E[i+ut],i<0)break;continue}else{i=E[i+xe];continue}else{if(u=E[i+we],u>=0&&u!==o&&(T=e[o+Ee]-e[u+Ee],_=e[o+be]-e[u+be],D=T*T+_*_,m===!0?D>0?(A=y*e[o+fe]*e[u+fe]/D,e[o+he]+=T*A,e[o+ce]+=_*A):D<0&&(A=-y*e[o+fe]*e[u+fe]/Math.sqrt(D),e[o+he]+=T*A,e[o+ce]+=_*A):D>0&&(A=y*e[o+fe]*e[u+fe]/D,e[o+he]+=T*A,e[o+ce]+=_*A)),i=E[i+ut],i<0)break;continue}else for(y=n.scalingRatio,s=0;s<g;s+=qe)for(l=0;l<s;l+=qe)T=e[s+Ee]-e[l+Ee],_=e[s+be]-e[l+be],m===!0?(D=Math.sqrt(T*T+_*_)-e[s+Xn]-e[l+Xn],D>0?(A=y*e[s+fe]*e[l+fe]/D/D,e[s+he]+=T*A,e[s+ce]+=_*A,e[l+he]-=T*A,e[l+ce]-=_*A):D<0&&(A=100*y*e[s+fe]*e[l+fe],e[s+he]+=T*A,e[s+ce]+=_*A,e[l+he]-=T*A,e[l+ce]-=_*A)):(D=Math.sqrt(T*T+_*_),D>0&&(A=y*e[s+fe]*e[l+fe]/D/D,e[s+he]+=T*A,e[s+ce]+=_*A,e[l+he]-=T*A,e[l+ce]-=_*A));for(c=n.gravity/n.scalingRatio,y=n.scalingRatio,o=0;o<g;o+=qe)A=0,T=e[o+Ee],_=e[o+be],D=Math.sqrt(Math.pow(T,2)+Math.pow(_,2)),n.strongGravityMode?D>0&&(A=y*e[o+fe]*c):D>0&&(A=y*e[o+fe]*c/D),e[o+he]-=T*A,e[o+ce]-=_*A;for(y=1*(n.outboundAttractionDistribution?p:1),d=0;d<b;d+=Cl)s=t[d+Dl],l=t[d+wl],h=t[d+Rl],S=Math.pow(h,n.edgeWeightInfluence),T=e[s+Ee]-e[l+Ee],_=e[s+be]-e[l+be],m===!0?(D=Math.sqrt(T*T+_*_)-e[s+Xn]-e[l+Xn],n.linLogMode?n.outboundAttractionDistribution?D>0&&(A=-y*S*Math.log(1+D)/D/e[s+fe]):D>0&&(A=-y*S*Math.log(1+D)/D):n.outboundAttractionDistribution?D>0&&(A=-y*S/e[s+fe]):D>0&&(A=-y*S)):(D=Math.sqrt(Math.pow(T,2)+Math.pow(_,2)),n.linLogMode?n.outboundAttractionDistribution?D>0&&(A=-y*S*Math.log(1+D)/D/e[s+fe]):D>0&&(A=-y*S*Math.log(1+D)/D):n.outboundAttractionDistribution?(D=1,A=-y*S/e[s+fe]):(D=1,A=-y*S)),D>0&&(e[s+he]+=T*A,e[s+ce]+=_*A,e[l+he]-=T*A,e[l+ce]-=_*A);var P,W,G,R,j,H;if(m===!0)for(o=0;o<g;o+=qe)e[o+ai]!==1&&(P=Math.sqrt(Math.pow(e[o+he],2)+Math.pow(e[o+ce],2)),P>za&&(e[o+he]=e[o+he]*za/P,e[o+ce]=e[o+ce]*za/P),W=e[o+fe]*Math.sqrt((e[o+at]-e[o+he])*(e[o+at]-e[o+he])+(e[o+rt]-e[o+ce])*(e[o+rt]-e[o+ce])),G=Math.sqrt((e[o+at]+e[o+he])*(e[o+at]+e[o+he])+(e[o+rt]+e[o+ce])*(e[o+rt]+e[o+ce]))/2,R=.1*Math.log(1+G)/(1+Math.sqrt(W)),j=e[o+Ee]+e[o+he]*(R/n.slowDown),e[o+Ee]=j,H=e[o+be]+e[o+ce]*(R/n.slowDown),e[o+be]=H);else for(o=0;o<g;o+=qe)e[o+ai]!==1&&(W=e[o+fe]*Math.sqrt((e[o+at]-e[o+he])*(e[o+at]-e[o+he])+(e[o+rt]-e[o+ce])*(e[o+rt]-e[o+ce])),G=Math.sqrt((e[o+at]+e[o+he])*(e[o+at]+e[o+he])+(e[o+rt]+e[o+ce])*(e[o+rt]+e[o+ce]))/2,R=e[o+ni]*Math.log(1+G)/(1+Math.sqrt(W)),e[o+ni]=Math.min(1,Math.sqrt(R*(Math.pow(e[o+he],2)+Math.pow(e[o+ce],2))/(1+Math.sqrt(W)))),j=e[o+Ee]+e[o+he]*(R/n.slowDown),e[o+Ee]=j,H=e[o+be]+e[o+ce]*(R/n.slowDown),e[o+be]=H);return{}}});var Na=et(dt=>{var cn=10,si=3;dt.assign=function(a){a=a||{};var n=Array.prototype.slice.call(arguments).slice(1),e,t,r;for(e=0,r=n.length;e<r;e++)if(n[e])for(t in n[e])a[t]=n[e][t];return a};dt.validateSettings=function(a){return"linLogMode"in a&&typeof a.linLogMode!="boolean"?{message:"the `linLogMode` setting should be a boolean."}:"outboundAttractionDistribution"in a&&typeof a.outboundAttractionDistribution!="boolean"?{message:"the `outboundAttractionDistribution` setting should be a boolean."}:"adjustSizes"in a&&typeof a.adjustSizes!="boolean"?{message:"the `adjustSizes` setting should be a boolean."}:"edgeWeightInfluence"in a&&typeof a.edgeWeightInfluence!="number"?{message:"the `edgeWeightInfluence` setting should be a number."}:"scalingRatio"in a&&!(typeof a.scalingRatio=="number"&&a.scalingRatio>=0)?{message:"the `scalingRatio` setting should be a number >= 0."}:"strongGravityMode"in a&&typeof a.strongGravityMode!="boolean"?{message:"the `strongGravityMode` setting should be a boolean."}:"gravity"in a&&!(typeof a.gravity=="number"&&a.gravity>=0)?{message:"the `gravity` setting should be a number >= 0."}:"slowDown"in a&&!(typeof a.slowDown=="number"||a.slowDown>=0)?{message:"the `slowDown` setting should be a number >= 0."}:"barnesHutOptimize"in a&&typeof a.barnesHutOptimize!="boolean"?{message:"the `barnesHutOptimize` setting should be a boolean."}:"barnesHutTheta"in a&&!(typeof a.barnesHutTheta=="number"&&a.barnesHutTheta>=0)?{message:"the `barnesHutTheta` setting should be a number >= 0."}:null};dt.graphToByteArrays=function(a,n){var e=a.order,t=a.size,r={},i,o=new Float32Array(e*cn),s=new Float32Array(t*si);return i=0,a.forEachNode(function(l,u){r[l]=i,o[i]=u.x,o[i+1]=u.y,o[i+2]=0,o[i+3]=0,o[i+4]=0,o[i+5]=0,o[i+6]=1,o[i+7]=1,o[i+8]=u.size||1,o[i+9]=u.fixed?1:0,i+=cn}),i=0,a.forEachEdge(function(l,u,d,h,c,f,g){var b=r[d],m=r[h],v=n(l,u,d,h,c,f,g);o[b+6]+=v,o[m+6]+=v,s[i]=b,s[i+1]=m,s[i+2]=v,i+=si}),{nodes:o,edges:s}};dt.assignLayoutChanges=function(a,n,e){var t=0;a.updateEachNodeAttributes(function(r,i){return i.x=n[t],i.y=n[t+1],t+=cn,e?e(r,i):i})};dt.readGraphPositions=function(a,n){var e=0;a.forEachNode(function(t,r){n[e]=r.x,n[e+1]=r.y,e+=cn})};dt.collectLayoutChanges=function(a,n,e){for(var t=a.nodes(),r={},i=0,o=0,s=n.length;i<s;i+=cn){if(e){var l=Object.assign({},a.getNodeAttributes(t[o]));l.x=n[i],l.y=n[i+1],l=e(t[o],l),r[t[o]]={x:l.x,y:l.y}}else r[t[o]]={x:n[i],y:n[i+1]};o++}return r};dt.createWorker=function(n){var e=window.URL||window.webkitURL,t=n.toString(),r=e.createObjectURL(new Blob(["("+t+").call(this);"],{type:"text/javascript"})),i=new Worker(r);return e.revokeObjectURL(r),i}});var Ma=et((Rc,li)=>{li.exports={linLogMode:!1,outboundAttractionDistribution:!1,adjustSizes:!1,edgeWeightInfluence:1,scalingRatio:1,strongGravityMode:!1,gravity:1,slowDown:1,barnesHutOptimize:!1,barnesHutTheta:.5}});var hi=et((Cc,di)=>{var Ll=hn(),kl=Ga().createEdgeWeightGetter,Pl=oi(),fn=Na(),Fl=Ma();function ui(a,n,e){if(!Ll(n))throw new Error("graphology-layout-forceatlas2: the given graph is not a valid graphology instance.");typeof e=="number"&&(e={iterations:e});var t=e.iterations;if(typeof t!="number")throw new Error("graphology-layout-forceatlas2: invalid number of iterations.");if(t<=0)throw new Error("graphology-layout-forceatlas2: you should provide a positive number of iterations.");var r=kl("getEdgeWeight"in e?e.getEdgeWeight:"weight").fromEntry,i=typeof e.outputReducer=="function"?e.outputReducer:null,o=fn.assign({},Fl,e.settings),s=fn.validateSettings(o);if(s)throw new Error("graphology-layout-forceatlas2: "+s.message);var l=fn.graphToByteArrays(n,r),u;for(u=0;u<t;u++)Pl(o,l.nodes,l.edges);if(a){fn.assignLayoutChanges(n,l.nodes,i);return}return fn.collectLayoutChanges(n,l.nodes)}function Il(a){var n=typeof a=="number"?a:a.order;return{barnesHutOptimize:n>2e3,strongGravityMode:!0,gravity:.05,scalingRatio:10,slowDown:1+Math.log(n)}}var Wa=ui.bind(null,!1);Wa.assign=ui.bind(null,!0);Wa.inferSettings=Il;di.exports=Wa});var fi=et((Lc,ci)=>{ci.exports=function(){var n,e,t={};(function(){var i=0,o=1,s=2,l=3,u=4,d=5,h=6,c=7,f=8,g=9,b=0,m=1,v=2,p=0,y=1,T=2,_=3,S=4,D=5,A=6,E=7,L=8,z=3,C=10,N=3,k=9,F=10;t.exports=function(I,x,P){var W,G,R,j,H,Y,Te,de,q,Re,V=x.length,te=P.length,ue=I.adjustSizes,pe=I.barnesHutTheta*I.barnesHutTheta,Le,se,ee,ie,Fe,J,K,B=[];for(R=0;R<V;R+=C)x[R+u]=x[R+s],x[R+d]=x[R+l],x[R+s]=0,x[R+l]=0;if(I.outboundAttractionDistribution){for(Le=0,R=0;R<V;R+=C)Le+=x[R+h];Le/=V/C}if(I.barnesHutOptimize){var yt=1/0,Ft=-1/0,_t=1/0,It=-1/0,We,Gt,wa;for(R=0;R<V;R+=C)yt=Math.min(yt,x[R+i]),Ft=Math.max(Ft,x[R+i]),_t=Math.min(_t,x[R+o]),It=Math.max(It,x[R+o]);var kn=Ft-yt,Pn=It-_t;for(kn>Pn?(_t-=(kn-Pn)/2,It=_t+kn):(yt-=(Pn-kn)/2,Ft=yt+Pn),B[0+p]=-1,B[0+y]=(yt+Ft)/2,B[0+T]=(_t+It)/2,B[0+_]=Math.max(Ft-yt,It-_t),B[0+S]=-1,B[0+D]=-1,B[0+A]=0,B[0+E]=0,B[0+L]=0,W=1,R=0;R<V;R+=C)for(G=0,wa=z;;)if(B[G+D]>=0){x[R+i]<B[G+y]?x[R+o]<B[G+T]?We=B[G+D]:We=B[G+D]+k:x[R+o]<B[G+T]?We=B[G+D]+k*2:We=B[G+D]+k*3,B[G+E]=(B[G+E]*B[G+A]+x[R+i]*x[R+h])/(B[G+A]+x[R+h]),B[G+L]=(B[G+L]*B[G+A]+x[R+o]*x[R+h])/(B[G+A]+x[R+h]),B[G+A]+=x[R+h],G=We;continue}else if(B[G+p]<0){B[G+p]=R;break}else{if(B[G+D]=W*k,de=B[G+_]/2,q=B[G+D],B[q+p]=-1,B[q+y]=B[G+y]-de,B[q+T]=B[G+T]-de,B[q+_]=de,B[q+S]=q+k,B[q+D]=-1,B[q+A]=0,B[q+E]=0,B[q+L]=0,q+=k,B[q+p]=-1,B[q+y]=B[G+y]-de,B[q+T]=B[G+T]+de,B[q+_]=de,B[q+S]=q+k,B[q+D]=-1,B[q+A]=0,B[q+E]=0,B[q+L]=0,q+=k,B[q+p]=-1,B[q+y]=B[G+y]+de,B[q+T]=B[G+T]-de,B[q+_]=de,B[q+S]=q+k,B[q+D]=-1,B[q+A]=0,B[q+E]=0,B[q+L]=0,q+=k,B[q+p]=-1,B[q+y]=B[G+y]+de,B[q+T]=B[G+T]+de,B[q+_]=de,B[q+S]=B[G+S],B[q+D]=-1,B[q+A]=0,B[q+E]=0,B[q+L]=0,W+=4,x[B[G+p]+i]<B[G+y]?x[B[G+p]+o]<B[G+T]?We=B[G+D]:We=B[G+D]+k:x[B[G+p]+o]<B[G+T]?We=B[G+D]+k*2:We=B[G+D]+k*3,B[G+A]=x[B[G+p]+h],B[G+E]=x[B[G+p]+i],B[G+L]=x[B[G+p]+o],B[We+p]=B[G+p],B[G+p]=-1,x[R+i]<B[G+y]?x[R+o]<B[G+T]?Gt=B[G+D]:Gt=B[G+D]+k:x[R+o]<B[G+T]?Gt=B[G+D]+k*2:Gt=B[G+D]+k*3,We===Gt)if(wa--){G=We;continue}else{wa=z;break}B[Gt+p]=R;break}}if(I.barnesHutOptimize)for(se=I.scalingRatio,R=0;R<V;R+=C)for(G=0;;)if(B[G+D]>=0)if(J=Math.pow(x[R+i]-B[G+E],2)+Math.pow(x[R+o]-B[G+L],2),Re=B[G+_],4*Re*Re/J<pe){if(ee=x[R+i]-B[G+E],ie=x[R+o]-B[G+L],ue===!0?J>0?(K=se*x[R+h]*B[G+A]/J,x[R+s]+=ee*K,x[R+l]+=ie*K):J<0&&(K=-se*x[R+h]*B[G+A]/Math.sqrt(J),x[R+s]+=ee*K,x[R+l]+=ie*K):J>0&&(K=se*x[R+h]*B[G+A]/J,x[R+s]+=ee*K,x[R+l]+=ie*K),G=B[G+S],G<0)break;continue}else{G=B[G+D];continue}else{if(Y=B[G+p],Y>=0&&Y!==R&&(ee=x[R+i]-x[Y+i],ie=x[R+o]-x[Y+o],J=ee*ee+ie*ie,ue===!0?J>0?(K=se*x[R+h]*x[Y+h]/J,x[R+s]+=ee*K,x[R+l]+=ie*K):J<0&&(K=-se*x[R+h]*x[Y+h]/Math.sqrt(J),x[R+s]+=ee*K,x[R+l]+=ie*K):J>0&&(K=se*x[R+h]*x[Y+h]/J,x[R+s]+=ee*K,x[R+l]+=ie*K)),G=B[G+S],G<0)break;continue}else for(se=I.scalingRatio,j=0;j<V;j+=C)for(H=0;H<j;H+=C)ee=x[j+i]-x[H+i],ie=x[j+o]-x[H+o],ue===!0?(J=Math.sqrt(ee*ee+ie*ie)-x[j+f]-x[H+f],J>0?(K=se*x[j+h]*x[H+h]/J/J,x[j+s]+=ee*K,x[j+l]+=ie*K,x[H+s]-=ee*K,x[H+l]-=ie*K):J<0&&(K=100*se*x[j+h]*x[H+h],x[j+s]+=ee*K,x[j+l]+=ie*K,x[H+s]-=ee*K,x[H+l]-=ie*K)):(J=Math.sqrt(ee*ee+ie*ie),J>0&&(K=se*x[j+h]*x[H+h]/J/J,x[j+s]+=ee*K,x[j+l]+=ie*K,x[H+s]-=ee*K,x[H+l]-=ie*K));for(q=I.gravity/I.scalingRatio,se=I.scalingRatio,R=0;R<V;R+=C)K=0,ee=x[R+i],ie=x[R+o],J=Math.sqrt(Math.pow(ee,2)+Math.pow(ie,2)),I.strongGravityMode?J>0&&(K=se*x[R+h]*q):J>0&&(K=se*x[R+h]*q/J),x[R+s]-=ee*K,x[R+l]-=ie*K;for(se=1*(I.outboundAttractionDistribution?Le:1),Te=0;Te<te;Te+=N)j=P[Te+b],H=P[Te+m],de=P[Te+v],Fe=Math.pow(de,I.edgeWeightInfluence),ee=x[j+i]-x[H+i],ie=x[j+o]-x[H+o],ue===!0?(J=Math.sqrt(ee*ee+ie*ie)-x[j+f]-x[H+f],I.linLogMode?I.outboundAttractionDistribution?J>0&&(K=-se*Fe*Math.log(1+J)/J/x[j+h]):J>0&&(K=-se*Fe*Math.log(1+J)/J):I.outboundAttractionDistribution?J>0&&(K=-se*Fe/x[j+h]):J>0&&(K=-se*Fe)):(J=Math.sqrt(Math.pow(ee,2)+Math.pow(ie,2)),I.linLogMode?I.outboundAttractionDistribution?J>0&&(K=-se*Fe*Math.log(1+J)/J/x[j+h]):J>0&&(K=-se*Fe*Math.log(1+J)/J):I.outboundAttractionDistribution?(J=1,K=-se*Fe/x[j+h]):(J=1,K=-se*Fe)),J>0&&(x[j+s]+=ee*K,x[j+l]+=ie*K,x[H+s]-=ee*K,x[H+l]-=ie*K);var Fn,rn,In,xt,Gn,zn;if(ue===!0)for(R=0;R<V;R+=C)x[R+g]!==1&&(Fn=Math.sqrt(Math.pow(x[R+s],2)+Math.pow(x[R+l],2)),Fn>F&&(x[R+s]=x[R+s]*F/Fn,x[R+l]=x[R+l]*F/Fn),rn=x[R+h]*Math.sqrt((x[R+u]-x[R+s])*(x[R+u]-x[R+s])+(x[R+d]-x[R+l])*(x[R+d]-x[R+l])),In=Math.sqrt((x[R+u]+x[R+s])*(x[R+u]+x[R+s])+(x[R+d]+x[R+l])*(x[R+d]+x[R+l]))/2,xt=.1*Math.log(1+In)/(1+Math.sqrt(rn)),Gn=x[R+i]+x[R+s]*(xt/I.slowDown),x[R+i]=Gn,zn=x[R+o]+x[R+l]*(xt/I.slowDown),x[R+o]=zn);else for(R=0;R<V;R+=C)x[R+g]!==1&&(rn=x[R+h]*Math.sqrt((x[R+u]-x[R+s])*(x[R+u]-x[R+s])+(x[R+d]-x[R+l])*(x[R+d]-x[R+l])),In=Math.sqrt((x[R+u]+x[R+s])*(x[R+u]+x[R+s])+(x[R+d]+x[R+l])*(x[R+d]+x[R+l]))/2,xt=x[R+c]*Math.log(1+In)/(1+Math.sqrt(rn)),x[R+c]=Math.min(1,Math.sqrt(xt*(Math.pow(x[R+s],2)+Math.pow(x[R+l],2))/(1+Math.sqrt(rn)))),Gn=x[R+i]+x[R+s]*(xt/I.slowDown),x[R+i]=Gn,zn=x[R+o]+x[R+l]*(xt/I.slowDown),x[R+o]=zn);return{}}})();var r=t.exports;self.addEventListener("message",function(i){var o=i.data;n=new Float32Array(o.nodes),o.edges&&(e=new Float32Array(o.edges)),r(o.settings,n,e),self.postMessage({nodes:n.buffer},[n.buffer])})}});var vi=et((kc,gi)=>{var Gl=fi(),zl=hn(),Nl=Ga().createEdgeWeightGetter,Ut=Na(),Ml=Ma();function ht(a,n){if(n=n||{},!zl(a))throw new Error("graphology-layout-forceatlas2/worker: the given graph is not a valid graphology instance.");var e=Nl("getEdgeWeight"in n?n.getEdgeWeight:"weight").fromEntry,t=Ut.assign({},Ml,n.settings),r=Ut.validateSettings(t);if(r)throw new Error("graphology-layout-forceatlas2/worker: "+r.message);this.worker=null,this.graph=a,this.settings=t,this.getEdgeWeight=e,this.matrices=null,this.running=!1,this.killed=!1,this.outputReducer=typeof n.outputReducer=="function"?n.outputReducer:null,this.handleMessage=this.handleMessage.bind(this);var i=void 0,o=this;this.handleGraphUpdate=function(){o.worker&&o.worker.terminate(),i&&clearTimeout(i),i=setTimeout(function(){i=void 0,o.spawnWorker()},0)},a.on("nodeAdded",this.handleGraphUpdate),a.on("edgeAdded",this.handleGraphUpdate),a.on("nodeDropped",this.handleGraphUpdate),a.on("edgeDropped",this.handleGraphUpdate),this.spawnWorker()}ht.prototype.isRunning=function(){return this.running};ht.prototype.spawnWorker=function(){this.worker&&this.worker.terminate(),this.worker=Ut.createWorker(Gl),this.worker.addEventListener("message",this.handleMessage),this.running&&(this.running=!1,this.start())};ht.prototype.handleMessage=function(a){if(this.running){var n=new Float32Array(a.data.nodes);Ut.assignLayoutChanges(this.graph,n,this.outputReducer),this.outputReducer&&Ut.readGraphPositions(this.graph,n),this.matrices.nodes=n,this.askForIterations()}};ht.prototype.askForIterations=function(a){var n=this.matrices,e={settings:this.settings,nodes:n.nodes.buffer},t=[n.nodes.buffer];return a&&(e.edges=n.edges.buffer,t.push(n.edges.buffer)),this.worker.postMessage(e,t),this};ht.prototype.start=function(){if(this.killed)throw new Error("graphology-layout-forceatlas2/worker.start: layout was killed.");return this.running?this:(this.matrices=Ut.graphToByteArrays(this.graph,this.getEdgeWeight),this.running=!0,this.askForIterations(!0),this)};ht.prototype.stop=function(){return this.running=!1,this};ht.prototype.kill=function(){if(this.killed)return this;this.running=!1,this.killed=!0,this.matrices=null,this.worker.terminate(),this.graph.removeListener("nodeAdded",this.handleGraphUpdate),this.graph.removeListener("edgeAdded",this.handleGraphUpdate),this.graph.removeListener("nodeDropped",this.handleGraphUpdate),this.graph.removeListener("edgeDropped",this.handleGraphUpdate)};gi.exports=ht});var $r=lt(sn(),1);function ys(){let a=arguments[0];for(let n=1,e=arguments.length;n<e;n++)if(arguments[n])for(let t in arguments[n])a[t]=arguments[n][t];return a}var Ae=ys;typeof Object.assign=="function"&&(Ae=Object.assign);function Ue(a,n,e,t){let r=a._nodes.get(n),i=null;return r&&(t==="mixed"?i=r.out&&r.out[e]||r.undirected&&r.undirected[e]:t==="directed"?i=r.out&&r.out[e]:i=r.undirected&&r.undirected[e]),i}function Ce(a){return typeof a=="object"&&a!==null}function Vr(a){let n;for(n in a)return!1;return!0}function Be(a,n,e){Object.defineProperty(a,n,{enumerable:!1,configurable:!1,writable:!0,value:e})}function Xe(a,n,e){let t={enumerable:!0,configurable:!0};typeof e=="function"?t.get=e:(t.value=e,t.writable=!1),Object.defineProperty(a,n,t)}function Ur(a){return!(!Ce(a)||a.attributes&&!Array.isArray(a.attributes))}function _s(){let a=Math.floor(Math.random()*256)&255;return()=>a++}function tt(){let a=arguments,n=null,e=-1;return{[Symbol.iterator](){return this},next(){let t=null;do{if(n===null){if(e++,e>=a.length)return{done:!0};n=a[e][Symbol.iterator]()}if(t=n.next(),t.done){n=null;continue}break}while(!0);return t}}}function Mt(){return{[Symbol.iterator](){return this},next(){return{done:!0}}}}var dn=class extends Error{constructor(n){super(),this.name="GraphError",this.message=n}},X=class a extends dn{constructor(n){super(n),this.name="InvalidArgumentsGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},$=class a extends dn{constructor(n){super(n),this.name="NotFoundGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},Q=class a extends dn{constructor(n){super(n),this.name="UsageGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}};function jr(a,n){this.key=a,this.attributes=n,this.clear()}jr.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.undirectedDegree=0,this.undirectedLoops=0,this.directedLoops=0,this.in={},this.out={},this.undirected={}};function Xr(a,n){this.key=a,this.attributes=n,this.clear()}Xr.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.directedLoops=0,this.in={},this.out={}};function qr(a,n){this.key=a,this.attributes=n,this.clear()}qr.prototype.clear=function(){this.undirectedDegree=0,this.undirectedLoops=0,this.undirected={}};function Wt(a,n,e,t,r){this.key=n,this.attributes=r,this.undirected=a,this.source=e,this.target=t}Wt.prototype.attach=function(){let a="out",n="in";this.undirected&&(a=n="undirected");let e=this.source.key,t=this.target.key;this.source[a][t]=this,!(this.undirected&&e===t)&&(this.target[n][e]=this)};Wt.prototype.attachMulti=function(){let a="out",n="in",e=this.source.key,t=this.target.key;this.undirected&&(a=n="undirected");let r=this.source[a],i=r[t];if(typeof i>"u"){r[t]=this,this.undirected&&e===t||(this.target[n][e]=this);return}i.previous=this,this.next=i,r[t]=this,this.target[n][e]=this};Wt.prototype.detach=function(){let a=this.source.key,n=this.target.key,e="out",t="in";this.undirected&&(e=t="undirected"),delete this.source[e][n],delete this.target[t][a]};Wt.prototype.detachMulti=function(){let a=this.source.key,n=this.target.key,e="out",t="in";this.undirected&&(e=t="undirected"),this.previous===void 0?this.next===void 0?(delete this.source[e][n],delete this.target[t][a]):(this.next.previous=void 0,this.source[e][n]=this.next,this.target[t][a]=this.next):(this.previous.next=this.next,this.next!==void 0&&(this.next.previous=this.previous))};var Kr=0,Yr=1,xs=2,Zr=3;function nt(a,n,e,t,r,i,o){let s,l,u,d;if(t=""+t,e===Kr){if(s=a._nodes.get(t),!s)throw new $(`Graph.${n}: could not find the "${t}" node in the graph.`);u=r,d=i}else if(e===Zr){if(r=""+r,l=a._edges.get(r),!l)throw new $(`Graph.${n}: could not find the "${r}" edge in the graph.`);let h=l.source.key,c=l.target.key;if(t===h)s=l.target;else if(t===c)s=l.source;else throw new $(`Graph.${n}: the "${t}" node is not attached to the "${r}" edge (${h}, ${c}).`);u=i,d=o}else{if(l=a._edges.get(t),!l)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`);e===Yr?s=l.source:s=l.target,u=r,d=i}return[s,u,d]}function Ts(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);return o.attributes[s]}}function Es(a,n,e){a.prototype[n]=function(t,r){let[i]=nt(this,n,e,t,r);return i.attributes}}function Ss(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);return o.attributes.hasOwnProperty(s)}}function As(a,n,e){a.prototype[n]=function(t,r,i,o){let[s,l,u]=nt(this,n,e,t,r,i,o);return s.attributes[l]=u,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function Ds(a,n,e){a.prototype[n]=function(t,r,i,o){let[s,l,u]=nt(this,n,e,t,r,i,o);if(typeof u!="function")throw new X(`Graph.${n}: updater should be a function.`);let d=s.attributes,h=u(d[l]);return d[l]=h,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function ws(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);return delete o.attributes[s],this.emit("nodeAttributesUpdated",{key:o.key,type:"remove",attributes:o.attributes,name:s}),this}}function Rs(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);if(!Ce(s))throw new X(`Graph.${n}: provided attributes are not a plain object.`);return o.attributes=s,this.emit("nodeAttributesUpdated",{key:o.key,type:"replace",attributes:o.attributes}),this}}function Cs(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);if(!Ce(s))throw new X(`Graph.${n}: provided attributes are not a plain object.`);return Ae(o.attributes,s),this.emit("nodeAttributesUpdated",{key:o.key,type:"merge",attributes:o.attributes,data:s}),this}}function Ls(a,n,e){a.prototype[n]=function(t,r,i){let[o,s]=nt(this,n,e,t,r,i);if(typeof s!="function")throw new X(`Graph.${n}: provided updater is not a function.`);return o.attributes=s(o.attributes),this.emit("nodeAttributesUpdated",{key:o.key,type:"update",attributes:o.attributes}),this}}var ks=[{name:a=>`get${a}Attribute`,attacher:Ts},{name:a=>`get${a}Attributes`,attacher:Es},{name:a=>`has${a}Attribute`,attacher:Ss},{name:a=>`set${a}Attribute`,attacher:As},{name:a=>`update${a}Attribute`,attacher:Ds},{name:a=>`remove${a}Attribute`,attacher:ws},{name:a=>`replace${a}Attributes`,attacher:Rs},{name:a=>`merge${a}Attributes`,attacher:Cs},{name:a=>`update${a}Attributes`,attacher:Ls}];function Ps(a){ks.forEach(function({name:n,attacher:e}){e(a,n("Node"),Kr),e(a,n("Source"),Yr),e(a,n("Target"),xs),e(a,n("Opposite"),Zr)})}function Fs(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}return i.attributes[r]}}function Is(a,n,e){a.prototype[n]=function(t){let r;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>1){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let i=""+t,o=""+arguments[1];if(r=Ue(this,i,o,e),!r)throw new $(`Graph.${n}: could not find an edge for the given path ("${i}" - "${o}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,r=this._edges.get(t),!r)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}return r.attributes}}function Gs(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}return i.attributes.hasOwnProperty(r)}}function zs(a,n,e){a.prototype[n]=function(t,r,i){let o;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+t,l=""+r;if(r=arguments[2],i=arguments[3],o=Ue(this,s,l,e),!o)throw new $(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,o=this._edges.get(t),!o)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}return o.attributes[r]=i,this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function Ns(a,n,e){a.prototype[n]=function(t,r,i){let o;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+t,l=""+r;if(r=arguments[2],i=arguments[3],o=Ue(this,s,l,e),!o)throw new $(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,o=this._edges.get(t),!o)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}if(typeof i!="function")throw new X(`Graph.${n}: updater should be a function.`);return o.attributes[r]=i(o.attributes[r]),this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function Ms(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}return delete i.attributes[r],this.emit("edgeAttributesUpdated",{key:i.key,type:"remove",attributes:i.attributes,name:r}),this}}function Ws(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}if(!Ce(r))throw new X(`Graph.${n}: provided attributes are not a plain object.`);return i.attributes=r,this.emit("edgeAttributesUpdated",{key:i.key,type:"replace",attributes:i.attributes}),this}}function Bs(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}if(!Ce(r))throw new X(`Graph.${n}: provided attributes are not a plain object.`);return Ae(i.attributes,r),this.emit("edgeAttributesUpdated",{key:i.key,type:"merge",attributes:i.attributes,data:r}),this}}function Us(a,n,e){a.prototype[n]=function(t,r){let i;if(this.type!=="mixed"&&e!=="mixed"&&e!==this.type)throw new Q(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new Q(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+t,s=""+r;if(r=arguments[2],i=Ue(this,o,s,e),!i)throw new $(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(e!=="mixed")throw new Q(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(t=""+t,i=this._edges.get(t),!i)throw new $(`Graph.${n}: could not find the "${t}" edge in the graph.`)}if(typeof r!="function")throw new X(`Graph.${n}: provided updater is not a function.`);return i.attributes=r(i.attributes),this.emit("edgeAttributesUpdated",{key:i.key,type:"update",attributes:i.attributes}),this}}var Os=[{name:a=>`get${a}Attribute`,attacher:Fs},{name:a=>`get${a}Attributes`,attacher:Is},{name:a=>`has${a}Attribute`,attacher:Gs},{name:a=>`set${a}Attribute`,attacher:zs},{name:a=>`update${a}Attribute`,attacher:Ns},{name:a=>`remove${a}Attribute`,attacher:Ms},{name:a=>`replace${a}Attributes`,attacher:Ws},{name:a=>`merge${a}Attributes`,attacher:Bs},{name:a=>`update${a}Attributes`,attacher:Us}];function Hs(a){Os.forEach(function({name:n,attacher:e}){e(a,n("Edge"),"mixed"),e(a,n("DirectedEdge"),"directed"),e(a,n("UndirectedEdge"),"undirected")})}var $s=[{name:"edges",type:"mixed"},{name:"inEdges",type:"directed",direction:"in"},{name:"outEdges",type:"directed",direction:"out"},{name:"inboundEdges",type:"mixed",direction:"in"},{name:"outboundEdges",type:"mixed",direction:"out"},{name:"directedEdges",type:"directed"},{name:"undirectedEdges",type:"undirected"}];function Vs(a,n,e,t){let r=!1;for(let i in n){if(i===t)continue;let o=n[i];if(r=e(o.key,o.attributes,o.source.key,o.target.key,o.source.attributes,o.target.attributes,o.undirected),a&&r)return o.key}}function js(a,n,e,t){let r,i,o,s=!1;for(let l in n)if(l!==t){r=n[l];do{if(i=r.source,o=r.target,s=e(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected),a&&s)return r.key;r=r.next}while(r!==void 0)}}function La(a,n){let e=Object.keys(a),t=e.length,r,i=0;return{[Symbol.iterator](){return this},next(){do if(r)r=r.next;else{if(i>=t)return{done:!0};let o=e[i++];if(o===n){r=void 0;continue}r=a[o]}while(!r);return{done:!1,value:{edge:r.key,attributes:r.attributes,source:r.source.key,target:r.target.key,sourceAttributes:r.source.attributes,targetAttributes:r.target.attributes,undirected:r.undirected}}}}}function Xs(a,n,e,t){let r=n[e];if(!r)return;let i=r.source,o=r.target;if(t(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected)&&a)return r.key}function qs(a,n,e,t){let r=n[e];if(!r)return;let i=!1;do{if(i=t(r.key,r.attributes,r.source.key,r.target.key,r.source.attributes,r.target.attributes,r.undirected),a&&i)return r.key;r=r.next}while(r!==void 0)}function ka(a,n){let e=a[n];if(e.next!==void 0)return{[Symbol.iterator](){return this},next(){if(!e)return{done:!0};let r={edge:e.key,attributes:e.attributes,source:e.source.key,target:e.target.key,sourceAttributes:e.source.attributes,targetAttributes:e.target.attributes,undirected:e.undirected};return e=e.next,{done:!1,value:r}}};let t=!1;return{[Symbol.iterator](){return this},next(){return t===!0?{done:!0}:(t=!0,{done:!1,value:{edge:e.key,attributes:e.attributes,source:e.source.key,target:e.target.key,sourceAttributes:e.source.attributes,targetAttributes:e.target.attributes,undirected:e.undirected}})}}}function Ks(a,n){if(a.size===0)return[];if(n==="mixed"||n===a.type)return Array.from(a._edges.keys());let e=n==="undirected"?a.undirectedSize:a.directedSize,t=new Array(e),r=n==="undirected",i=a._edges.values(),o=0,s,l;for(;s=i.next(),s.done!==!0;)l=s.value,l.undirected===r&&(t[o++]=l.key);return t}function Qr(a,n,e,t){if(n.size===0)return;let r=e!=="mixed"&&e!==n.type,i=e==="undirected",o,s,l=!1,u=n._edges.values();for(;o=u.next(),o.done!==!0;){if(s=o.value,r&&s.undirected!==i)continue;let{key:d,attributes:h,source:c,target:f}=s;if(l=t(d,h,c.key,f.key,c.attributes,f.attributes,s.undirected),a&&l)return d}}function Ys(a,n){if(a.size===0)return Mt();let e=n!=="mixed"&&n!==a.type,t=n==="undirected",r=a._edges.values();return{[Symbol.iterator](){return this},next(){let i,o;for(;;){if(i=r.next(),i.done)return i;if(o=i.value,!(e&&o.undirected!==t))break}return{value:{edge:o.key,attributes:o.attributes,source:o.source.key,target:o.target.key,sourceAttributes:o.source.attributes,targetAttributes:o.target.attributes,undirected:o.undirected},done:!1}}}}function Pa(a,n,e,t,r,i){let o=n?js:Vs,s;if(e!=="undirected"&&(t!=="out"&&(s=o(a,r.in,i),a&&s)||t!=="in"&&(s=o(a,r.out,i,t?void 0:r.key),a&&s))||e!=="directed"&&(s=o(a,r.undirected,i),a&&s))return s}function Zs(a,n,e,t){let r=[];return Pa(!1,a,n,e,t,function(i){r.push(i)}),r}function Qs(a,n,e){let t=Mt();return a!=="undirected"&&(n!=="out"&&typeof e.in<"u"&&(t=tt(t,La(e.in))),n!=="in"&&typeof e.out<"u"&&(t=tt(t,La(e.out,n?void 0:e.key)))),a!=="directed"&&typeof e.undirected<"u"&&(t=tt(t,La(e.undirected))),t}function Fa(a,n,e,t,r,i,o){let s=e?qs:Xs,l;if(n!=="undirected"&&(typeof r.in<"u"&&t!=="out"&&(l=s(a,r.in,i,o),a&&l)||typeof r.out<"u"&&t!=="in"&&(t||r.key!==i)&&(l=s(a,r.out,i,o),a&&l))||n!=="directed"&&typeof r.undirected<"u"&&(l=s(a,r.undirected,i,o),a&&l))return l}function Js(a,n,e,t,r){let i=[];return Fa(!1,a,n,e,t,r,function(o){i.push(o)}),i}function el(a,n,e,t){let r=Mt();return a!=="undirected"&&(typeof e.in<"u"&&n!=="out"&&t in e.in&&(r=tt(r,ka(e.in,t))),typeof e.out<"u"&&n!=="in"&&t in e.out&&(n||e.key!==t)&&(r=tt(r,ka(e.out,t)))),a!=="directed"&&typeof e.undirected<"u"&&t in e.undirected&&(r=tt(r,ka(e.undirected,t))),r}function tl(a,n){let{name:e,type:t,direction:r}=n;a.prototype[e]=function(i,o){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return[];if(!arguments.length)return Ks(this,t);if(arguments.length===1){i=""+i;let s=this._nodes.get(i);if(typeof s>"u")throw new $(`Graph.${e}: could not find the "${i}" node in the graph.`);return Zs(this.multi,t==="mixed"?this.type:t,r,s)}if(arguments.length===2){i=""+i,o=""+o;let s=this._nodes.get(i);if(!s)throw new $(`Graph.${e}:  could not find the "${i}" source node in the graph.`);if(!this._nodes.has(o))throw new $(`Graph.${e}:  could not find the "${o}" target node in the graph.`);return Js(t,this.multi,r,s,o)}throw new X(`Graph.${e}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function nl(a,n){let{name:e,type:t,direction:r}=n,i="forEach"+e[0].toUpperCase()+e.slice(1,-1);a.prototype[i]=function(u,d,h){if(!(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)){if(arguments.length===1)return h=u,Qr(!1,this,t,h);if(arguments.length===2){u=""+u,h=d;let c=this._nodes.get(u);if(typeof c>"u")throw new $(`Graph.${i}: could not find the "${u}" node in the graph.`);return Pa(!1,this.multi,t==="mixed"?this.type:t,r,c,h)}if(arguments.length===3){u=""+u,d=""+d;let c=this._nodes.get(u);if(!c)throw new $(`Graph.${i}:  could not find the "${u}" source node in the graph.`);if(!this._nodes.has(d))throw new $(`Graph.${i}:  could not find the "${d}" target node in the graph.`);return Fa(!1,t,this.multi,r,c,d,h)}throw new X(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)}};let o="map"+e[0].toUpperCase()+e.slice(1);a.prototype[o]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),h;if(u.length===0){let c=0;t!=="directed"&&(c+=this.undirectedSize),t!=="undirected"&&(c+=this.directedSize),h=new Array(c);let f=0;u.push((g,b,m,v,p,y,T)=>{h[f++]=d(g,b,m,v,p,y,T)})}else h=[],u.push((c,f,g,b,m,v,p)=>{h.push(d(c,f,g,b,m,v,p))});return this[i].apply(this,u),h};let s="filter"+e[0].toUpperCase()+e.slice(1);a.prototype[s]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),h=[];return u.push((c,f,g,b,m,v,p)=>{d(c,f,g,b,m,v,p)&&h.push(c)}),this[i].apply(this,u),h};let l="reduce"+e[0].toUpperCase()+e.slice(1);a.prototype[l]=function(){let u=Array.prototype.slice.call(arguments);if(u.length<2||u.length>4)throw new X(`Graph.${l}: invalid number of arguments (expecting 2, 3 or 4 and got ${u.length}).`);if(typeof u[u.length-1]=="function"&&typeof u[u.length-2]!="function")throw new X(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let d,h;u.length===2?(d=u[0],h=u[1],u=[]):u.length===3?(d=u[1],h=u[2],u=[u[0]]):u.length===4&&(d=u[2],h=u[3],u=[u[0],u[1]]);let c=h;return u.push((f,g,b,m,v,p,y)=>{c=d(c,f,g,b,m,v,p,y)}),this[i].apply(this,u),c}}function al(a,n){let{name:e,type:t,direction:r}=n,i="find"+e[0].toUpperCase()+e.slice(1,-1);a.prototype[i]=function(l,u,d){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return!1;if(arguments.length===1)return d=l,Qr(!0,this,t,d);if(arguments.length===2){l=""+l,d=u;let h=this._nodes.get(l);if(typeof h>"u")throw new $(`Graph.${i}: could not find the "${l}" node in the graph.`);return Pa(!0,this.multi,t==="mixed"?this.type:t,r,h,d)}if(arguments.length===3){l=""+l,u=""+u;let h=this._nodes.get(l);if(!h)throw new $(`Graph.${i}:  could not find the "${l}" source node in the graph.`);if(!this._nodes.has(u))throw new $(`Graph.${i}:  could not find the "${u}" target node in the graph.`);return Fa(!0,t,this.multi,r,h,u,d)}throw new X(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)};let o="some"+e[0].toUpperCase()+e.slice(1,-1);a.prototype[o]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((h,c,f,g,b,m,v)=>u(h,c,f,g,b,m,v)),!!this[i].apply(this,l)};let s="every"+e[0].toUpperCase()+e.slice(1,-1);a.prototype[s]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((h,c,f,g,b,m,v)=>!u(h,c,f,g,b,m,v)),!this[i].apply(this,l)}}function rl(a,n){let{name:e,type:t,direction:r}=n,i=e.slice(0,-1)+"Entries";a.prototype[i]=function(o,s){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return Mt();if(!arguments.length)return Ys(this,t);if(arguments.length===1){o=""+o;let l=this._nodes.get(o);if(!l)throw new $(`Graph.${i}: could not find the "${o}" node in the graph.`);return Qs(t,r,l)}if(arguments.length===2){o=""+o,s=""+s;let l=this._nodes.get(o);if(!l)throw new $(`Graph.${i}:  could not find the "${o}" source node in the graph.`);if(!this._nodes.has(s))throw new $(`Graph.${i}:  could not find the "${s}" target node in the graph.`);return el(t,r,l,s)}throw new X(`Graph.${i}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function il(a){$s.forEach(n=>{tl(a,n),nl(a,n),al(a,n),rl(a,n)})}var ol=[{name:"neighbors",type:"mixed"},{name:"inNeighbors",type:"directed",direction:"in"},{name:"outNeighbors",type:"directed",direction:"out"},{name:"inboundNeighbors",type:"mixed",direction:"in"},{name:"outboundNeighbors",type:"mixed",direction:"out"},{name:"directedNeighbors",type:"directed"},{name:"undirectedNeighbors",type:"undirected"}];function Vn(){this.A=null,this.B=null}Vn.prototype.wrap=function(a){this.A===null?this.A=a:this.B===null&&(this.B=a)};Vn.prototype.has=function(a){return this.A!==null&&a in this.A||this.B!==null&&a in this.B};function ln(a,n,e,t,r){for(let i in t){let o=t[i],s=o.source,l=o.target,u=s===e?l:s;if(n&&n.has(u.key))continue;let d=r(u.key,u.attributes);if(a&&d)return u.key}}function Ia(a,n,e,t,r){if(n!=="mixed"){if(n==="undirected")return ln(a,null,t,t.undirected,r);if(typeof e=="string")return ln(a,null,t,t[e],r)}let i=new Vn,o;if(n!=="undirected"){if(e!=="out"){if(o=ln(a,null,t,t.in,r),a&&o)return o;i.wrap(t.in)}if(e!=="in"){if(o=ln(a,i,t,t.out,r),a&&o)return o;i.wrap(t.out)}}if(n!=="directed"&&(o=ln(a,i,t,t.undirected,r),a&&o))return o}function sl(a,n,e){if(a!=="mixed"){if(a==="undirected")return Object.keys(e.undirected);if(typeof n=="string")return Object.keys(e[n])}let t=[];return Ia(!1,a,n,e,function(r){t.push(r)}),t}function un(a,n,e){let t=Object.keys(e),r=t.length,i=0;return{[Symbol.iterator](){return this},next(){let o=null;do{if(i>=r)return a&&a.wrap(e),{done:!0};let s=e[t[i++]],l=s.source,u=s.target;if(o=l===n?u:l,a&&a.has(o.key)){o=null;continue}}while(o===null);return{done:!1,value:{neighbor:o.key,attributes:o.attributes}}}}}function ll(a,n,e){if(a!=="mixed"){if(a==="undirected")return un(null,e,e.undirected);if(typeof n=="string")return un(null,e,e[n])}let t=Mt(),r=new Vn;return a!=="undirected"&&(n!=="out"&&(t=tt(t,un(r,e,e.in))),n!=="in"&&(t=tt(t,un(r,e,e.out)))),a!=="directed"&&(t=tt(t,un(r,e,e.undirected))),t}function ul(a,n){let{name:e,type:t,direction:r}=n;a.prototype[e]=function(i){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return[];i=""+i;let o=this._nodes.get(i);if(typeof o>"u")throw new $(`Graph.${e}: could not find the "${i}" node in the graph.`);return sl(t==="mixed"?this.type:t,r,o)}}function dl(a,n){let{name:e,type:t,direction:r}=n,i="forEach"+e[0].toUpperCase()+e.slice(1,-1);a.prototype[i]=function(u,d){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return;u=""+u;let h=this._nodes.get(u);if(typeof h>"u")throw new $(`Graph.${i}: could not find the "${u}" node in the graph.`);Ia(!1,t==="mixed"?this.type:t,r,h,d)};let o="map"+e[0].toUpperCase()+e.slice(1);a.prototype[o]=function(u,d){let h=[];return this[i](u,(c,f)=>{h.push(d(c,f))}),h};let s="filter"+e[0].toUpperCase()+e.slice(1);a.prototype[s]=function(u,d){let h=[];return this[i](u,(c,f)=>{d(c,f)&&h.push(c)}),h};let l="reduce"+e[0].toUpperCase()+e.slice(1);a.prototype[l]=function(u,d,h){if(arguments.length<3)throw new X(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let c=h;return this[i](u,(f,g)=>{c=d(c,f,g)}),c}}function hl(a,n){let{name:e,type:t,direction:r}=n,i=e[0].toUpperCase()+e.slice(1,-1),o="find"+i;a.prototype[o]=function(u,d){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return;u=""+u;let h=this._nodes.get(u);if(typeof h>"u")throw new $(`Graph.${o}: could not find the "${u}" node in the graph.`);return Ia(!0,t==="mixed"?this.type:t,r,h,d)};let s="some"+i;a.prototype[s]=function(u,d){return!!this[o](u,d)};let l="every"+i;a.prototype[l]=function(u,d){return!this[o](u,(c,f)=>!d(c,f))}}function cl(a,n){let{name:e,type:t,direction:r}=n,i=e.slice(0,-1)+"Entries";a.prototype[i]=function(o){if(t!=="mixed"&&this.type!=="mixed"&&t!==this.type)return Mt();o=""+o;let s=this._nodes.get(o);if(typeof s>"u")throw new $(`Graph.${i}: could not find the "${o}" node in the graph.`);return ll(t==="mixed"?this.type:t,r,s)}}function fl(a){ol.forEach(n=>{ul(a,n),dl(a,n),hl(a,n),cl(a,n)})}function Wn(a,n,e,t,r){let i=t._nodes.values(),o=t.type,s,l,u,d,h,c,f;for(;s=i.next(),s.done!==!0;){let g=!1;if(l=s.value,o!=="undirected"){d=l.out;for(u in d){h=d[u];do{if(c=h.target,g=!0,f=r(l.key,c.key,l.attributes,c.attributes,h.key,h.attributes,h.undirected),a&&f)return h;h=h.next}while(h)}}if(o!=="directed"){d=l.undirected;for(u in d)if(!(n&&l.key>u)){h=d[u];do{if(c=h.target,c.key!==u&&(c=h.source),g=!0,f=r(l.key,c.key,l.attributes,c.attributes,h.key,h.attributes,h.undirected),a&&f)return h;h=h.next}while(h)}}if(e&&!g&&(f=r(l.key,null,l.attributes,null,null,null,null),a&&f))return null}}function gl(a,n){let e={key:a};return Vr(n.attributes)||(e.attributes=Ae({},n.attributes)),e}function vl(a,n,e){let t={key:n,source:e.source.key,target:e.target.key};return Vr(e.attributes)||(t.attributes=Ae({},e.attributes)),a==="mixed"&&e.undirected&&(t.undirected=!0),t}function pl(a){if(!Ce(a))throw new X('Graph.import: invalid serialized node. A serialized node should be a plain object with at least a "key" property.');if(!("key"in a))throw new X("Graph.import: serialized node is missing its key.");if("attributes"in a&&(!Ce(a.attributes)||a.attributes===null))throw new X("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.")}function ml(a){if(!Ce(a))throw new X('Graph.import: invalid serialized edge. A serialized edge should be a plain object with at least a "source" & "target" property.');if(!("source"in a))throw new X("Graph.import: serialized edge is missing its source.");if(!("target"in a))throw new X("Graph.import: serialized edge is missing its target.");if("attributes"in a&&(!Ce(a.attributes)||a.attributes===null))throw new X("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.");if("undirected"in a&&typeof a.undirected!="boolean")throw new X("Graph.import: invalid undirectedness information. Undirected should be boolean or omitted.")}var bl=_s(),yl=new Set(["directed","undirected","mixed"]),Or=new Set(["domain","_events","_eventsCount","_maxListeners"]),_l=[{name:a=>`${a}Edge`,generateKey:!0},{name:a=>`${a}DirectedEdge`,generateKey:!0,type:"directed"},{name:a=>`${a}UndirectedEdge`,generateKey:!0,type:"undirected"},{name:a=>`${a}EdgeWithKey`},{name:a=>`${a}DirectedEdgeWithKey`,type:"directed"},{name:a=>`${a}UndirectedEdgeWithKey`,type:"undirected"}],xl={allowSelfLoops:!0,multi:!1,type:"mixed"};function Tl(a,n,e){if(e&&!Ce(e))throw new X(`Graph.addNode: invalid attributes. Expecting an object but got "${e}"`);if(n=""+n,e=e||{},a._nodes.has(n))throw new Q(`Graph.addNode: the "${n}" node already exist in the graph.`);let t=new a.NodeDataClass(n,e);return a._nodes.set(n,t),a.emit("nodeAdded",{key:n,attributes:e}),t}function Hr(a,n,e){let t=new a.NodeDataClass(n,e);return a._nodes.set(n,t),a.emit("nodeAdded",{key:n,attributes:e}),t}function Jr(a,n,e,t,r,i,o,s){if(!t&&a.type==="undirected")throw new Q(`Graph.${n}: you cannot add a directed edge to an undirected graph. Use the #.addEdge or #.addUndirectedEdge instead.`);if(t&&a.type==="directed")throw new Q(`Graph.${n}: you cannot add an undirected edge to a directed graph. Use the #.addEdge or #.addDirectedEdge instead.`);if(s&&!Ce(s))throw new X(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`);if(i=""+i,o=""+o,s=s||{},!a.allowSelfLoops&&i===o)throw new Q(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let l=a._nodes.get(i),u=a._nodes.get(o);if(!l)throw new $(`Graph.${n}: source node "${i}" not found.`);if(!u)throw new $(`Graph.${n}: target node "${o}" not found.`);let d={key:null,undirected:t,source:i,target:o,attributes:s};if(e)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new Q(`Graph.${n}: the "${r}" edge already exists in the graph.`);if(!a.multi&&(t?typeof l.undirected[o]<"u":typeof l.out[o]<"u"))throw new Q(`Graph.${n}: an edge linking "${i}" to "${o}" already exists. If you really want to add multiple edges linking those nodes, you should create a multi graph by using the 'multi' option.`);let h=new Wt(t,r,l,u,s);a._edges.set(r,h);let c=i===o;return t?(l.undirectedDegree++,u.undirectedDegree++,c&&(l.undirectedLoops++,a._undirectedSelfLoopCount++)):(l.outDegree++,u.inDegree++,c&&(l.directedLoops++,a._directedSelfLoopCount++)),a.multi?h.attachMulti():h.attach(),t?a._undirectedSize++:a._directedSize++,d.key=r,a.emit("edgeAdded",d),r}function El(a,n,e,t,r,i,o,s,l){if(!t&&a.type==="undirected")throw new Q(`Graph.${n}: you cannot merge/update a directed edge to an undirected graph. Use the #.mergeEdge/#.updateEdge or #.addUndirectedEdge instead.`);if(t&&a.type==="directed")throw new Q(`Graph.${n}: you cannot merge/update an undirected edge to a directed graph. Use the #.mergeEdge/#.updateEdge or #.addDirectedEdge instead.`);if(s){if(l){if(typeof s!="function")throw new X(`Graph.${n}: invalid updater function. Expecting a function but got "${s}"`)}else if(!Ce(s))throw new X(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`)}i=""+i,o=""+o;let u;if(l&&(u=s,s=void 0),!a.allowSelfLoops&&i===o)throw new Q(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let d=a._nodes.get(i),h=a._nodes.get(o),c,f;if(!e&&(c=a._edges.get(r),c)){if((c.source.key!==i||c.target.key!==o)&&(!t||c.source.key!==o||c.target.key!==i))throw new Q(`Graph.${n}: inconsistency detected when attempting to merge the "${r}" edge with "${i}" source & "${o}" target vs. ("${c.source.key}", "${c.target.key}").`);f=c}if(!f&&!a.multi&&d&&(f=t?d.undirected[o]:d.out[o]),f){let p=[f.key,!1,!1,!1];if(l?!u:!s)return p;if(l){let y=f.attributes;f.attributes=u(y),a.emit("edgeAttributesUpdated",{type:"replace",key:f.key,attributes:f.attributes})}else Ae(f.attributes,s),a.emit("edgeAttributesUpdated",{type:"merge",key:f.key,attributes:f.attributes,data:s});return p}s=s||{},l&&u&&(s=u(s));let g={key:null,undirected:t,source:i,target:o,attributes:s};if(e)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new Q(`Graph.${n}: the "${r}" edge already exists in the graph.`);let b=!1,m=!1;d||(d=Hr(a,i,{}),b=!0,i===o&&(h=d,m=!0)),h||(h=Hr(a,o,{}),m=!0),c=new Wt(t,r,d,h,s),a._edges.set(r,c);let v=i===o;return t?(d.undirectedDegree++,h.undirectedDegree++,v&&(d.undirectedLoops++,a._undirectedSelfLoopCount++)):(d.outDegree++,h.inDegree++,v&&(d.directedLoops++,a._directedSelfLoopCount++)),a.multi?c.attachMulti():c.attach(),t?a._undirectedSize++:a._directedSize++,g.key=r,a.emit("edgeAdded",g),[r,!0,b,m]}function Nt(a,n){a._edges.delete(n.key);let{source:e,target:t,attributes:r}=n,i=n.undirected,o=e===t;i?(e.undirectedDegree--,t.undirectedDegree--,o&&(e.undirectedLoops--,a._undirectedSelfLoopCount--)):(e.outDegree--,t.inDegree--,o&&(e.directedLoops--,a._directedSelfLoopCount--)),a.multi?n.detachMulti():n.detach(),i?a._undirectedSize--:a._directedSize--,a.emit("edgeDropped",{key:n.key,attributes:r,source:e.key,target:t.key,undirected:i})}var ve=class a extends $r.EventEmitter{constructor(n){if(super(),n=Ae({},xl,n),typeof n.multi!="boolean")throw new X(`Graph.constructor: invalid 'multi' option. Expecting a boolean but got "${n.multi}".`);if(!yl.has(n.type))throw new X(`Graph.constructor: invalid 'type' option. Should be one of "mixed", "directed" or "undirected" but got "${n.type}".`);if(typeof n.allowSelfLoops!="boolean")throw new X(`Graph.constructor: invalid 'allowSelfLoops' option. Expecting a boolean but got "${n.allowSelfLoops}".`);let e=n.type==="mixed"?jr:n.type==="directed"?Xr:qr;Be(this,"NodeDataClass",e);let t="geid_"+bl()+"_",r=0,i=()=>{let o;do o=t+r++;while(this._edges.has(o));return o};Be(this,"_attributes",{}),Be(this,"_nodes",new Map),Be(this,"_edges",new Map),Be(this,"_directedSize",0),Be(this,"_undirectedSize",0),Be(this,"_directedSelfLoopCount",0),Be(this,"_undirectedSelfLoopCount",0),Be(this,"_edgeKeyGenerator",i),Be(this,"_options",n),Or.forEach(o=>Be(this,o,this[o])),Xe(this,"order",()=>this._nodes.size),Xe(this,"size",()=>this._edges.size),Xe(this,"directedSize",()=>this._directedSize),Xe(this,"undirectedSize",()=>this._undirectedSize),Xe(this,"selfLoopCount",()=>this._directedSelfLoopCount+this._undirectedSelfLoopCount),Xe(this,"directedSelfLoopCount",()=>this._directedSelfLoopCount),Xe(this,"undirectedSelfLoopCount",()=>this._undirectedSelfLoopCount),Xe(this,"multi",this._options.multi),Xe(this,"type",this._options.type),Xe(this,"allowSelfLoops",this._options.allowSelfLoops),Xe(this,"implementation",()=>"graphology")}_resetInstanceCounters(){this._directedSize=0,this._undirectedSize=0,this._directedSelfLoopCount=0,this._undirectedSelfLoopCount=0}hasNode(n){return this._nodes.has(""+n)}hasDirectedEdge(n,e){if(this.type==="undirected")return!1;if(arguments.length===1){let t=""+n,r=this._edges.get(t);return!!r&&!r.undirected}else if(arguments.length===2){n=""+n,e=""+e;let t=this._nodes.get(n);return t?t.out.hasOwnProperty(e):!1}throw new X(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasUndirectedEdge(n,e){if(this.type==="directed")return!1;if(arguments.length===1){let t=""+n,r=this._edges.get(t);return!!r&&r.undirected}else if(arguments.length===2){n=""+n,e=""+e;let t=this._nodes.get(n);return t?t.undirected.hasOwnProperty(e):!1}throw new X(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasEdge(n,e){if(arguments.length===1){let t=""+n;return this._edges.has(t)}else if(arguments.length===2){n=""+n,e=""+e;let t=this._nodes.get(n);return t?typeof t.out<"u"&&t.out.hasOwnProperty(e)||typeof t.undirected<"u"&&t.undirected.hasOwnProperty(e):!1}throw new X(`Graph.hasEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}directedEdge(n,e){if(this.type==="undirected")return;if(n=""+n,e=""+e,this.multi)throw new Q("Graph.directedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.directedEdges instead.");let t=this._nodes.get(n);if(!t)throw new $(`Graph.directedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(e))throw new $(`Graph.directedEdge: could not find the "${e}" target node in the graph.`);let r=t.out&&t.out[e]||void 0;if(r)return r.key}undirectedEdge(n,e){if(this.type==="directed")return;if(n=""+n,e=""+e,this.multi)throw new Q("Graph.undirectedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.undirectedEdges instead.");let t=this._nodes.get(n);if(!t)throw new $(`Graph.undirectedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(e))throw new $(`Graph.undirectedEdge: could not find the "${e}" target node in the graph.`);let r=t.undirected&&t.undirected[e]||void 0;if(r)return r.key}edge(n,e){if(this.multi)throw new Q("Graph.edge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.edges instead.");n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.edge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(e))throw new $(`Graph.edge: could not find the "${e}" target node in the graph.`);let r=t.out&&t.out[e]||t.undirected&&t.undirected[e]||void 0;if(r)return r.key}areDirectedNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areDirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:e in t.in||e in t.out}areOutNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areOutNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:e in t.out}areInNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areInNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:e in t.in}areUndirectedNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areUndirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="directed"?!1:e in t.undirected}areNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&(e in t.in||e in t.out)||this.type!=="directed"&&e in t.undirected}areInboundNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areInboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&e in t.in||this.type!=="directed"&&e in t.undirected}areOutboundNeighbors(n,e){n=""+n,e=""+e;let t=this._nodes.get(n);if(!t)throw new $(`Graph.areOutboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&e in t.out||this.type!=="directed"&&e in t.undirected}inDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.inDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.inDegree}outDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.outDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.outDegree}directedDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.directedDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.inDegree+e.outDegree}undirectedDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.undirectedDegree: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:e.undirectedDegree}inboundDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.inboundDegree: could not find the "${n}" node in the graph.`);let t=0;return this.type!=="directed"&&(t+=e.undirectedDegree),this.type!=="undirected"&&(t+=e.inDegree),t}outboundDegree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.outboundDegree: could not find the "${n}" node in the graph.`);let t=0;return this.type!=="directed"&&(t+=e.undirectedDegree),this.type!=="undirected"&&(t+=e.outDegree),t}degree(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.degree: could not find the "${n}" node in the graph.`);let t=0;return this.type!=="directed"&&(t+=e.undirectedDegree),this.type!=="undirected"&&(t+=e.inDegree+e.outDegree),t}inDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.inDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.inDegree-e.directedLoops}outDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.outDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.outDegree-e.directedLoops}directedDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.directedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:e.inDegree+e.outDegree-e.directedLoops*2}undirectedDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.undirectedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:e.undirectedDegree-e.undirectedLoops*2}inboundDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.inboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let t=0,r=0;return this.type!=="directed"&&(t+=e.undirectedDegree,r+=e.undirectedLoops*2),this.type!=="undirected"&&(t+=e.inDegree,r+=e.directedLoops),t-r}outboundDegreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.outboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let t=0,r=0;return this.type!=="directed"&&(t+=e.undirectedDegree,r+=e.undirectedLoops*2),this.type!=="undirected"&&(t+=e.outDegree,r+=e.directedLoops),t-r}degreeWithoutSelfLoops(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.degreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let t=0,r=0;return this.type!=="directed"&&(t+=e.undirectedDegree,r+=e.undirectedLoops*2),this.type!=="undirected"&&(t+=e.inDegree+e.outDegree,r+=e.directedLoops*2),t-r}source(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.source: could not find the "${n}" edge in the graph.`);return e.source.key}target(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.target: could not find the "${n}" edge in the graph.`);return e.target.key}extremities(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.extremities: could not find the "${n}" edge in the graph.`);return[e.source.key,e.target.key]}opposite(n,e){n=""+n,e=""+e;let t=this._edges.get(e);if(!t)throw new $(`Graph.opposite: could not find the "${e}" edge in the graph.`);let r=t.source.key,i=t.target.key;if(n===r)return i;if(n===i)return r;throw new $(`Graph.opposite: the "${n}" node is not attached to the "${e}" edge (${r}, ${i}).`)}hasExtremity(n,e){n=""+n,e=""+e;let t=this._edges.get(n);if(!t)throw new $(`Graph.hasExtremity: could not find the "${n}" edge in the graph.`);return t.source.key===e||t.target.key===e}isUndirected(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.isUndirected: could not find the "${n}" edge in the graph.`);return e.undirected}isDirected(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.isDirected: could not find the "${n}" edge in the graph.`);return!e.undirected}isSelfLoop(n){n=""+n;let e=this._edges.get(n);if(!e)throw new $(`Graph.isSelfLoop: could not find the "${n}" edge in the graph.`);return e.source===e.target}addNode(n,e){return Tl(this,n,e).key}mergeNode(n,e){if(e&&!Ce(e))throw new X(`Graph.mergeNode: invalid attributes. Expecting an object but got "${e}"`);n=""+n,e=e||{};let t=this._nodes.get(n);return t?(e&&(Ae(t.attributes,e),this.emit("nodeAttributesUpdated",{type:"merge",key:n,attributes:t.attributes,data:e})),[n,!1]):(t=new this.NodeDataClass(n,e),this._nodes.set(n,t),this.emit("nodeAdded",{key:n,attributes:e}),[n,!0])}updateNode(n,e){if(e&&typeof e!="function")throw new X(`Graph.updateNode: invalid updater function. Expecting a function but got "${e}"`);n=""+n;let t=this._nodes.get(n);if(t){if(e){let i=t.attributes;t.attributes=e(i),this.emit("nodeAttributesUpdated",{type:"replace",key:n,attributes:t.attributes})}return[n,!1]}let r=e?e({}):{};return t=new this.NodeDataClass(n,r),this._nodes.set(n,t),this.emit("nodeAdded",{key:n,attributes:r}),[n,!0]}dropNode(n){n=""+n;let e=this._nodes.get(n);if(!e)throw new $(`Graph.dropNode: could not find the "${n}" node in the graph.`);let t;if(this.type!=="undirected"){for(let r in e.out){t=e.out[r];do Nt(this,t),t=t.next;while(t)}for(let r in e.in){t=e.in[r];do Nt(this,t),t=t.next;while(t)}}if(this.type!=="directed")for(let r in e.undirected){t=e.undirected[r];do Nt(this,t),t=t.next;while(t)}this._nodes.delete(n),this.emit("nodeDropped",{key:n,attributes:e.attributes})}dropEdge(n){let e;if(arguments.length>1){let t=""+arguments[0],r=""+arguments[1];if(e=Ue(this,t,r,this.type),!e)throw new $(`Graph.dropEdge: could not find the "${t}" -> "${r}" edge in the graph.`)}else if(n=""+n,e=this._edges.get(n),!e)throw new $(`Graph.dropEdge: could not find the "${n}" edge in the graph.`);return Nt(this,e),this}dropDirectedEdge(n,e){if(arguments.length<2)throw new Q("Graph.dropDirectedEdge: it does not make sense to try and drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new Q("Graph.dropDirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");n=""+n,e=""+e;let t=Ue(this,n,e,"directed");if(!t)throw new $(`Graph.dropDirectedEdge: could not find a "${n}" -> "${e}" edge in the graph.`);return Nt(this,t),this}dropUndirectedEdge(n,e){if(arguments.length<2)throw new Q("Graph.dropUndirectedEdge: it does not make sense to drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new Q("Graph.dropUndirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");let t=Ue(this,n,e,"undirected");if(!t)throw new $(`Graph.dropUndirectedEdge: could not find a "${n}" -> "${e}" edge in the graph.`);return Nt(this,t),this}clear(){this._edges.clear(),this._nodes.clear(),this._resetInstanceCounters(),this.emit("cleared")}clearEdges(){let n=this._nodes.values(),e;for(;e=n.next(),e.done!==!0;)e.value.clear();this._edges.clear(),this._resetInstanceCounters(),this.emit("edgesCleared")}getAttribute(n){return this._attributes[n]}getAttributes(){return this._attributes}hasAttribute(n){return this._attributes.hasOwnProperty(n)}setAttribute(n,e){return this._attributes[n]=e,this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}updateAttribute(n,e){if(typeof e!="function")throw new X("Graph.updateAttribute: updater should be a function.");let t=this._attributes[n];return this._attributes[n]=e(t),this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}removeAttribute(n){return delete this._attributes[n],this.emit("attributesUpdated",{type:"remove",attributes:this._attributes,name:n}),this}replaceAttributes(n){if(!Ce(n))throw new X("Graph.replaceAttributes: provided attributes are not a plain object.");return this._attributes=n,this.emit("attributesUpdated",{type:"replace",attributes:this._attributes}),this}mergeAttributes(n){if(!Ce(n))throw new X("Graph.mergeAttributes: provided attributes are not a plain object.");return Ae(this._attributes,n),this.emit("attributesUpdated",{type:"merge",attributes:this._attributes,data:n}),this}updateAttributes(n){if(typeof n!="function")throw new X("Graph.updateAttributes: provided updater is not a function.");return this._attributes=n(this._attributes),this.emit("attributesUpdated",{type:"update",attributes:this._attributes}),this}updateEachNodeAttributes(n,e){if(typeof n!="function")throw new X("Graph.updateEachNodeAttributes: expecting an updater function.");if(e&&!Ur(e))throw new X("Graph.updateEachNodeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let t=this._nodes.values(),r,i;for(;r=t.next(),r.done!==!0;)i=r.value,i.attributes=n(i.key,i.attributes);this.emit("eachNodeAttributesUpdated",{hints:e||null})}updateEachEdgeAttributes(n,e){if(typeof n!="function")throw new X("Graph.updateEachEdgeAttributes: expecting an updater function.");if(e&&!Ur(e))throw new X("Graph.updateEachEdgeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let t=this._edges.values(),r,i,o,s;for(;r=t.next(),r.done!==!0;)i=r.value,o=i.source,s=i.target,i.attributes=n(i.key,i.attributes,o.key,s.key,o.attributes,s.attributes,i.undirected);this.emit("eachEdgeAttributesUpdated",{hints:e||null})}forEachAdjacencyEntry(n){if(typeof n!="function")throw new X("Graph.forEachAdjacencyEntry: expecting a callback.");Wn(!1,!1,!1,this,n)}forEachAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new X("Graph.forEachAdjacencyEntryWithOrphans: expecting a callback.");Wn(!1,!1,!0,this,n)}forEachAssymetricAdjacencyEntry(n){if(typeof n!="function")throw new X("Graph.forEachAssymetricAdjacencyEntry: expecting a callback.");Wn(!1,!0,!1,this,n)}forEachAssymetricAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new X("Graph.forEachAssymetricAdjacencyEntryWithOrphans: expecting a callback.");Wn(!1,!0,!0,this,n)}nodes(){return Array.from(this._nodes.keys())}forEachNode(n){if(typeof n!="function")throw new X("Graph.forEachNode: expecting a callback.");let e=this._nodes.values(),t,r;for(;t=e.next(),t.done!==!0;)r=t.value,n(r.key,r.attributes)}findNode(n){if(typeof n!="function")throw new X("Graph.findNode: expecting a callback.");let e=this._nodes.values(),t,r;for(;t=e.next(),t.done!==!0;)if(r=t.value,n(r.key,r.attributes))return r.key}mapNodes(n){if(typeof n!="function")throw new X("Graph.mapNode: expecting a callback.");let e=this._nodes.values(),t,r,i=new Array(this.order),o=0;for(;t=e.next(),t.done!==!0;)r=t.value,i[o++]=n(r.key,r.attributes);return i}someNode(n){if(typeof n!="function")throw new X("Graph.someNode: expecting a callback.");let e=this._nodes.values(),t,r;for(;t=e.next(),t.done!==!0;)if(r=t.value,n(r.key,r.attributes))return!0;return!1}everyNode(n){if(typeof n!="function")throw new X("Graph.everyNode: expecting a callback.");let e=this._nodes.values(),t,r;for(;t=e.next(),t.done!==!0;)if(r=t.value,!n(r.key,r.attributes))return!1;return!0}filterNodes(n){if(typeof n!="function")throw new X("Graph.filterNodes: expecting a callback.");let e=this._nodes.values(),t,r,i=[];for(;t=e.next(),t.done!==!0;)r=t.value,n(r.key,r.attributes)&&i.push(r.key);return i}reduceNodes(n,e){if(typeof n!="function")throw new X("Graph.reduceNodes: expecting a callback.");if(arguments.length<2)throw new X("Graph.reduceNodes: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.");let t=e,r=this._nodes.values(),i,o;for(;i=r.next(),i.done!==!0;)o=i.value,t=n(t,o.key,o.attributes);return t}nodeEntries(){let n=this._nodes.values();return{[Symbol.iterator](){return this},next(){let e=n.next();if(e.done)return e;let t=e.value;return{value:{node:t.key,attributes:t.attributes},done:!1}}}}export(){let n=new Array(this._nodes.size),e=0;this._nodes.forEach((r,i)=>{n[e++]=gl(i,r)});let t=new Array(this._edges.size);return e=0,this._edges.forEach((r,i)=>{t[e++]=vl(this.type,i,r)}),{options:{type:this.type,multi:this.multi,allowSelfLoops:this.allowSelfLoops},attributes:this.getAttributes(),nodes:n,edges:t}}import(n,e=!1){if(n instanceof a)return n.forEachNode((l,u)=>{e?this.mergeNode(l,u):this.addNode(l,u)}),n.forEachEdge((l,u,d,h,c,f,g)=>{e?g?this.mergeUndirectedEdgeWithKey(l,d,h,u):this.mergeDirectedEdgeWithKey(l,d,h,u):g?this.addUndirectedEdgeWithKey(l,d,h,u):this.addDirectedEdgeWithKey(l,d,h,u)}),this;if(!Ce(n))throw new X("Graph.import: invalid argument. Expecting a serialized graph or, alternatively, a Graph instance.");if(n.attributes){if(!Ce(n.attributes))throw new X("Graph.import: invalid attributes. Expecting a plain object.");e?this.mergeAttributes(n.attributes):this.replaceAttributes(n.attributes)}let t,r,i,o,s;if(n.nodes){if(i=n.nodes,!Array.isArray(i))throw new X("Graph.import: invalid nodes. Expecting an array.");for(t=0,r=i.length;t<r;t++){o=i[t],pl(o);let{key:l,attributes:u}=o;e?this.mergeNode(l,u):this.addNode(l,u)}}if(n.edges){let l=!1;if(this.type==="undirected"&&(l=!0),i=n.edges,!Array.isArray(i))throw new X("Graph.import: invalid edges. Expecting an array.");for(t=0,r=i.length;t<r;t++){s=i[t],ml(s);let{source:u,target:d,attributes:h,undirected:c=l}=s,f;"key"in s?(f=e?c?this.mergeUndirectedEdgeWithKey:this.mergeDirectedEdgeWithKey:c?this.addUndirectedEdgeWithKey:this.addDirectedEdgeWithKey,f.call(this,s.key,u,d,h)):(f=e?c?this.mergeUndirectedEdge:this.mergeDirectedEdge:c?this.addUndirectedEdge:this.addDirectedEdge,f.call(this,u,d,h))}}return this}nullCopy(n){let e=new a(Ae({},this._options,n));return e.replaceAttributes(Ae({},this.getAttributes())),e}emptyCopy(n){let e=this.nullCopy(n);return this._nodes.forEach((t,r)=>{let i=Ae({},t.attributes);t=new e.NodeDataClass(r,i),e._nodes.set(r,t)}),e}copy(n){if(n=n||{},typeof n.type=="string"&&n.type!==this.type&&n.type!=="mixed")throw new Q(`Graph.copy: cannot create an incompatible copy from "${this.type}" type to "${n.type}" because this would mean losing information about the current graph.`);if(typeof n.multi=="boolean"&&n.multi!==this.multi&&n.multi!==!0)throw new Q("Graph.copy: cannot create an incompatible copy by downgrading a multi graph to a simple one because this would mean losing information about the current graph.");if(typeof n.allowSelfLoops=="boolean"&&n.allowSelfLoops!==this.allowSelfLoops&&n.allowSelfLoops!==!0)throw new Q("Graph.copy: cannot create an incompatible copy from a graph allowing self loops to one that does not because this would mean losing information about the current graph.");let e=this.emptyCopy(n),t=this._edges.values(),r,i;for(;r=t.next(),r.done!==!0;)i=r.value,Jr(e,"copy",!1,i.undirected,i.key,i.source.key,i.target.key,Ae({},i.attributes));return e}toJSON(){return this.export()}toString(){return"[object Graph]"}inspect(){let n={};this._nodes.forEach((i,o)=>{n[o]=i.attributes});let e={},t={};this._edges.forEach((i,o)=>{let s=i.undirected?"--":"->",l="",u=i.source.key,d=i.target.key,h;i.undirected&&u>d&&(h=u,u=d,d=h);let c=`(${u})${s}(${d})`;o.startsWith("geid_")?this.multi&&(typeof t[c]>"u"?t[c]=0:t[c]++,l+=`${t[c]}. `):l+=`[${o}]: `,l+=c,e[l]=i.attributes});let r={};for(let i in this)this.hasOwnProperty(i)&&!Or.has(i)&&typeof this[i]!="function"&&typeof i!="symbol"&&(r[i]=this[i]);return r.attributes=this._attributes,r.nodes=n,r.edges=e,Be(r,"constructor",this.constructor),r}};typeof Symbol<"u"&&(ve.prototype[Symbol.for("nodejs.util.inspect.custom")]=ve.prototype.inspect);_l.forEach(a=>{["add","merge","update"].forEach(n=>{let e=a.name(n),t=n==="add"?Jr:El;a.generateKey?ve.prototype[e]=function(r,i,o){return t(this,e,!0,(a.type||this.type)==="undirected",null,r,i,o,n==="update")}:ve.prototype[e]=function(r,i,o,s){return t(this,e,!1,(a.type||this.type)==="undirected",r,i,o,s,n==="update")}})});Ps(ve);Hs(ve);il(ve);fl(ve);var Bn=class extends ve{constructor(n){let e=Ae({type:"directed"},n);if("multi"in e&&e.multi!==!1)throw new X("DirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(e.type!=="directed")throw new X('DirectedGraph.from: inconsistent "'+e.type+'" type in given options!');super(e)}},Un=class extends ve{constructor(n){let e=Ae({type:"undirected"},n);if("multi"in e&&e.multi!==!1)throw new X("UndirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(e.type!=="undirected")throw new X('UndirectedGraph.from: inconsistent "'+e.type+'" type in given options!');super(e)}},On=class extends ve{constructor(n){let e=Ae({multi:!0},n);if("multi"in e&&e.multi!==!0)throw new X("MultiGraph.from: inconsistent indication that the graph should be simple in given options!");super(e)}},Hn=class extends ve{constructor(n){let e=Ae({type:"directed",multi:!0},n);if("multi"in e&&e.multi!==!0)throw new X("MultiDirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(e.type!=="directed")throw new X('MultiDirectedGraph.from: inconsistent "'+e.type+'" type in given options!');super(e)}},$n=class extends ve{constructor(n){let e=Ae({type:"undirected",multi:!0},n);if("multi"in e&&e.multi!==!0)throw new X("MultiUndirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(e.type!=="undirected")throw new X('MultiUndirectedGraph.from: inconsistent "'+e.type+'" type in given options!');super(e)}};function Bt(a){a.from=function(n,e){let t=Ae({},n.options,e),r=new a(t);return r.import(n),r}}Bt(ve);Bt(Bn);Bt(Un);Bt(On);Bt(Hn);Bt($n);ve.Graph=ve;ve.DirectedGraph=Bn;ve.UndirectedGraph=Un;ve.MultiGraph=On;ve.MultiDirectedGraph=Hn;ve.MultiUndirectedGraph=$n;ve.InvalidArgumentsGraphError=X;ve.NotFoundGraphError=$;ve.UsageGraphError=Q;var Ln=lt(hi(),1),cs=lt(vi(),1);function Yn(a,n){(n==null||n>a.length)&&(n=a.length);for(var e=0,t=Array(n);e<n;e++)t[e]=a[e];return t}function Zn(a,n){if(a){if(typeof a=="string")return Yn(a,n);var e={}.toString.call(a).slice(8,-1);return e==="Object"&&a.constructor&&(e=a.constructor.name),e==="Map"||e==="Set"?Array.from(a):e==="Arguments"||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(e)?Yn(a,n):void 0}}function Wl(a){if(Array.isArray(a))return a}function Bl(a,n){var e=a==null?null:typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(e!=null){var t,r,i,o,s=[],l=!0,u=!1;try{if(i=(e=e.call(a)).next,n===0){if(Object(e)!==e)return;l=!1}else for(;!(l=(t=i.call(e)).done)&&(s.push(t.value),s.length!==n);l=!0);}catch(d){u=!0,r=d}finally{try{if(!l&&e.return!=null&&(o=e.return(),Object(o)!==o))return}finally{if(u)throw r}}return s}}function Ul(){throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function le(a,n){return Wl(a)||Bl(a,n)||Zn(a,n)||Ul()}function U(a,n){var e=typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(!e){if(Array.isArray(a)||(e=Zn(a))||n&&a&&typeof a.length=="number"){e&&(a=e);var t=0,r=function(){};return{s:r,n:function(){return t>=a.length?{done:!0}:{done:!1,value:a[t++]}},e:function(l){throw l},f:r}}throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}var i,o=!0,s=!1;return{s:function(){e=e.call(a)},n:function(){var l=e.next();return o=l.done,l},e:function(l){s=!0,i=l},f:function(){try{o||e.return==null||e.return()}finally{if(s)throw i}}}}function He(){return Float32Array.of(1,0,0,0,1,0,0,0,1)}function qn(a,n,e){return a[0]=n,a[4]=typeof e=="number"?e:n,a}function pi(a,n){var e=Math.sin(n),t=Math.cos(n);return a[0]=t,a[1]=e,a[3]=-e,a[4]=t,a}function mi(a,n,e){return a[6]=n,a[7]=e,a}function ct(a,n){var e=a[0],t=a[1],r=a[2],i=a[3],o=a[4],s=a[5],l=a[6],u=a[7],d=a[8],h=n[0],c=n[1],f=n[2],g=n[3],b=n[4],m=n[5],v=n[6],p=n[7],y=n[8];return a[0]=h*e+c*i+f*l,a[1]=h*t+c*o+f*u,a[2]=h*r+c*s+f*d,a[3]=g*e+b*i+m*l,a[4]=g*t+b*o+m*u,a[5]=g*r+b*s+m*d,a[6]=v*e+p*i+y*l,a[7]=v*t+p*o+y*u,a[8]=v*r+p*s+y*d,a}function Ba(a,n){var e=Math.cos(n),t=Math.sin(n);return{x:e*a.x+t*a.y,y:-t*a.x+e*a.y}}function Ht(a,n){var e=arguments.length>2&&arguments[2]!==void 0?arguments[2]:1,t=a[0],r=a[1],i=a[3],o=a[4],s=a[6],l=a[7],u=n.x,d=n.y;return{x:u*t+d*i+s*e,y:u*r+d*o+l*e}}function Ol(a,n){var e=a.height/a.width,t=n.height/n.width;return e<1&&t>1||e>1&&t<1?1:Math.min(Math.max(t,1/t),Math.max(1/e,e))}function ft(a,n,e,t,r){var i=a.angle,o=a.ratio,s=a.x,l=a.y,u=n.width,d=n.height,h=He(),c=Math.min(u,d)-2*t,f=Ol(n,e);return r?(ct(h,mi(He(),s,l)),ct(h,qn(He(),o)),ct(h,pi(He(),i)),ct(h,qn(He(),u/c/2/f,d/c/2/f))):(ct(h,qn(He(),2*(c/u)*f,2*(c/d)*f)),ct(h,pi(He(),-i)),ct(h,qn(He(),1/o)),ct(h,mi(He(),-s,-l))),h}function bi(a,n,e){var t=Ht(a,{x:Math.cos(n.angle),y:Math.sin(n.angle)},0),r=t.x,i=t.y;return 1/Math.sqrt(Math.pow(r,2)+Math.pow(i,2))/e.width}function Ua(a,n,e,t,r){var i=Math.floor(n/r*t),o=Math.floor(a.drawingBufferHeight/r-e/r*t);return[i,o]}var vn={black:"#000000",silver:"#C0C0C0",gray:"#808080",grey:"#808080",white:"#FFFFFF",maroon:"#800000",red:"#FF0000",purple:"#800080",fuchsia:"#FF00FF",green:"#008000",lime:"#00FF00",olive:"#808000",yellow:"#FFFF00",navy:"#000080",blue:"#0000FF",teal:"#008080",aqua:"#00FFFF",darkblue:"#00008B",mediumblue:"#0000CD",darkgreen:"#006400",darkcyan:"#008B8B",deepskyblue:"#00BFFF",darkturquoise:"#00CED1",mediumspringgreen:"#00FA9A",springgreen:"#00FF7F",cyan:"#00FFFF",midnightblue:"#191970",dodgerblue:"#1E90FF",lightseagreen:"#20B2AA",forestgreen:"#228B22",seagreen:"#2E8B57",darkslategray:"#2F4F4F",darkslategrey:"#2F4F4F",limegreen:"#32CD32",mediumseagreen:"#3CB371",turquoise:"#40E0D0",royalblue:"#4169E1",steelblue:"#4682B4",darkslateblue:"#483D8B",mediumturquoise:"#48D1CC",indigo:"#4B0082",darkolivegreen:"#556B2F",cadetblue:"#5F9EA0",cornflowerblue:"#6495ED",rebeccapurple:"#663399",mediumaquamarine:"#66CDAA",dimgray:"#696969",dimgrey:"#696969",slateblue:"#6A5ACD",olivedrab:"#6B8E23",slategray:"#708090",slategrey:"#708090",lightslategray:"#778899",lightslategrey:"#778899",mediumslateblue:"#7B68EE",lawngreen:"#7CFC00",chartreuse:"#7FFF00",aquamarine:"#7FFFD4",skyblue:"#87CEEB",lightskyblue:"#87CEFA",blueviolet:"#8A2BE2",darkred:"#8B0000",darkmagenta:"#8B008B",saddlebrown:"#8B4513",darkseagreen:"#8FBC8F",lightgreen:"#90EE90",mediumpurple:"#9370DB",darkviolet:"#9400D3",palegreen:"#98FB98",darkorchid:"#9932CC",yellowgreen:"#9ACD32",sienna:"#A0522D",brown:"#A52A2A",darkgray:"#A9A9A9",darkgrey:"#A9A9A9",lightblue:"#ADD8E6",greenyellow:"#ADFF2F",paleturquoise:"#AFEEEE",lightsteelblue:"#B0C4DE",powderblue:"#B0E0E6",firebrick:"#B22222",darkgoldenrod:"#B8860B",mediumorchid:"#BA55D3",rosybrown:"#BC8F8F",darkkhaki:"#BDB76B",mediumvioletred:"#C71585",indianred:"#CD5C5C",peru:"#CD853F",chocolate:"#D2691E",tan:"#D2B48C",lightgray:"#D3D3D3",lightgrey:"#D3D3D3",thistle:"#D8BFD8",orchid:"#DA70D6",goldenrod:"#DAA520",palevioletred:"#DB7093",crimson:"#DC143C",gainsboro:"#DCDCDC",plum:"#DDA0DD",burlywood:"#DEB887",lightcyan:"#E0FFFF",lavender:"#E6E6FA",darksalmon:"#E9967A",violet:"#EE82EE",palegoldenrod:"#EEE8AA",lightcoral:"#F08080",khaki:"#F0E68C",aliceblue:"#F0F8FF",honeydew:"#F0FFF0",azure:"#F0FFFF",sandybrown:"#F4A460",wheat:"#F5DEB3",beige:"#F5F5DC",whitesmoke:"#F5F5F5",mintcream:"#F5FFFA",ghostwhite:"#F8F8FF",salmon:"#FA8072",antiquewhite:"#FAEBD7",linen:"#FAF0E6",lightgoldenrodyellow:"#FAFAD2",oldlace:"#FDF5E6",magenta:"#FF00FF",deeppink:"#FF1493",orangered:"#FF4500",tomato:"#FF6347",hotpink:"#FF69B4",coral:"#FF7F50",darkorange:"#FF8C00",lightsalmon:"#FFA07A",orange:"#FFA500",lightpink:"#FFB6C1",pink:"#FFC0CB",gold:"#FFD700",peachpuff:"#FFDAB9",navajowhite:"#FFDEAD",moccasin:"#FFE4B5",bisque:"#FFE4C4",mistyrose:"#FFE4E1",blanchedalmond:"#FFEBCD",papayawhip:"#FFEFD5",lavenderblush:"#FFF0F5",seashell:"#FFF5EE",cornsilk:"#FFF8DC",lemonchiffon:"#FFFACD",floralwhite:"#FFFAF0",snow:"#FFFAFA",lightyellow:"#FFFFE0",ivory:"#FFFFF0"},yi=new Int8Array(4),Kn=new Int32Array(yi.buffer,0,1),_i=new Float32Array(yi.buffer,0,1),Hl=/^\s*rgba?\s*\(/,$l=/^\s*rgba?\s*\(\s*([0-9]*)\s*,\s*([0-9]*)\s*,\s*([0-9]*)(?:\s*,\s*(.*)?)?\)\s*$/;function Qn(a){var n=0,e=0,t=0,r=1,i=a.toLowerCase();if(i==="transparent")return{r:0,g:0,b:0,a:0};if(i in vn)return Qn(vn[i]);if(a[0]==="#")a.length===4?(n=parseInt(a.charAt(1)+a.charAt(1),16),e=parseInt(a.charAt(2)+a.charAt(2),16),t=parseInt(a.charAt(3)+a.charAt(3),16)):(n=parseInt(a.charAt(1)+a.charAt(2),16),e=parseInt(a.charAt(3)+a.charAt(4),16),t=parseInt(a.charAt(5)+a.charAt(6),16)),a.length===9&&(r=parseInt(a.charAt(7)+a.charAt(8),16)/255);else if(Hl.test(a)){var o=a.match($l);o&&(n=+o[1],e=+o[2],t=+o[3],o[4]&&(r=+o[4]))}return{r:n,g:e,b:t,a:r}}function xi(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,e=Qn(a),t=e.r,r=e.g,i=e.b,o=e.a;return n?[t/255*o,r/255*o,i/255*o,o]:[t/255,r/255,i/255,o]}var Ot={};for(gn in vn)Ot[gn]=$e(vn[gn]),Ot[vn[gn]]=Ot[gn];var gn;function Vl(a,n,e,t,r){return Kn[0]=t<<24|e<<16|n<<8|a,r&&(Kn[0]=Kn[0]&4278190079),_i[0]}function $e(a){if(a=a.toLowerCase(),typeof Ot[a]<"u")return Ot[a];var n=Qn(a),e=n.r,t=n.g,r=n.b,i=n.a;i=i*255|0;var o=Vl(e,t,r,i,!0);return Ot[a]=o,o}function gt(a,n){_i[0]=$e(a);var e=Kn[0];n&&(e=e|16777216);var t=e&255,r=e>>8&255,i=e>>16&255,o=e>>24&255;return[t,r,i,o]}function $t(a){var n=a>>>8&255,e=a>>>16&255,t=a>>>24&255,r=a&255;return(r<<24|t<<16|e<<8|n)>>>0}function Oa(a,n,e,t){return(e<<24|n<<16|a<<8|t)>>>0}function Ti(a,n,e,t,r,i){var o=Ua(a,e,t,r,i),s=le(o,2),l=s[0],u=s[1],d=new Uint8Array(4);a.bindFramebuffer(a.FRAMEBUFFER,n),a.readPixels(l,u,1,1,a.RGBA,a.UNSIGNED_BYTE,d);var h=le(d,4),c=h[0],f=h[1],g=h[2],b=h[3];return[c,f,g,b]}function Ei(a){var n=Qn(a),e=n.r,t=n.g,r=n.b,i=n.a,o=(e/255).toFixed(6),s=(t/255).toFixed(6),l=(r/255).toFixed(6),u=i.toFixed(6);return"vec4(".concat(o,", ").concat(s,", ").concat(l,", ").concat(u,")")}function jl(a,n){if(typeof a!="object"||!a)return a;var e=a[Symbol.toPrimitive];if(e!==void 0){var t=e.call(a,n||"default");if(typeof t!="object")return t;throw new TypeError("@@toPrimitive must return a primitive value.")}return(n==="string"?String:Number)(a)}function Di(a){var n=jl(a,"string");return typeof n=="symbol"?n:n+""}function w(a,n,e){return(n=Di(n))in a?Object.defineProperty(a,n,{value:e,enumerable:!0,configurable:!0,writable:!0}):a[n]=e,a}function Si(a,n){var e=Object.keys(a);if(Object.getOwnPropertySymbols){var t=Object.getOwnPropertySymbols(a);n&&(t=t.filter(function(r){return Object.getOwnPropertyDescriptor(a,r).enumerable})),e.push.apply(e,t)}return e}function O(a){for(var n=1;n<arguments.length;n++){var e=arguments[n]!=null?arguments[n]:{};n%2?Si(Object(e),!0).forEach(function(t){w(a,t,e[t])}):Object.getOwnPropertyDescriptors?Object.defineProperties(a,Object.getOwnPropertyDescriptors(e)):Si(Object(e)).forEach(function(t){Object.defineProperty(a,t,Object.getOwnPropertyDescriptor(e,t))})}return a}function ne(a,n){if(!(a instanceof n))throw new TypeError("Cannot call a class as a function")}function Ai(a,n){for(var e=0;e<n.length;e++){var t=n[e];t.enumerable=t.enumerable||!1,t.configurable=!0,"value"in t&&(t.writable=!0),Object.defineProperty(a,Di(t.key),t)}}function ae(a,n,e){return n&&Ai(a.prototype,n),e&&Ai(a,e),Object.defineProperty(a,"prototype",{writable:!1}),a}function Et(a){return Et=Object.setPrototypeOf?Object.getPrototypeOf.bind():function(n){return n.__proto__||Object.getPrototypeOf(n)},Et(a)}function wi(){try{var a=!Boolean.prototype.valueOf.call(Reflect.construct(Boolean,[],function(){}))}catch{}return(wi=function(){return!!a})()}function Xl(a){if(a===void 0)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return a}function ql(a,n){if(n&&(typeof n=="object"||typeof n=="function"))return n;if(n!==void 0)throw new TypeError("Derived constructors may only return object or undefined");return Xl(a)}function ye(a,n,e){return n=Et(n),ql(a,wi()?Reflect.construct(n,e||[],Et(a).constructor):n.apply(a,e))}function Ha(a,n){return Ha=Object.setPrototypeOf?Object.setPrototypeOf.bind():function(e,t){return e.__proto__=t,e},Ha(a,n)}function _e(a,n){if(typeof n!="function"&&n!==null)throw new TypeError("Super expression must either be null or a function");a.prototype=Object.create(n&&n.prototype,{constructor:{value:a,writable:!0,configurable:!0}}),Object.defineProperty(a,"prototype",{writable:!1}),n&&Ha(a,n)}function Kl(a){if(Array.isArray(a))return Yn(a)}function Yl(a){if(typeof Symbol<"u"&&a[Symbol.iterator]!=null||a["@@iterator"]!=null)return Array.from(a)}function Zl(){throw new TypeError(`Invalid attempt to spread non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function Z(a){return Kl(a)||Yl(a)||Zn(a)||Zl()}function oe(a){"@babel/helpers - typeof";return oe=typeof Symbol=="function"&&typeof Symbol.iterator=="symbol"?function(n){return typeof n}:function(n){return n&&typeof Symbol=="function"&&n.constructor===Symbol&&n!==Symbol.prototype?"symbol":typeof n},oe(a)}function it(a){return oe(a)==="object"&&a!==null&&"attribute"in a}function Ri(){var a=`
float sdf_circle(vec2 uv, float size) {
  return length(uv) - size;
}
`;return{name:"circle",glsl:a,uniforms:[]}}function Ci(a){var n,e=WebGL2RenderingContext,t=e.UNSIGNED_BYTE,r=(n=a?.color)!==null&&n!==void 0?n:{attribute:"color"};if(!it(r)){var i=r,o=`
vec4 layer_fill() {
  return `.concat(Ei(i),`;
}
`);return{name:"fill",uniforms:[],attributes:[],glsl:o}}var s=r.attribute,l=`
vec4 layer_fill(vec4 v_fillColor) {
  return v_fillColor;
}
`;return{name:"fill",uniforms:[],attributes:[{name:"fillColor",size:4,type:t,normalized:!0,source:s}],glsl:l}}function Ql(a,n){if(typeof a=="string"){var e=xi(a).map(function(u){return u.toFixed(6)}),t=le(e,4),r=t[0],i=t[1],o=t[2],s=t[3];return{glsl:"vec4(".concat(r,", ").concat(i,", ").concat(o,", ").concat(s,")"),attributes:[],needsNodeColors:!1}}if("node"in a&&a.node)return{glsl:a.node==="source"?"v_sourceColor":"v_targetColor",attributes:[],needsNodeColors:!0};var l=a;return{glsl:"v_".concat(n),attributes:[{name:"a_".concat(n),size:4,type:WebGL2RenderingContext.UNSIGNED_BYTE,normalized:!0,source:l.attribute,defaultValue:l.default}],needsNodeColors:!1}}function St(a){var n=a?.color;if(n===void 0){var e=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return v_color;
}
`;return{name:"plain",glsl:e,uniforms:[],attributes:[]}}var t=Ql(n,"plainColor"),r=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return `.concat(t.glsl,`;
}
`);return{name:"plain",glsl:r,uniforms:[],attributes:t.attributes,needsNodeColors:t.needsNodeColors}}function Li(){var a=`
// Position at parameter t \u2208 [0, 1]
vec2 path_straight_position(float t, vec2 source, vec2 target) {
  return mix(source, target, t);
}

// Total length of the path (analytical - more efficient than sampling)
float path_straight_length(vec2 source, vec2 target) {
  return length(target - source);
}
`;return{name:"straight",segments:1,minBodyLengthRatio:0,linearParameterization:!0,glsl:a,uniforms:[],attributes:[]}}function ki(a){var n=a??{},e=n.segments,t=e===void 0?32:e,r=`
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
`;return{name:"loop",segments:t,glsl:r,needsNodeSize:!0,uniforms:[],attributes:[{name:"loopRadius",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopAngle",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopSpread",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopFixedOrientation",size:1,type:WebGL2RenderingContext.FLOAT}],variables:{loopRadius:{type:"number",default:4},loopAngle:{type:"number",default:Math.PI/4},loopSpread:{type:"number",default:80*Math.PI/180},loopFixedOrientation:{type:"number",default:0}},spread:{variable:"loopRadius",compute:function(o){return o+4}}}}var Ni=lt(sn());function Pi(a){return oe(a)==="object"&&"glsl"in a&&!("attributes"in a)}function Fi(a){return oe(a)==="object"&&"glsl"in a&&!("attributes"in a)}var pn={shapes:[Ri()],variables:{},layers:[Ci()],label:{},backdrop:{},labelAttachments:{}},Vt={paths:[Li(),ki()],extremities:[],variables:{},layers:[St()],defaultHead:"none",defaultTail:"none",label:{}},Jn=["nodes","topNodes"],ea=["edges","topEdges"],ta=[].concat(ea,Jn),Ii={nodes:pn,edges:Vt,depthLayers:Z(ta)};var Jl=function(n){return n},eu=function(n){return n*n},tu=function(n){return n*(2-n)},nu=function(n){return(n*=2)<1?.5*n*n:-.5*(--n*(n-2)-1)},au=function(n){return n*n*n},ru=function(n){return--n*n*n+1},iu=function(n){return(n*=2)<1?.5*n*n*n:.5*((n-=2)*n*n+2)},ou=function(n){return n===0?0:Math.pow(2,10*(n-1))},su=function(n){return n===1?1:1-Math.pow(2,-10*n)},lu=function(n){return n===0?0:n===1?1:n<.5?Math.pow(2,10*(2*n-1))/2:(2-Math.pow(2,-10*(2*n-1)))/2},$a={linear:Jl,quadraticIn:eu,quadraticOut:tu,quadraticInOut:nu,cubicIn:au,cubicOut:ru,cubicInOut:iu,exponentialIn:ou,exponentialOut:su,exponentialInOut:lu};function na(a){return a?typeof a=="function"?a:$a[a]:$a.linear}function ja(a){return oe(a)==="object"&&a!==null&&"attribute"in a}function Mi(a){return typeof a=="function"}function Wi(a){return oe(a)==="object"&&a!==null&&"when"in a&&typeof a.when=="function"&&"then"in a}function Bi(a){return oe(a)==="object"&&a!==null&&"whenState"in a&&"then"in a}function Ui(a){return oe(a)==="object"&&a!==null&&"whenData"in a&&"then"in a}var uu={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,isDragged:!1},du={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,parallelIndex:0,parallelCount:1},hu={isIdle:!0,isPanning:!1,isZooming:!1,isDragging:!1,hasHovered:!1,hasHighlighted:!1};function Oi(a){return O(O({},uu),a)}function Hi(a){return O(O({},du),a)}function Xa(a){return O(O({},hu),a)}var cu={x:{attribute:"x"},y:{attribute:"y"},size:{whenState:"isHovered",then:{attribute:"size",defaultValue:12},else:{attribute:"size",defaultValue:10}},color:{attribute:"color",defaultValue:"#666"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"},labelVisibility:{whenState:"isHovered",then:"visible",else:"auto"},backdropVisibility:{whenState:"isHovered",then:"visible",else:"hidden"},backdropColor:"#ffffff",backdropShadowColor:"rgba(0, 0, 0, 0.5)",backdropShadowBlur:12,backdropPadding:6},fu={size:{attribute:"size",defaultValue:1},color:{attribute:"color",defaultValue:"#ccc"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"}};var ra={nodes:O(O({},cu),{},{depth:{whenState:"isHovered",then:"topNodes",else:"nodes"}}),edges:O(O({},fu),{},{depth:{whenState:["isHighlighted","isHovered"],then:"topEdges",else:"edges"}})};function gu(a){return"min"in a||"max"in a||"minValue"in a||"maxValue"in a||"easing"in a}function vu(a){return"dict"in a}function ia(a,n){return typeof a=="string"?n[a]===!0:Array.isArray(a)?a.every(function(e){return n[e]===!0}):oe(a)==="object"&&a!==null?Object.entries(a).every(function(e){var t=le(e,2),r=t[0],i=t[1];return n[r]===i}):!1}function pu(a,n){var e=n[a.attribute];return e===void 0?a.defaultValue:e}function mu(a,n,e){var t,r,i,o,s=n[a.attribute];if(s===void 0)return a.defaultValue;var l=Number(s);if(isNaN(l))return a.defaultValue;if(a.min===void 0&&a.max===void 0)return l;var u=(t=a.minValue)!==null&&t!==void 0?t:l,d=(r=a.maxValue)!==null&&r!==void 0?r:l;if(d===u){var h;return(h=a.min)!==null&&h!==void 0?h:l}var c=(l-u)/(d-u);c=Math.max(0,Math.min(1,c));var f=na(a.easing);c=f(c);var g=(i=a.min)!==null&&i!==void 0?i:0,b=(o=a.max)!==null&&o!==void 0?o:1;return g+c*(b-g)}function bu(a,n){var e=n[a.attribute];if(e===void 0)return a.defaultValue;var t=String(e);return t in a.dict?a.dict[t]:a.defaultValue}function yu(a,n,e){return vu(a)?bu(a,n):gu(a)?mu(a,n):pu(a,n)}function $i(a,n){return typeof a=="string"?!!n[a]:Array.isArray(a)?a.every(function(e){return!!n[e]}):oe(a)==="object"&&a!==null?Object.entries(a).every(function(e){var t=le(e,2),r=t[0],i=t[1];return n[r]===i}):!1}function aa(a,n,e,t,r,i){if(a==null)return i;if(oe(a)!=="object"&&typeof a!="function")return a;if(Wi(a)){var o=a.when(n,e,t,r)?a.then:a.else;return o===void 0?i:aa(o,n,e,t,r,i)}if(Bi(a)){var s=ia(a.whenState,e)?a.then:a.else;return s===void 0?i:aa(s,n,e,t,r,i)}if(Ui(a)){var l=$i(a.whenData,n)?a.then:a.else;return l===void 0?i:aa(l,n,e,t,r,i)}if(Mi(a)){var u=a(n,e,t,r);return u??i}if(ja(a)){var d=yu(a,n);return d??i}return a}function Ke(a){if(a==null)return"static";if(Wi(a))return"graph-state";if(Bi(a)){var n=Ke(a.then),e=a.else!==void 0?Ke(a.else):"static";return Ve("item-state",Ve(n,e))}if(Ui(a)){var t=Ke(a.then),r=a.else!==void 0?Ke(a.else):"static";return Ve(t,r)}return Mi(a)?"graph-state":(ja(a),"static")}function Ve(a,n){return a==="graph-state"||n==="graph-state"?"graph-state":a==="item-state"||n==="item-state"?"item-state":"static"}function qa(a){if(!a)return{dependency:"static",xAttribute:null,yAttribute:null};var n=Array.isArray(a)?a:[a],e="static",t=null,r=null,i=U(n),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if("matchData"in s&&"cases"in s)for(var l=s.cases,u=0,d=Object.values(l);u<d.length;u++)for(var h=d[u],c=0,f=Object.values(h);c<f.length;c++){var g=f[c];e=Ve(e,Ke(g))}else if("matchState"in s&&"cases"in s){e=Ve(e,"item-state");for(var b=s.cases,m=0,v=Object.values(b);m<v.length;m++)for(var p=v[m],y=0,T=Object.values(p);y<T.length;y++){var _=T[y];e=Ve(e,Ke(_))}}else if("when"in s&&"then"in s)e="graph-state";else if("whenState"in s){e=Ve(e,"item-state");var S=s.then;if(S&&oe(S)==="object")for(var D=0,A=Object.values(S);D<A.length;D++){var E=A[D];e=Ve(e,Ke(E))}var L=s.else;if(L&&oe(L)==="object")for(var z=0,C=Object.values(L);z<C.length;z++){var N=C[z];e=Ve(e,Ke(N))}}else if("whenData"in s){var k=s.then;if(k&&oe(k)==="object")for(var F=0,M=Object.values(k);F<M.length;F++){var I=M[F];e=Ve(e,Ke(I))}var x=s.else;if(x&&oe(x)==="object")for(var P=0,W=Object.values(x);P<W.length;P++){var G=W[P];e=Ve(e,Ke(G))}}else for(var R=0,j=Object.entries(s);R<j.length;R++){var H=le(j[R],2),Y=H[0],Te=H[1];if(!Ya.has(Y)&&(e=Ve(e,Ke(Te)),(Y==="x"||Y==="y")&&ja(Te))){var de=Te.attribute;Y==="x"&&!t&&(t=de),Y==="y"&&!r&&(r=de)}}}}catch(q){i.e(q)}finally{i.f()}return{dependency:e,xAttribute:t,yAttribute:r}}var ot={size:10,color:"#666",opacity:1,shape:"circle",rotationAlignment:"viewport",labelRotationAlignment:"viewport",visibility:"visible",depth:"nodes",zIndex:0,label:"",labelColor:"#000",labelSize:12,labelFont:"sans-serif",labelVisibility:"auto",labelPosition:"right",labelAngle:0,labelDepth:"nodes",backdropVisibility:"hidden",backdropColor:"transparent",backdropShadowColor:"transparent",backdropShadowBlur:0,backdropPadding:0,backdropBorderColor:"transparent",backdropBorderWidth:0,backdropCornerRadius:0,backdropLabelPadding:-1,backdropArea:"both",labelAttachment:null,labelAttachmentPlacement:"below"},Ka={size:1,color:"#ccc",opacity:1,path:"straight",selfLoopPath:"loop",parallelSpread:.25,tail:"none",head:"none",visibility:"visible",depth:"edges",zIndex:0,label:"",labelColor:"#666",labelVisibility:"auto",labelDepth:"edges"},Ya=new Set(["when","whenState","whenData","then","else"]),Gi=Object.keys(ot),_u=Object.values(ot),zi=Object.keys(Ka),xu=Object.values(Ka);function jt(a,n,e,t,r,i,o){for(var s in n){var l;if(!Ya.has(s)){var u=(l=a[s])!==null&&l!==void 0?l:o[s];a[s]=aa(n[s],e,t,r,i,u)}}"depth"in n&&!("labelDepth"in n)&&(a.labelDepth=a.depth)}function Vi(a,n,e,t,r,i,o){var s=U(n),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if("matchData"in u&&"cases"in u){var d,h=String((d=e[u.matchData])!==null&&d!==void 0?d:""),c=u.cases;h in c&&jt(a,c[h],e,t,r,i,o)}else if("matchState"in u&&"cases"in u){var f,g=String((f=t[u.matchState])!==null&&f!==void 0?f:""),b=u.cases;g in b&&jt(a,b[g],e,t,r,i,o)}else if("when"in u&&"then"in u){var m=u.when;if(!m(e,t,r,i))continue;jt(a,u.then,e,t,r,i,o)}else if("whenState"in u&&"then"in u){if(!ia(u.whenState,t))continue;jt(a,u.then,e,t,r,i,o)}else if("whenData"in u&&"then"in u){if(!$i(u.whenData,e))continue;jt(a,u.then,e,t,r,i,o)}else jt(a,u,e,t,r,i,o)}}catch(v){s.e(v)}finally{s.f()}}function Za(a,n,e,t,r,i){for(var o=i||{},s=o,l=0,u=Gi.length;l<u;l++)s[Gi[l]]=_u[l];if(s.x=void 0,s.y=void 0,s.labelBackgroundColor=void 0,s.labelBackgroundPadding=void 0,s.labelCursor=void 0,!a){var d,h,c,f,g;return s.x=(d=n.x)!==null&&d!==void 0?d:0,s.y=(h=n.y)!==null&&h!==void 0?h:0,o.size=(c=n.size)!==null&&c!==void 0?c:10,o.color=(f=n.color)!==null&&f!==void 0?f:"#666",o.label=(g=n.label)!==null&&g!==void 0?g:"",o}var b=Array.isArray(a)?a:[a];return Vi(s,b,n,e,t,r,ot),o}function Qa(a,n,e,t,r,i){for(var o=i||{},s=o,l=0,u=zi.length;l<u;l++)s[zi[l]]=xu[l];if(!a){var d,h,c;return o.size=(d=n.size)!==null&&d!==void 0?d:1,o.color=(h=n.color)!==null&&h!==void 0?h:"#ccc",o.label=(c=n.label)!==null&&c!==void 0?c:"",o}var f=Array.isArray(a)?a:[a];return Vi(s,f,n,e,t,r,Ka),typeof s.labelPosition=="number"&&(s.labelPosition=void 0),o}function Tu(a,n,e){var t,r;if(a==null)return e;if(typeof a=="function")return(t=a(n))!==null&&t!==void 0?t:e;if(oe(a)==="object"&&"when"in a){var i,o=a,s=o.when(n),l=s?o.then:o.else;return l===void 0?e:typeof l=="function"?(i=l(n))!==null&&i!==void 0?i:e:l??e}if(oe(a)==="object"&&"whenState"in a){var u,d=a,h=ia(d.whenState,n),c=h?d.then:d.else;return c===void 0?e:typeof c=="function"?(u=c(n))!==null&&u!==void 0?u:e:c??e}return(r=a)!==null&&r!==void 0?r:e}function Ja(a,n){var e={};if(!a)return e;var t=Array.isArray(a)?a:[a],r=U(t),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;if("when"in o&&"then"in o){var s=o.when(n),l=s?o.then:o.else;l&&oe(l)==="object"&&Va(e,l,n)}else if("whenState"in o&&"then"in o){var u=ia(o.whenState,n),d=u?o.then:o.else;d&&oe(d)==="object"&&Va(e,d,n)}else Va(e,o,n)}}catch(h){r.e(h)}finally{r.f()}return e}function Va(a,n,e){for(var t=0,r=Object.entries(n);t<r.length;t++){var i=le(r[t],2),o=i[0],s=i[1];Ya.has(o)||(a[o]=Tu(s,e))}}var oa=(function(a){function n(){var e;return ne(this,n),e=ye(this,n),e.rawEmitter=e,e}return _e(n,a),ae(n)})(Ni.EventEmitter);var ji=Ii;function er(a,n){var e=n.size;if(e!==0){var t=a.length;a.length+=e;var r=0;n.forEach(function(i){a[t+r]=i,r++})}}function Xi(a){a=a||{};for(var n=0,e=arguments.length<=1?0:arguments.length-1;n<e;n++){var t=n+1<1||arguments.length<=n+1?void 0:arguments[n+1];t&&Object.assign(a,t)}return a}function Xt(a,n){for(var e in n)if(Object.prototype.hasOwnProperty.call(n,e)&&n[e]!==a[e])return!0;return!1}function qi(a,n){var e=a,t=n;for(var r in e)if(Object.prototype.hasOwnProperty.call(e,r)&&e[r]!==t[r])return!1;for(var i in t)if(Object.prototype.hasOwnProperty.call(t,i)&&t[i]!==e[i])return!1;return!0}function At(a,n,e){e?a.add(n):a.delete(n)}var Ki=lt(hn());var Yi={easing:"quadraticInOut",duration:150};function vt(a){return a.labelVisibility==="visible"&&a.visibility!=="hidden"}function tr(a){return a.backdropVisibility==="visible"&&a.visibility!=="hidden"}function sa(a){return[a.rotationAlignment==="graph"?1:0,a.labelRotationAlignment==="graph"?1:0]}function nr(a,n,e){var t=a[n];if(t)for(var r=0;r<t.length;r++){var i=t[r];if(!(e<i.offset||e>=i.offset+i.count)){if(i.count===1)t.splice(r,1);else if(e===i.offset)i.offset++,i.count--;else if(e===i.offset+i.count-1)i.count--;else{var o=e+1,s=i.offset+i.count-o;i.count=e-i.offset,t.splice(r+1,0,{offset:o,count:s})}return}}}function ar(a,n,e){if(!a[n]){a[n]=[{offset:e,count:1}];return}for(var t=a[n],r=t.length,i=0;i<t.length;i++)if(e<t[i].offset){r=i;break}var o=r>0?t[r-1]:null,s=r<t.length?t[r]:null,l=o&&o.offset+o.count===e,u=s&&e+1===s.offset;l&&u?(o.count+=1+s.count,t.splice(r,1)):l?o.count++:u?(s.offset--,s.count++):t.splice(r,0,{offset:e,count:1})}function Zi(a){if(!(0,Ki.default)(a))throw new Error("Sigma: invalid graph instance.")}function la(a){var n=le(a.x,2),e=n[0],t=n[1],r=le(a.y,2),i=r[0],o=r[1],s=Math.max(t-e,o-i),l=(t+e)/2,u=(o+i)/2;(s===0||Math.abs(s)===1/0||isNaN(s))&&(s=1),isNaN(l)&&(l=0),isNaN(u)&&(u=0);var d=function(c){return{x:.5+(c.x-l)/s,y:.5+(c.y-u)/s}};return d.applyTo=function(h){h.x=.5+(h.x-l)/s,h.y=.5+(h.y-u)/s},d.inverse=function(h){return{x:l+s*(h.x-.5),y:u+s*(h.y-.5)}},d.ratio=s,d}var ua={hideEdgesOnMove:!1,hideLabelsOnMove:!1,renderLabels:!0,renderEdgeLabels:!1,edgeLabelAnchors:"nodeLabels",enableEdgeEvents:!1,nodeLabelEvents:!1,edgeLabelEvents:!1,pickingDownSizingRatio:2,nodePickingPadding:0,edgePickingPadding:4,labelPickingPadding:10,stagePadding:30,minEdgeThickness:1.7,antiAliasingFeather:1,antialiasEdges:!0,antialiasNodes:!0,dragTimeout:100,draggedEventsTolerance:3,inertiaDuration:200,inertiaRatio:3,zoomDuration:250,zoomingRatio:1.7,doubleClickTimeout:300,doubleClickZoomingRatio:2.2,doubleClickZoomingDuration:200,tapMoveTolerance:10,zoomToSizeRatioFunction:Math.sqrt,itemSizesReference:"positions",autoRescale:!0,autoRescaleContent:"positions",enableNodeDrag:!1,getDraggedNodes:function(n){return[n]},dragPositionToAttributes:null,labelDensity:1,labelGridCellSize:100,labelRenderedSizeThreshold:6,labelPixelSnapping:!0,minCameraRatio:null,maxCameraRatio:null,enableCameraZooming:!0,enableCameraPanning:!0,enableCameraRotation:!0,enableCameraMouseRotation:!0,cameraPanBoundaries:null,gestureTarget:"graph",sharedGestureWheelMessage:"Use Ctrl + scroll to zoom the graph",sharedGestureAppleWheelMessage:"Use \u2318 + scroll to zoom the graph",sharedGestureTouchMessage:"Use two fingers to move the graph",allowInvalidContainer:!1,DEBUG_displayPickingLayer:!1,DEBUG_logShaders:!1,DEBUG_logRenderStats:!1,DEBUG_gpuTimerQueries:!1};function da(a){if(typeof a.labelDensity!="number"||a.labelDensity<0)throw new Error("Settings: invalid `labelDensity`. Expecting a positive number.");var n=a.minCameraRatio,e=a.maxCameraRatio;if(typeof n=="number"&&typeof e=="number"&&e<n)throw new Error("Settings: invalid camera ratio boundaries. Expecting `maxCameraRatio` to be greater than `minCameraRatio`.")}function Qi(a){return Xi({},ua,a)}var Eu=new Set(["italic","oblique"]),Su=new Set(["bold","bolder","lighter"]);function ha(a){for(var n="normal",e="normal",t=a.trim(),r=t.split(/\s+/),i=0,o=0;o<r.length;o++){var s=r[o].toLowerCase();if(Eu.has(s))e=s,i=o+1;else if(Su.has(s)||/^\d{3}$/.test(s))n=s,i=o+1;else if(s==="normal")i=o+1;else break}var l=r.slice(i).join(" ");return{family:l||t,weight:n,style:e}}function qt(a,n,e){var t=document.createElement(a);if(n)for(var r in n)t.style[r]=n[r];if(e)for(var i in e)t.setAttribute(i,e[i]);return t}function mn(){return typeof window.devicePixelRatio<"u"?window.devicePixelRatio:1}var lo=lt(sn());function Au(a){var n=a.type===WebGL2RenderingContext.FLOAT&&!a.normalized,e=a.type===WebGL2RenderingContext.UNSIGNED_BYTE&&a.size===4&&a.normalized;if(!n&&!e)throw new Error('Attribute "'.concat(a.name,'" is invalid: only non-normalized FLOAT and normalized UNSIGNED_BYTE with size 4 are supported.'));return a.normalized?1:a.size}function rr(a){var n=0;return a.forEach(function(e){return n+=Au(e)}),n}function uo(a,n,e){var t=a==="VERTEX"?n.VERTEX_SHADER:n.FRAGMENT_SHADER,r=n.createShader(t);if(r===null)throw new Error("loadShader: error while creating the shader");n.shaderSource(r,e),n.compileShader(r);var i=n.getShaderParameter(r,n.COMPILE_STATUS);if(!i){var o=n.getShaderInfoLog(r);throw n.deleteShader(r),new Error(`loadShader: error while compiling the shader:
`.concat(o,`
`).concat(e))}return r}function or(a,n){return uo("VERTEX",a,n)}function sr(a,n){return uo("FRAGMENT",a,n)}function lr(a,n){var e=a.createProgram();if(e===null)throw new Error("loadProgram: error while creating the program.");var t,r;for(t=0,r=n.length;t<r;t++)a.attachShader(e,n[t]);a.linkProgram(e);var i=a.getProgramParameter(e,a.LINK_STATUS);if(!i){var o=a.getProgramInfoLog(e);throw a.deleteProgram(e),new Error("loadProgram: error while linking the program: ".concat(o))}return e}function Ji(a){var n=a.gl,e=a.buffer,t=a.program,r=a.vertexShader,i=a.fragmentShader;n.deleteShader(r),n.deleteShader(i),n.deleteProgram(t),n.deleteBuffer(e)}function Se(a){return a%1===0?a.toFixed(1):a.toString()}var ma=new Map,ho=new Map,Du=0;function co(a,n){var e,t=(e=a.variables)===null||e===void 0||(e=e[n.source||n.name.replace(/^a_/,"")])===null||e===void 0?void 0:e.default,r=typeof t=="number"?t:typeof n.defaultValue=="number"?n.defaultValue:0;return Se(r)}function ba(a,n,e){var t,r=arguments.length>3&&arguments[3]!==void 0?arguments[3]:function(o){return"v_".concat(o.name.replace(/^a_/,""))},i=[n,e].concat(Z(a.uniforms.filter(function(o){return o.type==="float"}).map(function(o){var s;return Se((s=o.value)!==null&&s!==void 0?s:0)})),Z(((t=a.attributes)!==null&&t!==void 0?t:[]).map(r)));return"sdf_".concat(a.name,"(").concat(i.join(", "),")")}function wu(a){var n,e=a.name,t=a.uniforms.filter(function(o){return o.type==="float"&&o.value!==void 0&&o.value!==0}).map(function(o){return"".concat(o.name.replace("u_",""),"=").concat(o.value)}).sort(),r=((n=a.attributes)!==null&&n!==void 0?n:[]).map(function(o){var s;return"".concat(o.name,"@").concat((s=o.source)!==null&&s!==void 0?s:"","=").concat(co(a,o))}),i=[].concat(Z(t),Z(r));return i.length>0&&(e+="#"+i.join("#")),e}function Ru(a){var n=wu(a);if(!ma.has(n)){var e={},t=U(a.uniforms),r;try{for(t.s();!(r=t.n()).done;){var i=r.value;i.type==="float"&&i.value!==void 0&&(e[i.name]=i.value)}}catch(o){t.e(o)}finally{t.f()}ma.set(n,{shape:a,uniformValues:e,slug:n}),ho.set(n,Du++)}return n}function Tn(a){var n;return(n=ho.get(a))!==null&&n!==void 0?n:-1}function fo(a){var n=[],e=new Set,t=new Set,r=/mat2 rotate2D\(float angle\)\s*\{[^}]+\}/,i=U(a),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if(!t.has(s.name)){t.add(s.name);var l=s.glsl;r.test(l)&&(e.has("rotate2D")?l=l.replace(r,""):e.add("rotate2D")),n.push(l)}}}catch(u){i.e(u)}finally{i.f()}return n.join(`
`)}function Cu(){return fo(Array.from(ma.values()).map(function(a){return a.shape}))}function Lu(a){var n=Array.from(ma.entries());if(n.length===0)return`
float querySDF(int shapeId, vec2 uv, float size) {
  return length(uv) - size;
}
`;var e=n.map(function(t,r){var i=le(t,2),o=i[0],s=i[1].shape,l=ba(s,"uv","size",function(u){var d=u.name.replace(/^a_/,"");return a!=null&&a.has(d)?"g_".concat(d):co(s,u)});return"    case ".concat(r,": return ").concat(l,"; // ").concat(o)}).join(`
`);return`
float querySDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(e,`
    default: return length(uv) - size;
  }
}
`)}function ur(a){return fo(a)}function En(a){return Z(new Map(a.flatMap(function(n){return n.uniforms}).map(function(n){return[n.name,n]})).values())}function ku(a,n){if(a.length===0)return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = length(uv) - size;
  context.inradiusFactor = 1.0;
}
`;var e=function(h){return ba(h,"uv","size")},t=function(h){var c,f;return(c=h.inradiusFactorGLSL)!==null&&c!==void 0?c:Se((f=h.inradiusFactor)!==null&&f!==void 0?f:1)};if(a.length===1){var r=a[0];return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = `.concat(e(r),`;
  context.inradiusFactor = `).concat(t(r),`;
}
`)}var i=a.map(function(d,h){return"    case ".concat(h,": // ").concat(d.name,`
      context.sdf = `).concat(e(d),`;
      context.inradiusFactor = `).concat(t(d),`;
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
      context.sdf = `).concat(e(o),`;
      context.inradiusFactor = `).concat(t(o),`;
  }
}
`)}var Yt={right:0,left:1,above:2,below:3,over:4},Zt=5,dr=3,go=`
float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
float nodeRadiusGraphSpace = nodeSize * u_correctionRatio / u_sizeRatio * 2.0;
float nodeRadiusNDC = nodeRadiusGraphSpace * matrixScaleX;
float nodeRadiusPixels = nodeRadiusNDC * u_resolution.x / 2.0;
`,Pu=`
mat2 rotate2D(float angle) {
  float c = cos(angle);
  float s = sin(angle);
  return mat2(c, -s, s, c);
}
`,vo=`
vec2 getLabelDirection(float positionMode) {
  if (positionMode < 0.5) return vec2(1.0, 0.0);   // Right
  if (positionMode < 1.5) return vec2(-1.0, 0.0);  // Left
  if (positionMode < 2.5) return vec2(0.0, -1.0);  // Above
  if (positionMode < 3.5) return vec2(0.0, 1.0);   // Below
  return vec2(0.0);                                 // Over (centered)
}
`,Fu=`
float sdfBox(vec2 p, vec2 halfSize) {
  vec2 d = abs(p) - halfSize;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}
`,Iu=`
float sdfRotatedBox(vec2 p, vec2 halfSize, float angle) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfBox(rotatedP, halfSize);
}
`,Gu=`
float sdfRoundedBox(vec2 p, vec2 halfSize, float radius) {
  vec2 d = abs(p) - halfSize + radius;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - radius;
}
`,zu=`
float sdfRoundedRotatedBox(vec2 p, vec2 halfSize, float angle, float radius) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfRoundedBox(rotatedP, halfSize, radius);
}
`,po=`
vec2 labelBoxCenter(float positionMode, float labelStart, vec2 halfSize, float textHalfY) {
  if (positionMode < 0.5) return vec2(labelStart + halfSize.x, 0.0);    // right
  if (positionMode < 1.5) return vec2(-(labelStart + halfSize.x), 0.0); // left
  if (positionMode < 2.5) return vec2(0.0, -(labelStart + textHalfY));  // above
  if (positionMode < 3.5) return vec2(0.0, labelStart + textHalfY);     // below
  return vec2(0.0);                                                     // over
}
`,hr=2,st=`
vec4 readNodeData(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(hr,`;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),wt=`
vec4 readNodeFlags(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(hr,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),Nu=`
vec4 readNodeColor(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(hr,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  vec4 texel = texelFetch(nodeDataTexture, coord, 0);
  float r = floor(texel.b / 65536.0);
  float g = floor(mod(texel.b, 65536.0) / 256.0);
  float b = mod(texel.b, 256.0);
  return vec4(r / 255.0, g / 255.0, b / 255.0, texel.a);
}
`),Qt=`
vec4 readFrameTexel(sampler2D frameTexture, int frameTextureWidth, int index) {
  ivec2 coord = ivec2(index % frameTextureWidth, index / frameTextureWidth);
  return texelFetch(frameTexture, coord, 0);
}
`;function Mu(a,n){var e=function(o){return ba(o,"uv","size")};if(a.length===1)return{code:eo(e(a[0])),multiShape:!1};var t=a.map(function(i,o){return"    case ".concat(n?n[o]:o,": return ").concat(e(i),";")}).join(`
`),r=`
float queryShapeSDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(t,`
    default: return `).concat(e(a[0]),`;
  }
}
int g_shapeId;
`).concat(eo("queryShapeSDF(g_shapeId, uv, size)"),`
`);return{code:r,multiShape:!0}}function eo(a){return`
float findEdgeDistance(vec2 direction, float size) {
  float lo = 0.0, hi = 2.0;
  for (int i = 0; i < 8; i++) {
    float mid = (lo + hi) * 0.5;
    vec2 uv = direction * mid;
    if (`.concat(a,` < 0.0) lo = mid; else hi = mid;
  }
  return (lo + hi) * 0.5;
}
`)}function Wu(a,n){for(;!{}.hasOwnProperty.call(a,n)&&(a=Et(a))!==null;);return a}function ir(){return ir=typeof Reflect<"u"&&Reflect.get?Reflect.get.bind():function(a,n,e){var t=Wu(a,n);if(t){var r=Object.getOwnPropertyDescriptor(t,n);return r.get?r.get.call(arguments.length<3?a:e):r.value}},ir.apply(null,arguments)}function De(a,n,e,t){var r=ir(Et(1&t?a.prototype:a),n,e);return 2&t&&typeof r=="function"?function(i){return r.apply(e,i)}:r}var Bu=1024,Uu=1.5,Ou=4096,mo=-1,ya=(function(){function a(n,e){var t=arguments.length>2&&arguments[2]!==void 0?arguments[2]:Bu;ne(this,a),w(this,"texture",null),w(this,"dirty",!1),w(this,"dirtyRangeStart",1/0),w(this,"dirtyRangeEnd",-1),w(this,"indexMap",new Map),w(this,"freeIndices",[]),w(this,"nextIndex",0),this.gl=n,this.TEXELS_PER_ITEM=e,this.capacity=this.roundUpToPowerOfTwo(t);var r=this.computeTextureDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.data=new Float32Array(this.textureWidth*this.textureHeight*4),this.createTexture()}return ae(a,[{key:"computeTextureDimensions",value:function(e){var t=e*this.TEXELS_PER_ITEM,r=Math.min(t,Ou),i=Math.ceil(t/r);return{width:r,height:i}}},{key:"roundUpToPowerOfTwo",value:function(e){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,e))))}},{key:"createTexture",value:function(){var e=this.gl;this.texture=e.createTexture(),e.activeTexture(e.TEXTURE0),e.bindTexture(e.TEXTURE_2D,this.texture),e.texImage2D(e.TEXTURE_2D,0,e.RGBA32F,this.textureWidth,this.textureHeight,0,e.RGBA,e.FLOAT,this.data),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE),e.bindTexture(e.TEXTURE_2D,null)}},{key:"resize",value:function(e){if(!(e<=this.capacity)){var t=this.roundUpToPowerOfTwo(Math.ceil(e*Uu)),r=this.gl,i=this.computeTextureDimensions(t),o=new Float32Array(i.width*i.height*4);o.set(this.data),this.texture&&r.deleteTexture(this.texture),this.data=o,this.capacity=t,this.textureWidth=i.width,this.textureHeight=i.height,this.createTexture(),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.nextIndex}}},{key:"allocate",value:function(e){var t=this.indexMap.get(e);if(t!==void 0)return t;var r;return this.freeIndices.length>0?r=this.freeIndices.pop():(r=this.nextIndex++,r>=this.capacity&&this.resize(r+1)),this.indexMap.set(e,r),r}},{key:"free",value:function(e){var t=this.indexMap.get(e);if(t!==void 0){this.indexMap.delete(e),this.freeIndices.push(t);for(var r=t*this.TEXELS_PER_ITEM*4,i=0;i<this.TEXELS_PER_ITEM*4;i++)this.data[r+i]=0;this.markDirty(t)}}},{key:"getIndex",value:function(e){var t;return(t=this.indexMap.get(e))!==null&&t!==void 0?t:-1}},{key:"has",value:function(e){return this.indexMap.has(e)}},{key:"markDirty",value:function(e){this.dirty=!0,this.dirtyRangeStart=Math.min(this.dirtyRangeStart,e),this.dirtyRangeEnd=Math.max(this.dirtyRangeEnd,e+1)}},{key:"upload",value:function(){if(!(!this.dirty||!this.texture)){var e=this.gl,t=this.textureWidth;e.activeTexture(e.TEXTURE0),e.bindTexture(e.TEXTURE_2D,this.texture);var r=this.dirtyRangeStart*this.TEXELS_PER_ITEM,i=Math.min(this.dirtyRangeEnd*this.TEXELS_PER_ITEM,this.capacity*this.TEXELS_PER_ITEM);if(r<i)for(var o=Math.floor(r/t),s=Math.floor((i-1)/t),l=o;l<=s;l++){var u=l*t,d=Math.min(u+t,this.capacity*this.TEXELS_PER_ITEM),h=Math.max(r,u),c=Math.min(i,d);if(h<c){var f=h-u,g=c-h,b=this.data.subarray(h*4,c*4);e.texSubImage2D(e.TEXTURE_2D,0,f,l,g,1,e.RGBA,e.FLOAT,b)}}e.bindTexture(e.TEXTURE_2D,null),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}}},{key:"bind",value:function(e){var t=this.gl;t.activeTexture(t.TEXTURE0+e),t.bindTexture(t.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getCapacity",value:function(){return this.capacity}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"getTexelsPerItem",value:function(){return this.TEXELS_PER_ITEM}},{key:"getCount",value:function(){return this.indexMap.size}},{key:"getHighWaterMark",value:function(){return this.nextIndex}},{key:"isDirty",value:function(){return this.dirty}},{key:"clear",value:function(){this.indexMap.clear(),this.freeIndices=[],this.nextIndex=0,this.data.fill(0),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.capacity}},{key:"restore",value:function(){this.createTexture(),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}},{key:"kill",value:function(){this.texture&&(this.gl.deleteTexture(this.texture),this.texture=null),this.indexMap.clear(),this.freeIndices=[]}}])})();function je(a){var n={},e={},t=0,r=U(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=U(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");d in n||(n[d]=t,e[d]=u,t+=u.size)}}catch(h){s.e(h)}finally{s.f()}}}catch(h){r.e(h)}finally{r.f()}return{floatsPerItem:t,texelsPerItem:Math.max(1,Math.ceil(t/4)),offsets:n,specs:e}}var _a=(function(a){function n(e,t,r){var i;return ne(this,n),i=ye(this,n,[e,t.texelsPerItem,r]),i.floatsPerItem=t.floatsPerItem,i}return _e(n,a),ae(n,[{key:"updateAllAttributes",value:function(t,r){var i=this.indexMap.get(t);i===void 0&&(i=this.allocate(t));for(var o=i*this.TEXELS_PER_ITEM*4,s=Math.min(r.length,this.floatsPerItem),l=0;l<s;l++)this.data[o+l]=r[l];this.markDirty(i)}},{key:"updateAllAttributesAtRow",value:function(t,r){t>=this.capacity&&this.resize(t+1);for(var i=t*this.TEXELS_PER_ITEM*4,o=Math.min(r.length,this.floatsPerItem),s=0;s<o;s++)this.data[i+s]=r[s];this.markDirty(t)}}])})(ya);function xa(a,n,e){for(var t=[],r=new Set,i=0;i<a.length;i++){var o=a[i],s=U(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");if(!r.has(d)){r.add(d);var h=n.offsets[d];if(h!==void 0){var c=e?.get(i);t.push({sourceKey:u.source||d,packedOffset:h,size:u.size,isColor:u.size===4&&!!u.normalized,defaultNum:typeof u.defaultValue=="number"?u.defaultValue:u.defaultValue===!0?1:0,defaultColor:typeof u.defaultValue=="string"?u.defaultValue:"",sourceIndex:i,hasLifecycleHook:!!(c!=null&&c.getAttributeData)})}}}}catch(f){s.e(f)}finally{s.f()}}return t}function Ta(a,n,e,t,r,i){e.fill(0);for(var o=0,s=a.length;o<s;o++){var l=a[o],u=l.packedOffset,d=void 0;if(l.hasLifecycleHook){var h=r.get(l.sourceIndex-i);d=h.getAttributeData(n,l.sourceKey),d===null&&(d=n[l.sourceKey])}else d=n[l.sourceKey];if(l.isColor){var c=typeof d=="string"?d:l.defaultColor||t,f=gt(c),g=le(f,4),b=g[0],m=g[1],v=g[2],p=g[3];e[u]=b/255,e[u+1]=m/255,e[u+2]=v/255,e[u+3]=p/255}else if(l.size===1)e[u]=typeof d=="number"?d:typeof d=="boolean"?d?1:0:l.defaultNum;else{var y=Array.isArray(d)?d:null;if(y)for(var T=0;T<l.size;T++){var _;e[u+T]=(_=y[T])!==null&&_!==void 0?_:0}}}}var ca=["r","g","b","a"];function Kt(a,n){var e=a.offsets,t=a.specs,r=a.texelsPerItem,i=a.floatsPerItem,o=Object.keys(e);if(o.length===0||i===0)return{fetchCode:"",varyingAssignments:""};var s=n.varPrefix,l=n.baseTexelExpr,u=n.textureWidthUniform,d=n.textureSamplerUniform,h=n.outputPrefix,c=h===void 0?"v_":h,f=[];f.push("  int ".concat(s,"BaseTexel = ").concat(l,";")),f.push("");for(var g=0;g<r;g++)f.push("  ivec2 ".concat(s,"Coord").concat(g," = ivec2((").concat(s,"BaseTexel + ").concat(g,") % ").concat(u,", (").concat(s,"BaseTexel + ").concat(g,") / ").concat(u,");")),f.push("  vec4 ".concat(s,"Texel").concat(g," = texelFetch(").concat(d,", ").concat(s,"Coord").concat(g,", 0);"));f.push("");for(var b=[],m=function(){var T=p[v],_=t[T],S=e[T],D=Math.floor(S/4),A=S%4,E="".concat(s,"Texel").concat(D),L="".concat(s,"Texel").concat(D+1),z="".concat(s,"Fetched_").concat(T);if(_.size===1)f.push("  float ".concat(z," = ").concat(E,".").concat(ca[A],";"));else if(_.size===4&&A===0)f.push("  vec4 ".concat(z," = ").concat(E,";"));else{var C=A+_.size;if(C<=4){var N="vec".concat(_.size),k=ca.slice(A,C).join("");f.push("  ".concat(N," ").concat(z," = ").concat(E,".").concat(k,";"))}else{var F=_.size===4?"vec4":"vec".concat(_.size),M=ca.slice(A).map(function(P){return"".concat(E,".").concat(P)}),I=ca.slice(0,C-4).map(function(P){return"".concat(L,".").concat(P)}),x=[].concat(Z(M),Z(I)).join(", ");f.push("  ".concat(F," ").concat(z," = ").concat(F,"(").concat(x,");"))}}b.push("  ".concat(c).concat(T," = ").concat(z,";"))},v=0,p=o;v<p.length;v++)m();return{fetchCode:f.join(`
`),varyingAssignments:b.join(`
`)}}function cr(a,n,e){if(!(!n||e.type==="sampler2D"))switch(e.type){case"float":a.uniform1f(n,e.value);break;case"int":case"bool":a.uniform1i(n,e.value);break;case"vec2":a.uniform2fv(n,e.value);break;case"vec3":a.uniform3fv(n,e.value);break;case"vec4":a.uniform4fv(n,e.value);break;case"mat3":a.uniformMatrix3fv(n,!1,e.value);break;case"mat4":a.uniformMatrix4fv(n,!1,e.value);break}}var Hu=w(w(w(w(w(w(w(w({},WebGL2RenderingContext.BOOL,1),WebGL2RenderingContext.BYTE,1),WebGL2RenderingContext.UNSIGNED_BYTE,1),WebGL2RenderingContext.SHORT,2),WebGL2RenderingContext.UNSIGNED_SHORT,2),WebGL2RenderingContext.INT,4),WebGL2RenderingContext.UNSIGNED_INT,4),WebGL2RenderingContext.FLOAT,4);function fa(a){var n=a.match(/^(#version[^\n]*\n)/);return n?n[1]+`#define PICKING_MODE
`+a.slice(n[1].length):`#define PICKING_MODE
`+a}var Rt=(function(){function a(n,e,t){ne(this,a),w(this,"floats",new Float32Array),w(this,"ints",new Uint32Array),w(this,"constantArray",new Float32Array),w(this,"capacity",0),w(this,"verticesCount",0),w(this,"bufferGeneration",0),w(this,"uploadedGeneration",new Map),w(this,"constantBufferGeneration",0),w(this,"uploadedConstantGeneration",new Map),w(this,"renderOffset",0),w(this,"renderCount",-1),w(this,"debugStats",{drawCalls:0,verticesDrawn:0,bufferUploadBytes:0}),w(this,"shadersLogged",!1),w(this,"pickProgram",null);var r=this.getDefinition();if(this.VERTICES=r.VERTICES,this.VERTEX_SHADER_SOURCE=r.VERTEX_SHADER_SOURCE,this.FRAGMENT_SHADER_SOURCE=r.FRAGMENT_SHADER_SOURCE,this.UNIFORMS=r.UNIFORMS,this.ATTRIBUTES=r.ATTRIBUTES,this.METHOD=r.METHOD,this.CONSTANT_ATTRIBUTES="CONSTANT_ATTRIBUTES"in r?r.CONSTANT_ATTRIBUTES:[],this.CONSTANT_DATA="CONSTANT_DATA"in r?r.CONSTANT_DATA:[],this.isInstanced="CONSTANT_ATTRIBUTES"in r,this.ATTRIBUTES_ITEMS_COUNT=rr(this.ATTRIBUTES),this.STRIDE=this.VERTICES*this.ATTRIBUTES_ITEMS_COUNT,this.renderer=t,this.normalProgram=this.getProgramInfo("normal",n,r.VERTEX_SHADER_SOURCE,r.FRAGMENT_SHADER_SOURCE,null),this.pickProgram=this.getProgramInfo("pick",n,fa(r.VERTEX_SHADER_SOURCE),fa(r.FRAGMENT_SHADER_SOURCE),null),this.isInstanced){var i=rr(this.CONSTANT_ATTRIBUTES);if(this.CONSTANT_DATA.length!==this.VERTICES)throw new Error("Program: error while getting constant data (expected ".concat(this.VERTICES," items, received ").concat(this.CONSTANT_DATA.length," instead)"));this.constantArray=new Float32Array(this.CONSTANT_DATA.length*i);for(var o=0;o<this.CONSTANT_DATA.length;o++){var s=this.CONSTANT_DATA[o];if(s.length!==i)throw new Error("Program: error while getting constant data (one vector has ".concat(s.length," items instead of ").concat(i,")"));for(var l=0;l<s.length;l++)this.constantArray[o*i+l]=s[l]}this.STRIDE=this.ATTRIBUTES_ITEMS_COUNT}}return ae(a,[{key:"kill",value:function(){Ji(this.normalProgram),this.pickProgram&&Ji(this.pickProgram)}},{key:"resetDebugStats",value:function(){this.debugStats.drawCalls=0,this.debugStats.verticesDrawn=0,this.debugStats.bufferUploadBytes=0}},{key:"getProgramInfo",value:function(e,t,r,i,o){var s=t.createBuffer();if(s===null)throw new Error("Program: error while creating the WebGL buffer.");var l=or(t,r),u=sr(t,i),d=lr(t,[l,u]),h={};this.UNIFORMS.forEach(function(g){var b=t.getUniformLocation(d,g);b&&(h[g]=b)});var c={};this.ATTRIBUTES.forEach(function(g){c[g.name]=t.getAttribLocation(d,g.name)});var f;if(this.isInstanced&&(this.CONSTANT_ATTRIBUTES.forEach(function(g){c[g.name]=t.getAttribLocation(d,g.name)}),f=t.createBuffer(),f===null))throw new Error("Program: error while creating the WebGL constant buffer.");return{name:e,program:d,gl:t,frameBuffer:o,buffer:s,constantBuffer:f||{},uniformLocations:h,attributeLocations:c,isPicking:e==="pick",vertexShader:l,fragmentShader:u}}},{key:"bindProgram",value:function(e){var t=this,r=0,i=e.gl,o=e.buffer;if(this.isInstanced){if(i.bindBuffer(i.ARRAY_BUFFER,e.constantBuffer),r=0,this.CONSTANT_ATTRIBUTES.forEach(function(d){return r+=t.bindAttribute(d,e,r,!1)}),this.uploadedConstantGeneration.get(e.constantBuffer)!==this.constantBufferGeneration){var l;i.bufferData(i.ARRAY_BUFFER,this.constantArray,i.STATIC_DRAW),this.uploadedConstantGeneration.set(e.constantBuffer,this.constantBufferGeneration),(l=this.renderer)!==null&&l!==void 0&&l.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.constantArray.byteLength)}if(i.bindBuffer(i.ARRAY_BUFFER,e.buffer),r=this.renderOffset*this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT,this.ATTRIBUTES.forEach(function(d){return r+=t.bindAttribute(d,e,r,!0)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var u;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(u=this.renderer)!==null&&u!==void 0&&u.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}}else if(i.bindBuffer(i.ARRAY_BUFFER,o),r=0,this.ATTRIBUTES.forEach(function(d){return r+=t.bindAttribute(d,e,r)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var s;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(s=this.renderer)!==null&&s!==void 0&&s.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}i.bindBuffer(i.ARRAY_BUFFER,null)}},{key:"unbindProgram",value:function(e){var t=this;this.isInstanced?(this.CONSTANT_ATTRIBUTES.forEach(function(r){return t.unbindAttribute(r,e,!1)}),this.ATTRIBUTES.forEach(function(r){return t.unbindAttribute(r,e,!0)})):this.ATTRIBUTES.forEach(function(r){return t.unbindAttribute(r,e)})}},{key:"bindAttribute",value:function(e,t,r,i){var o=Hu[e.type];if(typeof o!="number")throw new Error('Program.bind: yet unsupported attribute type "'.concat(e.type,'"'));var s=t.attributeLocations[e.name],l=t.gl;if(s!==-1){l.enableVertexAttribArray(s);var u=this.isInstanced?(i?this.ATTRIBUTES_ITEMS_COUNT:rr(this.CONSTANT_ATTRIBUTES))*Float32Array.BYTES_PER_ELEMENT:this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT;l.vertexAttribPointer(s,e.size,e.type,e.normalized||!1,u,r),this.isInstanced&&i&&l.vertexAttribDivisor(s,1)}return e.size*o}},{key:"unbindAttribute",value:function(e,t,r){var i=t.attributeLocations[e.name],o=t.gl;i!==-1&&(o.disableVertexAttribArray(i),this.isInstanced&&r&&o.vertexAttribDivisor(i,0))}},{key:"reallocate",value:function(e){e!==this.capacity&&(this.capacity=e,this.verticesCount=this.VERTICES*e,this.floats=new Float32Array(this.isInstanced?this.capacity*this.ATTRIBUTES_ITEMS_COUNT:this.verticesCount*this.ATTRIBUTES_ITEMS_COUNT),this.ints=new Uint32Array(this.floats.buffer),this.invalidateBuffers())}},{key:"invalidateBuffers",value:function(){this.bufferGeneration++,this.constantBufferGeneration++}},{key:"hasNothingToRender",value:function(){return this.verticesCount===0}},{key:"setTypedUniform",value:function(e,t){var r;cr(t.gl,(r=t.uniformLocations[e.name])!==null&&r!==void 0?r:null,e)}},{key:"renderProgram",value:function(e,t){var r=t.gl,i=t.program,o=t.isPicking;o?r.disable(r.BLEND):r.enable(r.BLEND),r.useProgram(i),this.setUniforms(e,t),this.drawWebGL(this.METHOD,t)}},{key:"render",value:function(e,t,r){var i;if(!this.shadersLogged&&(i=this.renderer)!==null&&i!==void 0&&i.getSetting("DEBUG_logShaders")&&(this.shadersLogged=!0,console.log("[sigma] DEBUG_logShaders: ".concat(this.constructor.name),{normal:{vertexShaderSource:this.VERTEX_SHADER_SOURCE,fragmentShaderSource:this.FRAGMENT_SHADER_SOURCE},pick:{vertexShaderSource:fa(this.VERTEX_SHADER_SOURCE),fragmentShaderSource:fa(this.FRAGMENT_SHADER_SOURCE)}})),!this.hasNothingToRender()){this.renderOffset=t??0,this.renderCount=r??-1;var o=this.normalProgram.gl;if(o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,e.width*e.pixelRatio,e.height*e.pixelRatio),this.bindProgram(this.normalProgram),this.renderProgram(e,this.normalProgram),this.unbindProgram(this.normalProgram),this.pickProgram&&e.pickingFrameBuffer){var s=Math.ceil(e.width*e.pixelRatio/e.downSizingRatio),l=Math.ceil(e.height*e.pixelRatio/e.downSizingRatio);o.bindFramebuffer(o.FRAMEBUFFER,e.pickingFrameBuffer),o.viewport(0,0,s,l),this.bindProgram(this.pickProgram),this.renderProgram(e,this.pickProgram),this.unbindProgram(this.pickProgram),o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,e.width*e.pixelRatio,e.height*e.pixelRatio)}}}},{key:"drawWebGL",value:function(e,t){var r,i=t.gl,o=this.renderCount>=0?this.renderCount:this.capacity;(r=this.renderer)!==null&&r!==void 0&&r.getSetting("DEBUG_logRenderStats")&&(this.debugStats.drawCalls++,this.debugStats.verticesDrawn+=o*this.VERTICES),this.isInstanced?i.drawArraysInstanced(e,0,this.VERTICES,o):i.drawArrays(e,this.renderOffset*this.VERTICES,o*this.VERTICES)}}])})(),bt=2,Sn=7,bo={below:0,above:1,left:2,right:3};function $u(){var a=`#version 300 es
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

`.concat(st,`
`).concat(wt,`
`).concat(Qt,`
`).concat(po,`
`).concat(Pu,`

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
  float gap = `).concat(bt.toFixed(1),` * zoomScale * u_pixelRatio;
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
`);return a}var Vu=`#version 300 es
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
`;function ju(a,n,e,t){var r,i,o=t.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Zt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=$u(),h=Vu,c=(function(f){function g(){var b;ne(this,g);for(var m=arguments.length,v=new Array(m),p=0;p<m;p++)v[p]=arguments[p];return b=ye(this,g,[].concat(v)),w(b,"totalCount",0),w(b,"bufferCapacity",0),b}return _e(g,f),ae(g,[{key:"getDefinition",value:function(){var m=WebGL2RenderingContext,v=m.FLOAT,p=m.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:h,METHOD:p,UNIFORMS:["u_matrix","u_resolution","u_labelPixelSnapping","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_atlasTexture"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:v},{name:"a_atlasRect",size:4,type:v},{name:"a_attachmentSize",size:2,type:v},{name:"a_positionMode",size:1,type:v},{name:"a_attachmentPlacement",size:1,type:v},{name:"a_labelWidth",size:1,type:v},{name:"a_labelHeight",size:1,type:v},{name:"a_textHeight",size:1,type:v},{name:"a_labelAngle",size:1,type:v}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:v}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processAttachment",value:function(m,v){var p=this.floats,y=m*this.STRIDE;p[y++]=v.nodeIndex,p[y++]=v.atlasX,p[y++]=v.atlasY,p[y++]=v.atlasW,p[y++]=v.atlasH,p[y++]=v.attachWidth,p[y++]=v.attachHeight,p[y++]=v.positionMode,p[y++]=v.attachmentPlacement,p[y++]=v.labelWidth,p[y++]=v.labelHeight,p[y++]=v.textHeight,p[y++]=v.labelAngle}},{key:"setUniforms",value:function(m,v){var p=v.gl,y=v.uniformLocations;p.uniformMatrix3fv(y.u_matrix,!1,m.matrix),p.uniform2f(y.u_resolution,m.width*m.pixelRatio,m.height*m.pixelRatio),p.uniform1f(y.u_labelPixelSnapping,m.labelPixelSnapping),p.uniform1f(y.u_sizeRatio,m.sizeRatio),p.uniform1f(y.u_correctionRatio,m.correctionRatio),p.uniform1f(y.u_cameraAngle,m.cameraAngle),p.uniform1f(y.u_pixelRatio,m.pixelRatio),p.uniform1f(y.u_labelMargin,g.labelMargin),p.uniform1f(y.u_zoomLabelSizeRatio,1/u(m.zoomRatio)),p.uniform1i(y.u_nodeDataTexture,m.nodeDataTextureUnit),p.uniform1i(y.u_nodeDataTextureWidth,m.nodeDataTextureWidth),p.uniform1i(y.u_nodeFrameTexture,m.nodeFrameTextureUnit),p.uniform1i(y.u_nodeFrameTextureWidth,m.nodeFrameTextureWidth),p.uniform1i(y.u_atlasTexture,Sn)}},{key:"reallocateAttachments",value:function(m){this.totalCount=m,m>this.bufferCapacity&&(this.bufferCapacity=Math.max(m,Math.ceil(this.bufferCapacity*1.5)||10),De(g,"reallocate",this,3)([this.bufferCapacity]))}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(m,v){var p=v.gl;this.totalCount!==0&&p.drawArraysInstanced(m,0,this.VERTICES,this.totalCount)}}])})(Rt);return w(c,"labelMargin",l),new c(a,n,e)}var pt=5;function Ea(a,n){return[].concat(Z(n),Z(a.map(function(e){var t;return{attributes:(t=e.attributes)!==null&&t!==void 0?t:[]}})))}function Jt(a,n){var e=je(Ea(a,n)),t=new Set(a.flatMap(function(u){var d;return(d=u.attributes)!==null&&d!==void 0?d:[]}).map(function(u){return u.name.replace(/^a_/,"")})),r={},i={},o=U(t),s;try{for(o.s();!(s=o.n()).done;){var l=s.value;r[l]=e.offsets[l],i[l]=e.specs[l]}}catch(u){o.e(u)}finally{o.f()}return O(O({},e),{},{offsets:r,specs:i})}function Xu(a,n){var e=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pickingPadding","u_nodeDataTexture","u_layerAttributeTexture"]),t=new Set,r=a.flatMap(function(f){return f.uniforms}).filter(function(f){return e.has(f.name)||t.has(f.name)?!1:(t.add(f.name),!0)}).map(function(f){return"uniform ".concat(f.type," ").concat(f.name,";")}).join(`
`),i=n.flatMap(function(f){return f.uniforms}).filter(function(f){return e.has(f.name)||t.has(f.name)?!1:(t.add(f.name),!0)}).map(function(f){return"uniform ".concat(f.type," ").concat(f.name,";")}).join(`
`),o=Ea(a,n),s=new Set,l=o.flatMap(function(f){return f.attributes}).filter(function(f){var g=f.name.replace(/^a_/,"");return s.has(g)?!1:(s.add(g),!0)}).map(function(f){var g=f.name.replace(/^a_/,""),b=f.size===1?"float":"vec".concat(f.size);return"out ".concat(b," v_").concat(g,";")}).join(`
`),u=Kt(je(o),{varPrefix:"layer",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),d=u.fetchCode,h=u.varyingAssignments,c=`#version 300 es

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
`).concat(st,`
`).concat(wt,`

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
`);return c}function qu(a,n,e){var t=arguments.length>3&&arguments[3]!==void 0?arguments[3]:!0,r=n.map(function(g,b){var m="layer_".concat(g.name),v=[].concat(Z(g.attributes.map(function(p){return"v_".concat(p.name.replace(/^a_/,""))})),Z(g.uniforms.map(function(p){return p.name}))).join(", ");return"  // Layer ".concat(b+1,": ").concat(g.name,`
  color = blendOver(color, `).concat(m,"(").concat(v,"));")}).join(`

`),i=new Set(["u_correctionRatio"]),o=new Set,s=a.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),l=n.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),u=new Set,d=Ea(a,n).flatMap(function(g){return g.attributes}).filter(function(g){var b=g.name.replace(/^a_/,"");return u.has(b)?!1:(u.add(b),!0)}).map(function(g){var b=g.name.replace(/^a_/,""),m=g.size===1?"float":"vec".concat(g.size);return"in ".concat(m," v_").concat(b,";")}).join(`
`),h=ur(a),c=ku(a,e),f=`#version 300 es
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
`).concat(c,`

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
`).concat(t?`    // Visual pass: apply antialiasing at shape boundary, node opacity once
    // smoothstep provides smooth transition from opaque to transparent
    float alpha = smoothstep(context.aaWidth, -context.aaWidth, context.sdf) * v_opacity;`:`    // Visual pass: hard-edged (no anti-aliasing gradient) shape boundary, node opacity applied once
    float alpha = (context.sdf < 0.0 ? 1.0 : 0.0) * v_opacity;`,`
    // Mix with transparent to fade both color AND alpha together (avoids bright halo)
    fragColor = mix(vec4(0.0), color, alpha);
  #endif
}
`);return f}function Ku(a,n){var e=new Set;return e.add("u_matrix"),e.add("u_sizeRatio"),e.add("u_correctionRatio"),e.add("u_cameraAngle"),e.add("u_pickingPadding"),e.add("u_nodeDataTexture"),e.add("u_nodeDataTextureWidth"),e.add("u_layerAttributeTexture"),e.add("u_layerAttributeTextureWidth"),e.add("u_layerAttributeTexelsPerNode"),a.forEach(function(t){t.uniforms.forEach(function(r){return e.add(r.name)})}),n.forEach(function(t){t.uniforms.forEach(function(r){return e.add(r.name)})}),Array.from(e)}function Yu(a){var n=WebGL2RenderingContext,e=n.UNSIGNED_BYTE,t=n.FLOAT;return[{name:"a_nodeIndex",size:1,type:t},{name:"a_id",size:4,type:e,normalized:!0},{name:"a_opacity",size:1,type:t}]}function to(a){var n=a.shapes,e=a.layers,t=a.shapeGlobalIds,r=a.antialias,i=r===void 0?!0:r;return{vertexShader:Xu(n,e),fragmentShader:qu(n,e,t,i),uniforms:Ku(n,e),attributes:Yu()}}function yo(a,n,e){var t=Jt(a,n),r=t.specs;return Object.keys(r).map(function(i){return"flat ".concat(e," ").concat(r[i].size===1?"float":"vec".concat(r[i].size)," v_").concat(i,";")}).join(`
`)}function Zu(a){var n,e,t=a.shapes,r=a.layers,i=a.shapeGlobalIds,o=Kt(Jt(t,r),{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),s=t.length===1?"float inradiusFactor = ".concat(Se((n=t[0].inradiusFactor)!==null&&n!==void 0?n:1),";"):"float inradiusFactor = ".concat(Se((e=t[0].inradiusFactor)!==null&&e!==void 0?e:1),`;
  switch (int(shapeId)) {
`).concat(t.map(function(u,d){var h,c=i?i[d]:d;return"    case ".concat(c,": inradiusFactor = ").concat(Se((h=u.inradiusFactor)!==null&&h!==void 0?h:1),"; break;")}).join(`
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
`.concat(yo(t,r,"out"),`

`).concat(st,`
`).concat(wt,`
`).concat(Qt,`

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

  `).concat(go,`
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
`);return l}function Qu(a){var n=a.shapes,e=a.layers,t=a.shapeGlobalIds,r=ur(n),i=En(n).map(function(c){return"uniform ".concat(c.type," ").concat(c.name,";")}).join(`
`),o=function(f){return ba(f,"nodeUV","1.0")},s;if(n.length===1)s="float nodeSdfNormalized = ".concat(o(n[0]),";");else{var l=n.map(function(c,f){var g=t?t[f]:f;return"    case ".concat(g,": nodeSdfNormalized = ").concat(o(c),"; break;")}).join(`
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
`.concat(yo(n,e,"in"),`

uniform float u_cameraAngle;
`).concat(i,`

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec4 fragPicking;

`).concat(r,`
`).concat(Fu,`
`).concat(Iu,`
`).concat(Gu,`
`).concat(zu,`

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
`);return h}function Ju(a){var n=["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"],e=U(En(a)),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;n.includes(r.name)||n.push(r.name)}}catch(i){e.e(i)}finally{e.f()}return n}function ed(a){return{vertexShader:Zu(a),fragmentShader:Qu(a),uniforms:Ju(a.shapes)}}function td(a,n,e,t){var r,i,o=t.label,s=o===void 0?{}:o,l=t.shapes,u=t.layers,d=t.getAttributeTexture,h=t.shapeGlobalIds;if(l.length===0)throw new Error("createBackdropProgram: at least one shape must be provided in 'shapes'");var c=(r=s.margin)!==null&&r!==void 0?r:5,f=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},g={shapes:l,layers:u,shapeGlobalIds:h},b=ed(g),m=(function(v){function p(){var y;ne(this,p);for(var T=arguments.length,_=new Array(T),S=0;S<T;S++)_[S]=arguments[S];return y=ye(this,p,[].concat(_)),w(y,"totalBackdropCount",0),w(y,"bufferCapacity",0),y}return _e(p,v),ae(p,[{key:"getDefinition",value:function(){var T=WebGL2RenderingContext,_=T.FLOAT,S=T.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:b.vertexShader,FRAGMENT_SHADER_SOURCE:b.fragmentShader,METHOD:S,UNIFORMS:b.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:_},{name:"a_labelWidth",size:1,type:_},{name:"a_labelHeight",size:1,type:_},{name:"a_textHeight",size:1,type:_},{name:"a_positionMode",size:1,type:_},{name:"a_labelAngle",size:1,type:_},{name:"a_backdropColor",size:4,type:_},{name:"a_backdropShadowColor",size:4,type:_},{name:"a_backdropShadowBlur",size:1,type:_},{name:"a_backdropPadding",size:1,type:_},{name:"a_backdropBorderColor",size:4,type:_},{name:"a_backdropExtra",size:4,type:_},{name:"a_labelBoxOffset",size:2,type:_}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:_}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processBackdrop",value:function(T,_){var S=this.floats,D=this.STRIDE,A=T*D;S[A++]=_.nodeIndex,S[A++]=_.labelWidth,S[A++]=_.labelHeight,S[A++]=_.textHeight,S[A++]=Yt[_.position],S[A++]=_.labelAngle,S[A++]=_.backdropColor[0],S[A++]=_.backdropColor[1],S[A++]=_.backdropColor[2],S[A++]=_.backdropColor[3],S[A++]=_.backdropShadowColor[0],S[A++]=_.backdropShadowColor[1],S[A++]=_.backdropShadowColor[2],S[A++]=_.backdropShadowColor[3],S[A++]=_.backdropShadowBlur,S[A++]=_.backdropPadding,S[A++]=_.backdropBorderColor[0],S[A++]=_.backdropBorderColor[1],S[A++]=_.backdropBorderColor[2],S[A++]=_.backdropBorderColor[3],S[A++]=_.backdropBorderWidth,S[A++]=_.backdropCornerRadius,S[A++]=_.backdropLabelPadding,S[A++]=_.backdropArea,S[A++]=_.labelBoxOffset[0],S[A++]=_.labelBoxOffset[1]}},{key:"setUniforms",value:function(T,_){var S=_.gl,D=_.uniformLocations;S.uniformMatrix3fv(D.u_matrix,!1,T.matrix),S.uniform1f(D.u_sizeRatio,T.sizeRatio),S.uniform1f(D.u_correctionRatio,T.correctionRatio),S.uniform1f(D.u_cameraAngle,T.cameraAngle),S.uniform2f(D.u_resolution,T.width*T.pixelRatio,T.height*T.pixelRatio),S.uniform1f(D.u_pixelRatio,T.pixelRatio),S.uniform1f(D.u_labelMargin,p.labelMargin),S.uniform1f(D.u_zoomLabelSizeRatio,1/f(T.zoomRatio)),S.uniform1f(D.u_labelPixelSnapping,T.labelPixelSnapping),S.uniform1i(D.u_nodeDataTexture,T.nodeDataTextureUnit),S.uniform1i(D.u_nodeDataTextureWidth,T.nodeDataTextureWidth),S.uniform1i(D.u_nodeFrameTexture,T.nodeFrameTextureUnit),S.uniform1i(D.u_nodeFrameTextureWidth,T.nodeFrameTextureWidth);var A=d();D.u_layerAttributeTexture&&A&&(A.bind(pt),S.uniform1i(D.u_layerAttributeTexture,pt),S.uniform1i(D.u_layerAttributeTextureWidth,A.getTextureWidth()),S.uniform1i(D.u_layerAttributeTexelsPerNode,A.getTexelsPerItem()));var E=U(En(l)),L;try{for(E.s();!(L=E.n()).done;){var z=L.value;this.setTypedUniform(z,_)}}catch(C){E.e(C)}finally{E.f()}}},{key:"hasNothingToRender",value:function(){return this.totalBackdropCount===0}},{key:"drawWebGL",value:function(T,_){var S=_.gl;this.totalBackdropCount!==0&&(this.isInstanced?S.drawArraysInstanced(T,0,this.VERTICES,this.totalBackdropCount):S.drawArrays(T,0,this.totalBackdropCount*this.VERTICES))}},{key:"reallocate",value:function(T){this.totalBackdropCount=T,T>this.bufferCapacity&&(this.bufferCapacity=Math.max(T,Math.ceil(this.bufferCapacity*1.5)||10),De(p,"reallocate",this,3)([this.bufferCapacity]))}}])})(Rt);return w(m,"labelMargin",c),new m(a,n,e)}var _o=(function(a){function n(){var e;ne(this,n);for(var t=arguments.length,r=new Array(t),i=0;i<t;i++)r[i]=arguments[i];return e=ye(this,n,[].concat(r)),w(e,"totalCharacterCount",0),w(e,"bufferCapacity",0),e}return _e(n,a),ae(n,[{key:"processLabel",value:function(t,r,i){if(i.hidden||!i.text)return 0;for(var o=i.text,s=o.length,l=0;l<s;l++){var u=o[l];this.processCharacter(r+l,i,u,l)}return s}},{key:"hasNothingToRender",value:function(){return this.totalCharacterCount===0}},{key:"drawWebGL",value:function(t,r){var i=r.gl;this.totalCharacterCount!==0&&(this.isInstanced?i.drawArraysInstanced(t,0,this.VERTICES,this.totalCharacterCount):i.drawArrays(t,0,this.totalCharacterCount*this.VERTICES))}},{key:"reallocate",value:function(t){this.totalCharacterCount=t,t>this.bufferCapacity&&(this.bufferCapacity=Math.max(t,Math.ceil(this.bufferCapacity*1.5)||1e3),De(n,"reallocate",this,3)([this.bufferCapacity]))}}])})(Rt);function nd(){var a=`#version 300 es

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

`.concat(st,`
`).concat(wt,`
`).concat(Qt,`
`).concat(po,`

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

  `).concat(go,`

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
`);return a}var ad=`#version 300 es
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
`;function rd(a,n,e,t){var r,i,o=t.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Zt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=nd(),h=(function(c){function f(){var g;ne(this,f);for(var b=arguments.length,m=new Array(b),v=0;v<b;v++)m[v]=arguments[v];return g=ye(this,f,[].concat(m)),w(g,"totalCount",0),w(g,"bufferCapacity",0),g}return _e(f,c),ae(f,[{key:"getDefinition",value:function(){var b=WebGL2RenderingContext,m=b.FLOAT,v=b.UNSIGNED_BYTE,p=b.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:ad,METHOD:p,UNIFORMS:["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:m},{name:"a_id",size:4,type:v,normalized:!0},{name:"a_color",size:4,type:v,normalized:!0},{name:"a_labelWidth",size:1,type:m},{name:"a_labelHeight",size:1,type:m},{name:"a_textHeight",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_labelAngle",size:1,type:m},{name:"a_padding",size:1,type:m}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:m}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processLabelBackground",value:function(b,m){var v=this.floats,p=this.ints,y=b*this.STRIDE;v[y++]=m.nodeIndex,p[y++]=m.id,v[y++]=m.color,v[y++]=m.labelWidth,v[y++]=m.labelHeight,v[y++]=m.textHeight,v[y++]=m.positionMode,v[y++]=m.labelAngle,v[y++]=m.padding}},{key:"setUniforms",value:function(b,m){var v=m.gl,p=m.uniformLocations;v.uniformMatrix3fv(p.u_matrix,!1,b.matrix),v.uniform1f(p.u_sizeRatio,b.sizeRatio),v.uniform1f(p.u_correctionRatio,b.correctionRatio),v.uniform1f(p.u_cameraAngle,b.cameraAngle),v.uniform2f(p.u_resolution,b.width*b.pixelRatio,b.height*b.pixelRatio),v.uniform1f(p.u_pixelRatio,b.pixelRatio),v.uniform1f(p.u_labelMargin,f.labelMargin),v.uniform1f(p.u_zoomLabelSizeRatio,1/u(b.zoomRatio)),v.uniform1f(p.u_labelPixelSnapping,b.labelPixelSnapping),v.uniform1f(p.u_pickingPadding,b.labelPickingPadding),v.uniform1i(p.u_nodeDataTexture,b.nodeDataTextureUnit),v.uniform1i(p.u_nodeDataTextureWidth,b.nodeDataTextureWidth),v.uniform1i(p.u_nodeFrameTexture,b.nodeFrameTextureUnit),v.uniform1i(p.u_nodeFrameTextureWidth,b.nodeFrameTextureWidth)}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(b,m){var v=m.gl;this.totalCount!==0&&v.drawArraysInstanced(v.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(b){this.totalCount=b,b>this.bufferCapacity&&(this.bufferCapacity=Math.max(b,Math.ceil(this.bufferCapacity*1.5)||10),De(f,"reallocate",this,3)([this.bufferCapacity]))}}])})(Rt);return w(h,"labelMargin",l),new h(a,n,e)}var Qe={fontSize:64,buffer:8,radius:24,cutoff:.25,maxTextureSize:2048,debounceTimeout:100},bn=2,_n=1e20;function id(a,n,e,t,r){var i=a+r*4,o=document.createElement("canvas");o.width=i,o.height=i;var s=o.getContext("2d",{willReadFrequently:!0});return s.font="".concat(t," ").concat(e," ").concat(a,"px ").concat(n),s.textBaseline="alphabetic",s.textAlign="left",s.fillStyle="black",{ctx:s,canvasSize:i,gridOuter:new Float64Array(i*i),gridInner:new Float64Array(i*i),f:new Float64Array(i),z:new Float64Array(i+1),v:new Uint16Array(i)}}function od(a,n,e,t,r){var i=a.ctx,o=a.canvasSize,s=i.measureText(n),l=s.width,u=s.actualBoundingBoxAscent,d=s.actualBoundingBoxDescent,h=s.actualBoundingBoxLeft,c=s.actualBoundingBoxRight,f=Math.ceil(u),g=Math.ceil(h),b=Math.max(0,Math.min(o-e,Math.ceil(h)+Math.ceil(c))),m=Math.min(o-e,f+Math.ceil(d)),v=b+2*e,p=m+2*e,y=Math.max(v*p,0),T=new Uint8ClampedArray(y),_={data:T,width:v,height:p,glyphWidth:b,glyphHeight:m,glyphTop:f,glyphLeft:g,glyphAdvance:l};if(b===0||m===0)return _;var S=a.gridInner,D=a.gridOuter;i.clearRect(e,e,b,m),i.fillText(n,e+g,e+f);var A=i.getImageData(e,e,b,m);D.fill(_n,0,y),S.fill(0,0,y);for(var E=0;E<m;E++)for(var L=0;L<b;L++){var z=A.data[4*(E*b+L)+3]/255;if(z!==0){var C=(E+e)*v+L+e;if(z===1)D[C]=0,S[C]=_n;else{var N=.5-z;D[C]=N>0?N*N:0,S[C]=N<0?N*N:0}}}no(D,0,0,v,p,v,a.f,a.v,a.z),no(S,e,e,b,m,v,a.f,a.v,a.z);for(var k=0;k<y;k++){var F=Math.sqrt(D[k])-Math.sqrt(S[k]);T[k]=Math.round(255-255*(F/t+r))}return _}function no(a,n,e,t,r,i,o,s,l){for(var u=n;u<n+t;u++)ao(a,e*i+u,i,r,o,s,l);for(var d=e;d<e+r;d++)ao(a,d*i+n,1,t,o,s,l)}function ao(a,n,e,t,r,i,o){i[0]=0,o[0]=-_n,o[1]=_n,r[0]=a[n];for(var s=1,l=0,u=0;s<t;s++){r[s]=a[n+s*e];var d=s*s;do{var h=i[l];u=(r[s]-r[h]+d-h*h)/(s-h)/2}while(u<=o[l]&&--l>-1);l++,i[l]=s,o[l]=u,o[l+1]=_n}for(var c=0,f=0;c<t;c++){for(;o[f+1]<c;)f++;var g=i[f],b=c-g;a[n+c*e]=r[g]+b*b}}var Dt=(function(a){function n(){var e,t=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{};return ne(this,n),e=ye(this,n),w(e,"fonts",new Map),w(e,"textures",[]),w(e,"cursor",{x:0,y:0,rowHeight:0,atlasIndex:0}),w(e,"pendingGlyphs",[]),w(e,"debounceTimer",null),e.options=O(O({},Qe),t),e.canvas=document.createElement("canvas"),e.canvas.width=e.options.maxTextureSize,e.canvas.height=e.options.maxTextureSize,e.ctx=e.canvas.getContext("2d",{willReadFrequently:!0}),e.measureCanvas=document.createElement("canvas"),e.measureCtx=e.measureCanvas.getContext("2d"),e.textures.push(e.ctx.getImageData(0,0,1,1)),e}return _e(n,a),ae(n,[{key:"getFontKey",value:function(t){return"".concat(t.family,"-").concat(t.weight,"-").concat(t.style)}},{key:"registerFont",value:function(t){var r=this.getFontKey(t);if(this.fonts.has(r))return r;var i=id(this.options.fontSize,t.family,t.weight,t.style,this.options.buffer);return this.fonts.set(r,{descriptor:t,generator:i,glyphs:new Map}),r}},{key:"ensureGlyphs",value:function(t,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered. Call registerFont() first.'));var o=!1,s=U(t),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.codePointAt(0);d!==void 0&&(i.glyphs.has(d)||(this.pendingGlyphs.push({fontKey:r,charCode:d}),o=!0))}}catch(h){s.e(h)}finally{s.f()}o&&this.scheduleTextureGeneration()}},{key:"measureText",value:function(t,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered.'));var o=i.descriptor,s=o.family,l=o.weight,u=o.style;return this.measureCtx.font="".concat(u," ").concat(l," ").concat(this.options.fontSize,"px ").concat(s),this.measureCtx.measureText(t).width}},{key:"getGlyph",value:function(t,r){var i=this.fonts.get(r);if(i)return i.glyphs.get(t)}},{key:"getTextures",value:function(){return this.textures}},{key:"getFontCount",value:function(){return this.fonts.size}},{key:"getGlyphCount",value:function(){var t=0,r=U(this.fonts.values()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;t+=o.glyphs.size}}catch(s){r.e(s)}finally{r.f()}return t}},{key:"hasPendingGlyphs",value:function(){return this.pendingGlyphs.length>0}},{key:"flush",value:function(){this.debounceTimer&&(clearTimeout(this.debounceTimer),this.debounceTimer=null),this.generateTextures()}},{key:"destroy",value:function(){this.debounceTimer&&clearTimeout(this.debounceTimer),this.fonts.clear(),this.textures=[],this.pendingGlyphs=[],this.removeAllListeners()}},{key:"scheduleTextureGeneration",value:function(){var t=this;this.debounceTimer===null&&(this.options.debounceTimeout===null?this.generateTextures():this.debounceTimer=setTimeout(function(){t.debounceTimer=null,t.generateTextures()},this.options.debounceTimeout))}},{key:"generateTextures",value:function(){if(this.pendingGlyphs.length!==0){var t=this.options,r=t.maxTextureSize,i=t.buffer,o=t.radius,s=t.cutoff,l=U(this.pendingGlyphs),u;try{for(l.s();!(u=l.n()).done;){var d=u.value,h=d.fontKey,c=d.charCode,f=this.fonts.get(h);if(!(!f||f.glyphs.has(c))){var g=String.fromCodePoint(c),b=od(f.generator,g,i,o,s),m=b.width,v=b.height;this.cursor.x+m+bn>r&&(this.cursor.x=0,this.cursor.y+=this.cursor.rowHeight+bn,this.cursor.rowHeight=0),this.cursor.y+v+bn>r&&(this.finalizeCurrentTexture(),this.cursor={x:0,y:0,rowHeight:0,atlasIndex:this.cursor.atlasIndex+1},this.ctx.clearRect(0,0,r,r));for(var p=b.data,y=new Uint8ClampedArray(m*v*4),T=0;T<p.length;T++){var _=T*4;y[_]=255,y[_+1]=255,y[_+2]=255,y[_+3]=p[T]}var S=new ImageData(y,m,v);this.ctx.putImageData(S,this.cursor.x,this.cursor.y);var D={charCode:c,width:b.glyphWidth,height:b.glyphHeight,bearingX:-b.glyphLeft-i,bearingY:b.glyphTop+i,advance:b.glyphAdvance,atlasX:this.cursor.x,atlasY:this.cursor.y,atlasWidth:m,atlasHeight:v,atlasIndex:this.cursor.atlasIndex};f.glyphs.set(c,D),this.cursor.x+=m+bn,this.cursor.rowHeight=Math.max(this.cursor.rowHeight,v)}}}catch(A){l.e(A)}finally{l.f()}this.finalizeCurrentTexture(),this.pendingGlyphs=[],this.emit(n.ATLAS_UPDATED_EVENT,{textures:this.textures,glyphCount:this.getGlyphCount()})}}},{key:"finalizeCurrentTexture",value:function(){var t=this.options.maxTextureSize,r=this.cursor.y>0||this.cursor.rowHeight>0,i=Math.min(t,Math.max(this.cursor.x,r?t:1)),o=Math.min(t,this.cursor.y+this.cursor.rowHeight+bn),s=this.ctx.getImageData(0,0,i,o);this.cursor.atlasIndex>=this.textures.length?this.textures.push(s):this.textures[this.cursor.atlasIndex]=s}}])})(lo.EventEmitter);w(Dt,"ATLAS_UPDATED_EVENT","atlasUpdated");var sd=Qe.fontSize;function ld(){var a=`  // -------------------------------------------------------------------------
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
const float ATLAS_FONT_SIZE = `.concat(Se(sd),`;

// ============================================================================
// Helper Functions
// ============================================================================

`).concat(st,`
`).concat(wt,`
`).concat(Qt,`
`).concat(vo,`

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
`);return n}function ud(){var a=`#version 300 es
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
`;return a}function dd(){return["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_pixelRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_zoomLabelSizeRatio","u_labelPixelSnapping"]}function hd(){return{vertexShader:ld(),fragmentShader:ud(),uniforms:dd()}}function cd(a,n,e,t){var r,i,o=t.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Zt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=hd(),h=(function(c){function f(g,b,m){var v,p,y,T;if(ne(this,f),T=ye(this,f,[g,b,m]),w(T,"atlasTexture",null),w(T,"atlasNeedsUpdate",!1),w(T,"labelGlyphCache",new Map),T.atlasFontSize=Qe.fontSize*mn(),T.atlasManager=new Dt({fontSize:T.atlasFontSize}),T.gamma=.025,T.sdfBuffer=Qe.cutoff,T.atlasTexture=g.createTexture(),!T.atlasTexture)throw new Error("NodeLabelProgram: failed to create atlas texture");g.bindTexture(g.TEXTURE_2D,T.atlasTexture),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_S,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_T,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MIN_FILTER,g.LINEAR),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MAG_FILTER,g.LINEAR),g.bindTexture(g.TEXTURE_2D,null),T.atlasManager.on(Dt.ATLAS_UPDATED_EVENT,function(){T.atlasNeedsUpdate=!0});var _={family:((v=s.font)===null||v===void 0?void 0:v.family)||"sans-serif",weight:((p=s.font)===null||p===void 0?void 0:p.weight)||"normal",style:((y=s.font)===null||y===void 0?void 0:y.style)||"normal"};return T.defaultFontKey=T.atlasManager.registerFont(_),T}return _e(f,c),ae(f,[{key:"getDefinition",value:function(){var b=WebGL2RenderingContext,m=b.FLOAT,v=b.UNSIGNED_BYTE,p=b.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d.vertexShader,FRAGMENT_SHADER_SOURCE:d.fragmentShader,METHOD:p,UNIFORMS:d.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:m},{name:"a_charOffset",size:2,type:m},{name:"a_charSize",size:2,type:m},{name:"a_texCoords",size:4,type:m},{name:"a_color",size:4,type:v,normalized:!0},{name:"a_margin",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_labelWidth",size:1,type:m},{name:"a_labelHeight",size:1,type:m},{name:"a_verticalCenter",size:1,type:m},{name:"a_textHeight",size:1,type:m},{name:"a_labelAngle",size:1,type:m}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:m}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(b,m){if(m.hidden||!m.text){this.labelGlyphCache.delete(b);return}var v=m.text,p=m.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(v,p);var y=[],T=[],_=0,S=0,D=0,A=U(v),E;try{for(A.s();!(E=A.n()).done;){var L=E.value,z=L.codePointAt(0);if(z===void 0){y.push(void 0),T.push(_);continue}var C=this.atlasManager.getGlyph(z,p);y.push(C),T.push(_),C&&(_+=C.advance,S=Math.max(S,C.bearingY),D=Math.max(D,C.atlasHeight-C.bearingY))}}catch(N){A.e(N)}finally{A.f()}this.labelGlyphCache.set(b,{glyphs:y,xOffsets:T,totalWidth:_,totalHeight:S+D,verticalCenterOffset:(S-D)/2})}},{key:"processCharacter",value:function(b,m,v,p){var y=this.floats,T=this.STRIDE,_=b*T,S=this.labelGlyphCache.get(m.parentKey);if(!S||!S.glyphs[p]){for(var D=0;D<T;D++)y[_+D]=0;return}var A=S.glyphs[p],E=S.xOffsets[p],L=m.size/this.atlasFontSize,z=$e(m.color),C=_;y[C++]=m.nodeIndex,y[C++]=(E+A.bearingX)*L,y[C++]=-A.bearingY*L,y[C++]=A.atlasWidth*L,y[C++]=A.atlasHeight*L,y[C++]=A.atlasX,y[C++]=A.atlasY,y[C++]=A.atlasWidth,y[C++]=A.atlasHeight,y[C++]=z,y[C++]=m.margin,y[C++]=Yt[m.position],y[C++]=S.totalWidth*L,y[C++]=m.size,y[C++]=S.verticalCenterOffset*L,y[C++]=S.totalHeight*L,y[C++]=m.labelAngle}},{key:"processLabel",value:function(b,m,v){return this.prepareLabelGlyphs(b,v),De(f,"processLabel",this,3)([b,m,v])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var b=this.normalProgram.gl,m=this.atlasManager.getTextures();if(m.length!==0){var v=m[0];b.bindTexture(b.TEXTURE_2D,this.atlasTexture),b.texImage2D(b.TEXTURE_2D,0,b.RGBA,v.width,v.height,0,b.RGBA,b.UNSIGNED_BYTE,v.data),b.bindTexture(b.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(b,m){var v=m.gl,p=m.uniformLocations;v.uniformMatrix3fv(p.u_matrix,!1,b.matrix),v.uniform1f(p.u_sizeRatio,b.sizeRatio),v.uniform1f(p.u_correctionRatio,b.correctionRatio),v.uniform1f(p.u_cameraAngle,b.cameraAngle),v.uniform2f(p.u_resolution,b.width*b.pixelRatio,b.height*b.pixelRatio);var y=this.atlasManager.getTextures();y.length>0?v.uniform2f(p.u_atlasSize,y[0].width,y[0].height):v.uniform2f(p.u_atlasSize,1,1),v.activeTexture(v.TEXTURE0),v.bindTexture(v.TEXTURE_2D,this.atlasTexture),v.uniform1i(p.u_atlas,0),p.u_nodeDataTexture!==void 0&&v.uniform1i(p.u_nodeDataTexture,b.nodeDataTextureUnit),p.u_nodeDataTextureWidth!==void 0&&v.uniform1i(p.u_nodeDataTextureWidth,b.nodeDataTextureWidth),v.uniform1i(p.u_nodeFrameTexture,b.nodeFrameTextureUnit),v.uniform1i(p.u_nodeFrameTextureWidth,b.nodeFrameTextureWidth),v.uniform1f(p.u_gamma,this.gamma),v.uniform1f(p.u_sdfBuffer,this.sdfBuffer),v.uniform1f(p.u_pixelRatio,b.pixelRatio),v.uniform1f(p.u_zoomLabelSizeRatio,1/f.zoomToLabelSizeRatioFunction(b.zoomRatio)),v.uniform1f(p.u_labelPixelSnapping,b.labelPixelSnapping)}},{key:"renderProgram",value:function(b,m){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),De(f,"renderProgram",this,3)([b,m])}},{key:"registerFont",value:function(b){var m=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",v=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:b,weight:m,style:v})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"measureLabel",value:function(b,m,v){var p=v||this.defaultFontKey;this.atlasManager.ensureGlyphs(b,p),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var y=0,T=0,_=0,S=U(b),D;try{for(S.s();!(D=S.n()).done;){var A=D.value,E=A.codePointAt(0);if(E!==void 0){var L=this.atlasManager.getGlyph(E,p);L&&(y+=L.advance,T=Math.max(T,L.bearingY),_=Math.max(_,L.atlasHeight-L.bearingY))}}}catch(C){S.e(C)}finally{S.f()}var z=m/this.atlasFontSize;return{width:y*z,height:m,textHeight:(T+_)*z}}},{key:"ensureGlyphsReady",value:function(b,m){var v=m||this.defaultFontKey,p=U(b),y;try{for(p.s();!(y=p.n()).done;){var T=y.value;this.atlasManager.ensureGlyphs(T,v)}}catch(_){p.e(_)}finally{p.f()}this.atlasManager.flush()}},{key:"kill",value:function(){var b=this.normalProgram.gl;this.atlasTexture&&(b.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),De(f,"kill",this,3)([])}}])})(_o);return w(h,"labelMargin",l),w(h,"zoomToLabelSizeRatioFunction",u),new h(a,n,e)}var fd=`#version 300 es
precision highp float;

in float v_edgeDist;

// R32F target: only the .r channel is stored.
out vec4 fragColor;

void main() {
  fragColor = vec4(v_edgeDist, 0.0, 0.0, 0.0);
}
`;function gd(a,n,e){var t=ur(a),r=En(a).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),i=Mu(a,e),o=i.code,s=i.multiShape,l=Jt(a,n),u=Object.keys(l.specs).map(function(g){return"float v_".concat(g,";")}).join(`
`),d=Kt(l,{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),h=d.fetchCode,c=d.varyingAssignments,f=`#version 300 es
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

`).concat(st,`
`).concat(wt,`
`).concat(t,`
`).concat(o,`
`).concat(vo,`

void main() {
  int nodeIdx = int(a_nodeIndex);
`).concat(h,`
`).concat(c,`
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
`);return f}var xo=(function(){function a(n,e){ne(this,a),w(this,"uniformLocations",{});var t=e.shapes,r=e.layers,i=e.shapeGlobalIds;if(t.length===0)throw new Error("NodeLabelFramePass: at least one shape must be provided");this.gl=n,this.shapeUniforms=En(t),this.hasAttributeData=Object.keys(Jt(t,r).offsets).length>0,this.vertexShader=or(n,gd(t,r,i)),this.fragmentShader=sr(n,fd),this.program=lr(n,[this.vertexShader,this.fragmentShader]);var o=["u_cameraAngle","u_nodeDataTexture","u_nodeDataTextureWidth","u_frameTextureWidth","u_frameTextureHeight","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"].concat(Z(this.shapeUniforms.map(function(v){return v.name}))),s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;this.uniformLocations[u]=n.getUniformLocation(this.program,u)}}catch(v){s.e(v)}finally{s.f()}var d=a.FLOATS_PER_POINT*4;this.vao=n.createVertexArray(),this.buffer=n.createBuffer(),n.bindVertexArray(this.vao),n.bindBuffer(n.ARRAY_BUFFER,this.buffer);var h=U([["a_nodeIndex",0],["a_positionMode",4],["a_labelAngle",8]]),c;try{for(h.s();!(c=h.n()).done;){var f=le(c.value,2),g=f[0],b=f[1],m=n.getAttribLocation(this.program,g);m>=0&&(n.enableVertexAttribArray(m),n.vertexAttribPointer(m,1,n.FLOAT,!1,d,b))}}catch(v){h.e(v)}finally{h.f()}n.bindVertexArray(null)}return ae(a,[{key:"run",value:function(e,t,r,i,o){if(t!==0){var s=this.gl;s.useProgram(this.program),s.bindVertexArray(this.vao),s.bindBuffer(s.ARRAY_BUFFER,this.buffer),s.bufferData(s.ARRAY_BUFFER,e.subarray(0,t*a.FLOATS_PER_POINT),s.DYNAMIC_DRAW);var l=this.uniformLocations;s.uniform1f(l.u_cameraAngle,i.cameraAngle),s.uniform1i(l.u_nodeDataTexture,i.nodeDataTextureUnit),s.uniform1i(l.u_nodeDataTextureWidth,i.nodeDataTextureWidth),s.uniform1f(l.u_frameTextureWidth,r.getTextureWidth()),s.uniform1f(l.u_frameTextureHeight,r.getTextureHeight()),this.hasAttributeData&&o&&(o.bind(pt),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,pt),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=U(this.shapeUniforms),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;cr(s,this.uniformLocations[h.name],h)}}catch(c){u.e(c)}finally{u.f()}r.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,t),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var e=this.gl;e.deleteProgram(this.program),e.deleteShader(this.vertexShader),e.deleteShader(this.fragmentShader),e.deleteBuffer(this.buffer),e.deleteVertexArray(this.vao)}}])})();w(xo,"FLOATS_PER_POINT",3);function To(a,n,e,t,r){var i,o=t.label,s=o===void 0?{}:o,l=t.shapes;if(l.length===0)throw new Error("createNodeProgram: at least one shape must be provided in 'shapes'");var u={},d=[],h;l.forEach(function(D,A){var E=Ru(D);A===0&&(h=E),u[D.name]=A,d[A]=Tn(E)});var c=Z(t.layers),f=to({shapes:l,layers:c,shapeGlobalIds:l.length>1?d:void 0,antialias:r}),g=cd(a,null,e,{label:s}),b=td(a,null,e,{shapes:l,layers:c,getAttributeTexture:function(){return S.getAttributeTexture()},label:s,shapeGlobalIds:l.length>1?d:void 0}),m=rd(a,n,e,{label:s}),v=t.labelAttachments&&Object.keys(t.labelAttachments).length>0?ju(a,null,e,{label:s}):null,p=new xo(a,{shapes:l,layers:c,shapeGlobalIds:l.length>1?d:void 0}),y=Ea(l,c),T=je(y),_=(i=(function(D){function A(E,L,z){var C;ne(this,A),C=ye(this,A,[E,L,z]),w(C,"layerLifecycles",new Map),w(C,"layersNeedingRegeneration",new Set),w(C,"attrDescriptors",[]),C._pickingBuffer=L;var N=_.layerTextures.get(E);N||(N=new _a(E,T),_.layerTextures.set(E,N)),C.layerAttributeTexture=N;var k=_.textureRefCounts.get(E)||0;_.textureRefCounts.set(E,k+1),C.packedAttributeData=new Float32Array(T.floatsPerItem),c.forEach(function(M,I){if(M.lifecycle){var x={gl:E,renderer:{refresh:function(){return z.refresh()}},getUniformLocation:function(G){return E.getUniformLocation(C.normalProgram.program,G)},requestShaderRegeneration:function(){C.layersNeedingRegeneration.add(I)},requestRefresh:function(){z.refresh()}},P=M.lifecycle(x);C.layerLifecycles.set(I,P)}}),C.layerLifecycles.forEach(function(M){var I;(I=M.init)===null||I===void 0||I.call(M)});var F=new Map;return C.layerLifecycles.forEach(function(M,I){M.getAttributeData&&F.set(I,M)}),C.attrDescriptors=xa(y,T,F),C}return _e(A,D),ae(A,[{key:"getDefinition",value:function(){var L=WebGL2RenderingContext,z=L.FLOAT,C=L.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:f.vertexShader,FRAGMENT_SHADER_SOURCE:f.fragmentShader,METHOD:C,UNIFORMS:f.uniforms,ATTRIBUTES:f.attributes,CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:z}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"maybeRegenerateShaders",value:function(){var L=this;if(this.layersNeedingRegeneration.size!==0){c=c.map(function(I,x){if(L.layersNeedingRegeneration.has(x)){var P=L.layerLifecycles.get(x);if(P!=null&&P.regenerate){var W=P.regenerate();return O(O({},W),{},{lifecycle:I.lifecycle})}}return I}),this.layersNeedingRegeneration.clear(),f=to({shapes:l,layers:c,shapeGlobalIds:l.length>1?d:void 0,antialias:this.renderer.getSetting("antialiasNodes")});var z=this.normalProgram.gl,C=this.normalProgram,N=C.program,k=C.buffer,F=C.vertexShader,M=C.fragmentShader;z.deleteProgram(N),z.deleteBuffer(k),z.deleteShader(F),z.deleteShader(M),this.normalProgram=this.getProgramInfo("normal",z,f.vertexShader,f.fragmentShader,this._pickingBuffer)}}},{key:"allocateNode",value:function(L){this.layerAttributeTexture.allocate(L)}},{key:"freeNode",value:function(L){this.layerAttributeTexture.free(L)}},{key:"getAttributeTexture",value:function(){return this.layerAttributeTexture}},{key:"uploadLayerTexture",value:function(){this.layerAttributeTexture.upload()}},{key:"process",value:function(L,z,C,N,k){var F=z*this.STRIDE;if(C.visibility==="hidden"){for(var M=F+this.STRIDE;F<M;F++)this.floats[F]=0;this.floats[z*this.STRIDE]=mo;return}this.processVisibleItem($t(L),F,C,N,k)}},{key:"processVisibleItem",value:function(L,z,C,N,k){var F,M=this.floats,I=this.ints;if(M[z++]=N,I[z++]=L,M[z++]=(F=C.opacity)!==null&&F!==void 0?F:1,T.floatsPerItem!==0){var x=this.packedAttributeData;Ta(this.attrDescriptors,C,x,C.color,this.layerLifecycles,0),this.layerAttributeTexture.updateAllAttributes(k,x)}}},{key:"setUniforms",value:function(L,z){var C=this,N=z.gl,k=z.uniformLocations;k.u_matrix&&N.uniformMatrix3fv(k.u_matrix,!1,L.matrix),k.u_sizeRatio&&N.uniform1f(k.u_sizeRatio,L.sizeRatio),k.u_correctionRatio&&N.uniform1f(k.u_correctionRatio,L.correctionRatio),k.u_pickingPadding&&N.uniform1f(k.u_pickingPadding,L.nodePickingPadding),k.u_cameraAngle&&N.uniform1f(k.u_cameraAngle,L.cameraAngle),k.u_nodeDataTexture&&N.uniform1i(k.u_nodeDataTexture,L.nodeDataTextureUnit),k.u_nodeDataTextureWidth&&N.uniform1i(k.u_nodeDataTextureWidth,L.nodeDataTextureWidth),k.u_layerAttributeTexture&&(this.layerAttributeTexture.bind(pt),N.uniform1i(k.u_layerAttributeTexture,pt)),k.u_layerAttributeTextureWidth&&N.uniform1i(k.u_layerAttributeTextureWidth,this.layerAttributeTexture.getTextureWidth()),k.u_layerAttributeTexelsPerNode&&N.uniform1i(k.u_layerAttributeTexelsPerNode,this.layerAttributeTexture.getTexelsPerItem()),l.forEach(function(F){F.uniforms.forEach(function(M){C.setTypedUniform(M,z)})}),c.forEach(function(F){F.uniforms.forEach(function(M){C.setTypedUniform(M,z)})})}},{key:"render",value:function(L,z,C){this.maybeRegenerateShaders(),De(A,"render",this,3)([L,z,C])}},{key:"renderProgram",value:function(L,z){var C=z.gl,N=z.program;C.useProgram(N),z===this.normalProgram&&this.layerLifecycles.forEach(function(k){var F;(F=k.beforeRender)===null||F===void 0||F.call(k)}),De(A,"renderProgram",this,3)([L,z])}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(C){var N;(N=C.kill)===null||N===void 0||N.call(C)}),this.layerLifecycles.clear();var L=this.normalProgram.gl,z=(_.textureRefCounts.get(L)||1)-1;z<=0?(this.layerAttributeTexture.kill(),_.layerTextures.delete(L),_.textureRefCounts.delete(L)):_.textureRefCounts.set(L,z),De(A,"kill",this,3)([])}}])})(Rt),w(i,"layerTextures",new WeakMap),w(i,"textureRefCounts",new WeakMap),i),S=new _(a,null,e);return{nodeProgram:S,labelProgram:g,backdropProgram:b,labelBackgroundProgram:m,attachmentProgram:v,framePass:p,shapeSlug:h,shapeNameToIndex:l.length>1?u:void 0,shapeGlobalIds:l.length>1?d:void 0}}var mt=6;function An(a){var n=a.offsets,e=a.specs,t=a.floatsPerItem,r=Object.keys(n);if(r.length===0||t===0)return{uniformDeclarations:"",uniformNames:[],vertexVaryingDeclarations:"",fragmentVaryingDeclarations:"",fetchCode:"",varyingAssignments:""};var i=`
uniform sampler2D u_edgeAttributeTexture;
uniform int u_edgeAttributeTextureWidth;
uniform int u_edgeAttributeTexelsPerEdge;`,o=["u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"],s=r.map(function(f){var g=e[f].size===1?"float":"vec".concat(e[f].size);return"".concat(g," v_").concat(f,";")}),l=s.map(function(f){return"out ".concat(f)}).join(`
`),u=s.map(function(f){return"in ".concat(f)}).join(`
`),d=Kt(a,{varPrefix:"attr",baseTexelExpr:"edgeIdx * u_edgeAttributeTexelsPerEdge",textureWidthUniform:"u_edgeAttributeTextureWidth",textureSamplerUniform:"u_edgeAttributeTexture"}),h=d.fetchCode,c=d.varyingAssignments;return{uniformDeclarations:i,uniformNames:o,vertexVaryingDeclarations:l,fragmentVaryingDeclarations:u,fetchCode:h,varyingAssignments:c}}function vd(a){return`
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
`)}function pd(a){return`
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
`)}function Eo(a){return`
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
`)}function md(a){return`
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
`)}function bd(a){return`
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
`)}function yd(a){return`
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
`)}function _d(a){return`
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
`)}function ga(a,n){var e=new RegExp("\\b(float|vec[234]|void|int|bool)\\s+".concat(n,"\\s*\\("));return e.test(a)}function So(a,n){var e=[];return ga(n,"path_".concat(a,"_length"))||e.push(md(a)),ga(n,"path_".concat(a,"_closest_t"))||e.push(bd(a)),ga(n,"path_".concat(a,"_distance"))||e.push(yd(a)),ga(n,"path_".concat(a,"_t_at_distance"))||e.push(_d(a)),e.length>0?`
// ============================================================================
// Auto-generated fallback functions (path only provided position)
// ============================================================================
`.concat(e.join(`
`)):""}function Ao(a){var n,e,t,r=a.paths,i=a.layers,o=(n=a.extremities)!==null&&n!==void 0?n:[];if(r.length===0)throw new Error("At least one path is required in 'paths'");if(i.length===0)throw new Error("At least one layer is required in 'layers'");var s=(e=a.defaultHead)!==null&&e!==void 0?e:"none",l=(t=a.defaultTail)!==null&&t!==void 0?t:"none";return{paths:r,extremities:o,layers:i,path:r[0],layer:i[0],defaultHead:s,defaultTail:l}}var Do=WebGL2RenderingContext,xn=Do.FLOAT,ro=Do.UNSIGNED_BYTE;function xd(a,n,e){var t=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"]),r=new Set(t);return a.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),n.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),e.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),Array.from(r)}function fr(a){var n=new Set,e=[],t=U(a),r;try{for(t.s();!(r=t.n()).done;){var i=r.value;n.has(i.name)||(n.add(i.name),e.push("// Path: ".concat(i.name)),e.push(i.glsl),e.push(Eo(i.name)),e.push(So(i.name,i.glsl)))}}catch(o){t.e(o)}finally{t.f()}return e.join(`

`)}function Td(a){var n=[],e=U(a),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;n.push("// Extremity: ".concat(r.name)),n.push(r.glsl)}}catch(i){e.e(i)}finally{e.f()}return n.join(`

`)}var Ed=[{queryName:"queryPathPosition",pathFunc:"position",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathTangent",pathFunc:"tangent",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathNormal",pathFunc:"normal",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathLength",pathFunc:"length",returnType:"float",params:"vec2 source, vec2 target",args:"source, target"},{queryName:"queryPathClosestT",pathFunc:"closest_t",returnType:"float",params:"vec2 p, vec2 source, vec2 target",args:"p, source, target"}];function Sd(a,n){var e=n.queryName,t=n.pathFunc,r=n.returnType,i=n.params,o=n.args;if(a.length===1)return"".concat(r," ").concat(e,"(int pathId, ").concat(i,`) {
  return path_`).concat(a[0].name,"_").concat(t,"(").concat(o,`);
}`);var s=a.map(function(l,u){return"    case ".concat(u,": return path_").concat(l.name,"_").concat(t,"(").concat(o,");")}).join(`
`);return"".concat(r," ").concat(e,"(int pathId, ").concat(i,`) {
  switch (pathId) {
`).concat(s,`
    default: return path_`).concat(a[0].name,"_").concat(t,"(").concat(o,`);
  }
}`)}function gr(a){return Ed.map(function(n){return Sd(a,n)}).join(`

`)}function Ad(a){if(a.length===1)return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  return extremity_`.concat(a[0].name,`(uv, lengthRatio, widthRatio);
}`);var n=a.map(function(e,t){return"    case ".concat(t,": return extremity_").concat(e.name,"(uv, lengthRatio, widthRatio);")}).join(`
`);return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  switch (extremityId) {
`.concat(n,`
    default: return extremity_`).concat(a[0].name,`(uv, lengthRatio, widthRatio);
  }
}`)}function Dd(a){var n=[],e=U(a),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;n.push(vd(r.name)),n.push(pd(r.name))}}catch(s){e.e(s)}finally{e.f()}if(a.length===1)n.push(`
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

`)}function wd(a,n,e,t,r){var i=je([].concat(Z(a),Z(e))),o=An(i),s=Jt(t,r),l=Object.keys(s.offsets),u=function(D){var A=s.specs[D].size;return A===1?"float":"vec".concat(A)},d=l.map(function(S){return"".concat(u(S)," v_source_").concat(S,`;
`).concat(u(S)," v_target_").concat(S,";")}).join(`
`),h=Kt(s,{varPrefix:"srcNodeAttr",baseTexelExpr:"srcIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_source_"}),c=Kt(s,{varPrefix:"tgtNodeAttr",baseTexelExpr:"tgtIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_target_"}),f=l.map(function(S){return"".concat(u(S)," g_").concat(S,";")}).join(`
`),g=l.map(function(S){return"  g_".concat(S," = v_source_").concat(S,";")}).join(`
`),b=l.map(function(S){return"  g_".concat(S," = v_target_").concat(S,";")}).join(`
`),m=new Set,v=[],p=function(D){m.has(D.name)||(m.add(D.name),v.push("uniform ".concat(D.type," ").concat(D.name,";")))};a.forEach(function(S){return S.uniforms.forEach(p)}),n.forEach(function(S){return S.uniforms.forEach(p)});var y=n.map(function(S){return Se(S.widthFactor)}).join(", "),T=Math.max.apply(Math,Z(a.map(function(S){return S.minBodyLengthRatio||0}))),_=`#version 300 es

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
`).concat(v.join(`
`),`

// Path attribute varyings \u2014 assigned so path functions can read them as globals
`).concat(o.vertexVaryingDeclarations,`

// Node size varyings \u2014 read by path functions like loop
out float v_sourceNodeSize;
out float v_targetNodeSize;

// Node shape attributes per endpoint, and the ones querySDF reads
`).concat(d,`
`).concat(f,`

// Scattered output written to the edge-frame texel: [tStart, tEnd, straightenFactor, pathLength]
out vec4 v_clamp;

// Extremity width factor array (needed for extremityScale computation)
const float EXTREMITY_WIDTH_FACTORS[`).concat(n.length,"] = float[](").concat(y,`);

// Node-data fetch helpers (geometry texel + rotation-flags texel)
`).concat(st,`
`).concat(wt,`

// Shape SDFs for node boundary clamping
`).concat(Cu(),`
`).concat(Lu(new Set(l)),`

// Path functions
`).concat(fr(a),`
`).concat(gr(a),`
`).concat(Dd(a),`

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
`).concat(c.fetchCode,`
`).concat(c.varyingAssignments,`

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
  float minBodyLengthRatio = `).concat(Se(T),`;

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
`);return _}var va=0,io=1,pa=2;function Rd(a,n,e){var t=[];e&&(t.push([va,0,-1],[va,0,1]),t.push([va,1,-1],[va,1,1]));for(var r=0;r<=a;r++){var i=r/a;t.push([io,i,-1],[io,i,1])}return n&&(t.push([pa,0,-1],[pa,0,1]),t.push([pa,1,-1],[pa,1,1])),{data:t,attributes:[{name:"a_zone",size:1,type:xn},{name:"a_zoneT",size:1,type:xn},{name:"a_side",size:1,type:xn}],verticesPerEdge:t.length}}function Cd(a,n,e,t){var r=je([].concat(Z(a),Z(e))),i=An(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture"]),s=new Set,l=[],u=function(v){!o.has(v.name)&&!s.has(v.name)&&(s.add(v.name),l.push("uniform ".concat(v.type," ").concat(v.name,";")))};a.forEach(function(m){return m.uniforms.forEach(u)}),n.forEach(function(m){return m.uniforms.forEach(u)}),e.forEach(function(m){return m.uniforms.forEach(u)});var d=t.map(function(m){var v=m.size===1?"float":"vec".concat(m.size);return"in ".concat(v," ").concat(m.name,";")}).join(`
`),h=n.map(function(m){return Se(m.widthFactor)}).join(", "),c=Math.max.apply(Math,Z(a.map(function(m){return m.minBodyLengthRatio||0}))),f=e.some(function(m){return m.needsNodeColors}),g=a.some(function(m){return m.needsNodeSize}),b=`#version 300 es

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
`).concat(f?`
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
`).concat(fr(a),`

// Path selector functions
`).concat(gr(a),`

// Node-data fetch helper (geometry texel of the two-texel node stride)
`).concat(st,`
`).concat(f?Nu:"",`
// Per-edge clamp from the frame-pass (tStart, tEnd, straightenFactor, pathLength)
`).concat(Qt,`

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
`).concat(f?`
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
  float minBodyLengthRatio = `).concat(Se(c),`;

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
`);return b}function Ld(a,n,e,t){var r=je([].concat(Z(a),Z(e))),i=An(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding"]),s=new Set,l=[],u=function(p){!o.has(p.name)&&!s.has(p.name)&&(s.add(p.name),l.push("uniform ".concat(p.type," ").concat(p.name,";")))};a.forEach(function(v){return v.uniforms.forEach(u)}),n.forEach(function(v){return v.uniforms.forEach(u)}),e.forEach(function(v){return v.uniforms.forEach(u)});var d=n.map(function(v){var p;return Se((p=v.baseRatio)!==null&&p!==void 0?p:.5)}).join(", "),h=function(p){var y,T;return n.length>1?"EXTREMITY_BASE_RATIOS[".concat(p,"]"):Se((y=(T=n[0])===null||T===void 0?void 0:T.baseRatio)!==null&&y!==void 0?y:.5)},c=n.some(function(v){return it(v.length)?!0:v.length>0}),f=e.some(function(v){return v.needsNodeColors}),g=a.some(function(v){return v.needsNodeSize}),b=e.map(function(v,p){return"  // Layer ".concat(p+1,": ").concat(v.name,`
  color = blendOver(color, layer_`).concat(v.name,"(context));")}).join(`

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
`).concat(f?`
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
`).concat(fr(a),`

// Path selector functions
`).concat(gr(a),`

// All extremity functions
`).concat(Td(n),`

// Extremity SDF selector (shared pool for head/tail)
`).concat(Ad(n),`

// Layer functions
`).concat(e.map(function(v){return v.glsl}).join(`

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
`).concat(a.every(function(v){return v.linearParameterization})?`  // All paths have linear parameterization: t maps directly to arc distance
  context.distanceFromSource = pathTNorm * visibleLength;
  context.distanceToTarget = (1.0 - pathTNorm) * visibleLength;`:a.every(function(v){return!v.linearParameterization})?`  // No paths have linear parameterization: use numerical integration
  context.distanceFromSource = computeArcLengthMulti(v_pathId, v_tStart, pathT, v_source, v_target, 16);
  context.distanceToTarget = computeArcLengthMulti(v_pathId, pathT, v_tEnd, v_source, v_target, 16);`:`  // Mixed parameterization: use analytical for linear paths, numerical for others
  if (`.concat(a.filter(function(v){return v.linearParameterization}).map(function(v){return"v_pathId == ".concat(a.indexOf(v))}).join(" || "),`) {
    context.distanceFromSource = pathTNorm * visibleLength;
    context.distanceToTarget = (1.0 - pathTNorm) * visibleLength;
  } else {
    context.distanceFromSource = computeArcLengthMulti(v_pathId, v_tStart, pathT, v_source, v_target, 16);
    context.distanceToTarget = computeArcLengthMulti(v_pathId, pathT, v_tEnd, v_source, v_target, 16);
  }`),`

  // Compute SDF based on zone using extremity selector (shared pool)
  float bodySDF = distFromCenter - halfThickness;
  float finalSDF;

`).concat(c?`  // Base ratios, from baseRatioLookup
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
`).concat(t?`    // Visual pass: anti-aliased edge with layers, edge opacity applied once
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
`);return m}function kd(a,n,e){var t=it(n.length)?!0:n.length>0,r=it(e.length)?!0:e.length>0;if(a.generateConstantData){var i=a.generateConstantData();return{data:i.data,attributes:i.attributes,verticesPerEdge:i.verticesPerEdge}}return Rd(a.segments,t,r)}function oo(a){var n,e=Ao(a),t=e.paths,r=e.extremities,i=e.layers,o=(n=a.antialias)!==null&&n!==void 0?n:!0,s=new Map,l=new Map,u=new Map,d=U(t),h;try{for(d.s();!(h=d.n()).done;){var c=h.value,f=U(r),g;try{for(f.s();!(g=f.n()).done;){var b=g.value,m=U(r),v;try{for(m.s();!(v=m.n()).done;){var p=v.value,y="".concat(c.name,":").concat(b.name,":").concat(p.name),T=kd(c,b,p);s.set(y,T.verticesPerEdge),l.set(y,T.data);var _=U(T.attributes),S;try{for(_.s();!(S=_.n()).done;){var D=S.value;u.has(D.name)||u.set(D.name,D)}}catch(H){_.e(H)}finally{_.f()}}}catch(H){m.e(H)}finally{m.f()}}}catch(H){f.e(H)}finally{f.f()}}}catch(H){d.e(H)}finally{d.f()}var A=Array.from(u.values()),E={};A.forEach(function(H,Y){E[H.name]=Y});var L=0,z="",C=U(s),N;try{for(C.s();!(N=C.n()).done;){var k=le(N.value,2),F=k[0],M=k[1];M>L&&(L=M,z=F)}}catch(H){C.e(H)}finally{C.f()}var I=l.get(z)||[],x=z.split(":"),P=le(x,1),W=P[0],G=t.find(function(H){return H.name===W}),R=[];G!=null&&G.generateConstantData?R=G.generateConstantData().attributes:R=[{name:"a_zone"},{name:"a_zoneT"},{name:"a_side"}];var j=I.map(function(H){var Y=new Array(A.length).fill(0);return R.forEach(function(Te,de){var q=E[Te.name];q!==void 0&&de<H.length&&(Y[q]=H[de])}),Y});return{vertexShader:Cd(t,r,i,A),fragmentShader:Ld(t,r,i,o),uniforms:xd(t,r,i),attributes:Pd(),verticesPerEdge:L,constantData:j,constantAttributes:A,vertexCountsPerCombination:s,constantDataPerCombination:l}}function Pd(a,n,e){return[{name:"a_edgeIndex",size:1,type:xn},{name:"a_color",size:4,type:ro,normalized:!0},{name:"a_id",size:4,type:ro,normalized:!0},{name:"a_opacity",size:1,type:xn}]}var Fd=`#version 300 es
precision highp float;

in vec4 v_clamp;

// RGBA32F target: stores (tStart, tEnd, straightenFactor, pathLength).
out vec4 fragColor;

void main() {
  fragColor = v_clamp;
}
`,Id=(function(){function a(n,e){ne(this,a),w(this,"uniformLocations",{});var t=e.paths,r=e.extremities,i=e.layers,o=e.nodeShapes,s=e.nodeLayers;this.gl=n,this.hasAttributeData=je([].concat(Z(t),Z(i))).floatsPerItem>0,this.hasNodeAttributeData=Object.keys(Jt(o,s).offsets).length>0,this.vertexShader=or(n,wd(t,r,i,o,s)),this.fragmentShader=sr(n,Fd),this.program=lr(n,[this.vertexShader,this.fragmentShader]);var l=new Set;this.customUniforms=[];for(var u=0,d=[].concat(Z(t),Z(r));u<d.length;u++){var h=d[u],c=U(h.uniforms),f;try{for(c.s();!(f=c.n()).done;){var g=f.value;l.has(g.name)||(l.add(g.name),this.customUniforms.push(g))}}catch(y){c.e(y)}finally{c.f()}}var b=["u_sizeRatio","u_correctionRatio","u_cameraAngle","u_minEdgeThickness","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode","u_frameTextureWidth","u_frameTextureHeight"].concat(Z(this.customUniforms.map(function(y){return y.name}))),m=U(b),v;try{for(m.s();!(v=m.n()).done;){var p=v.value;this.uniformLocations[p]=n.getUniformLocation(this.program,p)}}catch(y){m.e(y)}finally{m.f()}this.vao=n.createVertexArray()}return ae(a,[{key:"run",value:function(e,t,r,i,o){if(r!==0){var s=this.gl,l=this.uniformLocations;s.useProgram(this.program),s.bindVertexArray(this.vao),l.u_sizeRatio&&s.uniform1f(l.u_sizeRatio,e.sizeRatio),l.u_correctionRatio&&s.uniform1f(l.u_correctionRatio,e.correctionRatio),l.u_cameraAngle&&s.uniform1f(l.u_cameraAngle,e.cameraAngle),l.u_minEdgeThickness&&s.uniform1f(l.u_minEdgeThickness,e.minEdgeThickness),l.u_nodeDataTexture&&s.uniform1i(l.u_nodeDataTexture,e.nodeDataTextureUnit),l.u_nodeDataTextureWidth&&s.uniform1i(l.u_nodeDataTextureWidth,e.nodeDataTextureWidth),l.u_edgeDataTexture&&s.uniform1i(l.u_edgeDataTexture,e.edgeDataTextureUnit),l.u_edgeDataTextureWidth&&s.uniform1i(l.u_edgeDataTextureWidth,e.edgeDataTextureWidth),l.u_frameTextureWidth&&s.uniform1f(l.u_frameTextureWidth,t.getTextureWidth()),l.u_frameTextureHeight&&s.uniform1f(l.u_frameTextureHeight,t.getTextureHeight()),this.hasAttributeData&&i&&(i.bind(mt),l.u_edgeAttributeTexture&&s.uniform1i(l.u_edgeAttributeTexture,mt),l.u_edgeAttributeTextureWidth&&s.uniform1i(l.u_edgeAttributeTextureWidth,i.getTextureWidth()),l.u_edgeAttributeTexelsPerEdge&&s.uniform1i(l.u_edgeAttributeTexelsPerEdge,i.getTexelsPerItem())),this.hasNodeAttributeData&&o&&(o.bind(pt),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,pt),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=U(this.customUniforms),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;cr(s,this.uniformLocations[h.name],h)}}catch(c){u.e(c)}finally{u.f()}t.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,r),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var e=this.gl;e.deleteProgram(this.program),e.deleteShader(this.vertexShader),e.deleteShader(this.fragmentShader),e.deleteVertexArray(this.vao)}}])})();function yn(a,n,e,t,r,i){if(a.length===1)return"".concat(t," ").concat(n,"(int pathId, ").concat(r,`) {
  return path_`).concat(a[0].name,"_").concat(e,"(").concat(i,`);
}`);var o=a.map(function(s,l){return"    case ".concat(l,": return path_").concat(s.name,"_").concat(e,"(").concat(i,");")}).join(`
`);return"".concat(t," ").concat(n,"(int pathId, ").concat(r,`) {
  switch (pathId) {
`).concat(o,`
    default: return path_`).concat(a[0].name,"_").concat(e,"(").concat(i,`);
  }
}`)}var Gd=`
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
`;function zd(a,n){var e=Se(a),t=Se(n);return`
float computeEdgeLabelAlpha(float bodyLength, float textWidthWebGL) {
  float ratio = textWidthWebGL > 0.0001 ? min(bodyLength / textWidthWebGL, 1.0) : 1.0;
  if (ratio < `.concat(e,`) return 0.0;
  if (ratio < `).concat(t,") return (ratio - ").concat(e,") / (").concat(t," - ").concat(e,`);
  return 1.0;
}
`)}var Nd=`
float computeEdgeLabelScreenDir(int pathId, float t, vec2 source, vec2 target, mat3 matrix) {
  vec2 tangent = queryPathTangent(pathId, t, source, target);
  return (matrix * vec3(tangent, 0.0)).x < 0.0 ? -1.0 : 1.0;
}
`,Md=`
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
`;function wo(a){var n=a.paths,e=a.minVisibilityThreshold,t=a.fullVisibilityThreshold,r=n.some(function(s){return s.hasSharpCorners}),i=n.map(function(s){return"// --- Path: ".concat(s.name,` ---
`).concat(s.glsl,`

// Tangent/normal functions: analytical if provided, otherwise numerical
`).concat(s.analyticalTangentGlsl||Eo(s.name),`

// Auto-generated fallbacks for any missing path functions
`).concat(So(s.name,s.glsl),`

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

`.concat(st,`
`).concat(Qt,`

// ============================================================================
// Path Functions (one block per path)
// ============================================================================

`).concat(i,`

// ============================================================================
// Path Query Selectors (dispatch by pathId)
// ============================================================================

`).concat(yn(n,"queryPathPosition","position","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(yn(n,"queryPathTangent","tangent","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(yn(n,"queryPathNormal","normal","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(yn(n,"queryPathLength","length","float","vec2 source, vec2 target","source, target"),`

`).concat(yn(n,"queryPathTAtDistance","t_at_distance","float","float dist, vec2 source, vec2 target","dist, source, target"),`

`).concat(o,`

// ============================================================================
// Shared helpers (body bounds, alpha ramp, screen direction, perpendicular offset)
// ============================================================================

`).concat(Gd,`
`).concat(zd(e,t),`
`).concat(Nd,`
`).concat(Md,`
`)}var Wd=Qe.fontSize,Bd=new Map,Ro=24,so=(Ro+1)*2;function Ud(a){var n=a.paths,e=a.fontSizeMode,t=a.headLengthRatio,r=a.tailLengthRatio,i=a.minVisibilityThreshold,o=a.fullVisibilityThreshold,s=a.keepUpright,l=e==="scaled",u=St(),d=je([].concat(Z(n),[u])),h=An(d),c=`#version 300 es

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

const float ATLAS_FONT_SIZE = `).concat(Se(Wd),`;
const float HEAD_RATIO = `).concat(Se(t),`;
const float TAIL_RATIO = `).concat(Se(r),`;
const int RIBBON_SEGMENTS = `).concat(Ro,`;

// Path attribute varyings (declared as plain locals since this shader has no FS inputs for them)
`).concat(h.vertexVaryingDeclarations.replace(/out /g,""),`

// Node size variables used by some path functions (self-loops, etc.)
float v_sourceNodeSize;
float v_targetNodeSize;

// Shared preamble: shape SDFs, path functions + selectors, clamp, helpers.
`).concat(wo({paths:n,minVisibilityThreshold:i,fullVisibilityThreshold:o}),`

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
`);return c}var Od=`#version 300 es
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
`;function Hd(a,n,e,t){var r=t.shaderConfig,i=r.paths,o=r.fontSizeMode;if(i.length===0)throw new Error("createEdgeLabelBackgroundProgram: shaderConfig must declare at least one path");var s=St(),l=je([].concat(Z(i),[s])),u=xa([].concat(Z(i),[s]),l),d=Ud(r),h=(function(c){function f(g,b,m){var v;return ne(this,f),v=ye(this,f,[g,b,m]),w(v,"totalCount",0),w(v,"bufferCapacity",0),w(v,"edgeAttributeTexture",null),v.edgeAttributeTexture=new _a(g,l),v.packedAttributeData=new Float32Array(l.floatsPerItem),v}return _e(f,c),ae(f,[{key:"getDefinition",value:function(){for(var b=WebGL2RenderingContext,m=b.FLOAT,v=b.UNSIGNED_BYTE,p=b.TRIANGLE_STRIP,y=[],T=0;T<so;T++)y.push([T]);var _=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_resolution","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth"];return o==="scaled"&&_.push("u_zoomSizeRatio"),l.floatsPerItem>0&&_.push("u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"),{VERTICES:so,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:Od,METHOD:p,UNIFORMS:_,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:m},{name:"a_edgeAttrIndex",size:1,type:m},{name:"a_baseFontSize",size:1,type:m},{name:"a_totalTextWidth",size:1,type:m},{name:"a_positionMode",size:1,type:m},{name:"a_margin",size:1,type:m},{name:"a_padding",size:1,type:m},{name:"a_color",size:4,type:v,normalized:!0},{name:"a_id",size:4,type:v,normalized:!0}],CONSTANT_ATTRIBUTES:[{name:"a_vertexIndex",size:1,type:m}],CONSTANT_DATA:y}}},{key:"processEdgeLabelBackground",value:function(b,m,v){var p=0;if(this.edgeAttributeTexture&&l.floatsPerItem>0){p=this.edgeAttributeTexture.allocate(m);var y=this.packedAttributeData;Ta(u,v.edgeAttributes,y,"",Bd,0),this.edgeAttributeTexture.updateAllAttributes(m,y)}var T=this.floats,_=this.ints,S=b*this.STRIDE;T[S++]=v.edgeIndex,T[S++]=p,T[S++]=v.baseFontSize,T[S++]=v.totalTextWidth,T[S++]=v.positionMode,T[S++]=v.margin,T[S++]=v.padding,T[S++]=v.color,_[S++]=v.id}},{key:"setUniforms",value:function(b,m){var v=m.gl,p=m.uniformLocations;if(v.uniformMatrix3fv(p.u_matrix,!1,b.matrix),v.uniform1f(p.u_sizeRatio,b.sizeRatio),v.uniform1f(p.u_correctionRatio,b.correctionRatio),v.uniform1f(p.u_pixelRatio,b.pixelRatio),v.uniform1f(p.u_cameraAngle,b.cameraAngle),v.uniform2f(p.u_resolution,b.width,b.height),p.u_nodeDataTexture!==void 0&&v.uniform1i(p.u_nodeDataTexture,b.nodeDataTextureUnit),p.u_nodeDataTextureWidth!==void 0&&v.uniform1i(p.u_nodeDataTextureWidth,b.nodeDataTextureWidth),p.u_edgeDataTexture!==void 0&&v.uniform1i(p.u_edgeDataTexture,b.edgeDataTextureUnit),p.u_edgeDataTextureWidth!==void 0&&v.uniform1i(p.u_edgeDataTextureWidth,b.edgeDataTextureWidth),p.u_edgeFrameTexture!==void 0&&v.uniform1i(p.u_edgeFrameTexture,b.edgeFrameTextureUnit),p.u_edgeFrameTextureWidth!==void 0&&v.uniform1i(p.u_edgeFrameTextureWidth,b.edgeFrameTextureWidth),o==="scaled"&&p.u_zoomSizeRatio!==void 0){var y=this.renderer.getSetting("zoomToSizeRatioFunction");v.uniform1f(p.u_zoomSizeRatio,1/y(b.zoomRatio))}this.edgeAttributeTexture&&l.floatsPerItem>0&&p.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(mt),v.uniform1i(p.u_edgeAttributeTexture,mt),v.uniform1i(p.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),v.uniform1i(p.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()))}},{key:"renderProgram",value:function(b,m){this.edgeAttributeTexture&&l.floatsPerItem>0&&this.edgeAttributeTexture.upload(),De(f,"renderProgram",this,3)([b,m])}},{key:"kill",value:function(){this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),De(f,"kill",this,3)([])}},{key:"drawWebGL",value:function(b,m){var v=m.gl;this.totalCount!==0&&v.drawArraysInstanced(v.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(b){this.totalCount=b,b>this.bufferCapacity&&(this.bufferCapacity=Math.max(b,Math.ceil(this.bufferCapacity*1.5)||10),De(f,"reallocate",this,3)([this.bufferCapacity]))}}])})(Rt);return new h(a,n,e)}function Co(a){var n,e,t,r,i,o;return{paths:a.paths,headLengthRatio:(n=a.headLengthRatio)!==null&&n!==void 0?n:0,tailLengthRatio:(e=a.tailLengthRatio)!==null&&e!==void 0?e:0,fontSizeMode:(t=a.fontSizeMode)!==null&&t!==void 0?t:"fixed",minVisibilityThreshold:(r=a.minVisibilityThreshold)!==null&&r!==void 0?r:.7,fullVisibilityThreshold:(i=a.fullVisibilityThreshold)!==null&&i!==void 0?i:.8,keepUpright:(o=a.keepUpright)!==null&&o!==void 0?o:!0}}var $d=Qe.fontSize,Vd=17/64;function jd(a){var n=a.paths,e=a.hasBorder,t=e===void 0?!1:e,r=a.fontSizeMode,i=r===void 0?"fixed":r,o=a.minVisibilityThreshold,s=o===void 0?.5:o,l=a.fullVisibilityThreshold,u=l===void 0?.6:l,d=a.keepUpright,h=d===void 0?!0:d,c=i==="scaled",f=n.some(function(p){return p.hasSharpCorners}),g=St(),b=je([].concat(Z(n),[g])),m=An(b),v=`#version 300 es

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
`.concat(t?"in vec4 a_borderColor;      // Border color (RGBA, normalized)":"",`

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
`).concat(c?"uniform float u_zoomSizeRatio;  // Zoom-based size ratio from zoomToSizeRatioFunction":"",`

// Edge path attribute texture uniforms (for curvature and other path attributes)
`).concat(m.uniformDeclarations,`

// ============================================================================
// Varyings
// ============================================================================

out vec2 v_texCoord;
out vec4 v_color;
`).concat(t?"out vec4 v_borderColor;":"",`
out float v_edgeFade;  // 0 = fully visible, 1 = fully faded (outside body)
out float v_alphaModifier;  // 0-1 based on label visibility ratio
out float v_fontScale;  // Ratio of rendered font size to atlas font size
`).concat(t?"out float v_positionMode;  // Position mode for conditional border (0=over needs border)":"",`

// ============================================================================
// Constants
// ============================================================================

const float bias = 255.0 / 254.0;
const float FADE_WIDTH_PIXELS = 15.0;  // Width of fade gradient in pixels
const float ATLAS_FONT_SIZE = `).concat(Se($d),`;  // Base font size used in SDF atlas
const float VERTICAL_CENTER_RATIO = `).concat(Se(Vd),`;  // Baseline to visual center ratio

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
`).concat(wo({paths:n,minVisibilityThreshold:s,fullVisibilityThreshold:u}),`

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
  `).concat(c?`// Scaled mode: font scales with zoom
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
  `).concat(c?`// Scaled mode: margin and text height scale with zoom (same factor as font)
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

    `).concat(f?`// -----------------------------------------------------------------------
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
    `.concat(c?"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;":"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * pixelToGraph;",`

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

      `).concat(f?`// Track which concave corners we've crossed to add skip distance
      bool crossedCorner1 = false;
      bool crossedCorner2 = false;
      float effectiveTargetDist = targetOffsetDist;`:"",`

      const int STEPS = 32;
      for (int i = 1; i <= STEPS; i++) {
        // Step along centerline t, from center toward target
        float stepT = centerT + searchDir * float(i) * abs(tSearchEnd - centerT) / float(STEPS);

        `).concat(f?`// Check for concave corner crossings and add skip distance
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

        `).concat(f?`if (accumDist + segDist >= effectiveTargetDist) {
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
  `).concat(c?"vec2 quadPosWebGL = quadPos * u_correctionRatio / u_sizeRatio; // Scaled mode":"vec2 quadPosWebGL = quadPos * pixelToGraph; // Fixed mode: use pixelToGraph for zoom-independent size",`

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
`).concat(t?`  v_borderColor = a_borderColor;
  v_borderColor.a *= bias;
  v_positionMode = positionMode;`:"",`
  v_alphaModifier = alphaModifier;

  // -------------------------------------------------------------------------
  // Step 13: Compute edge fade for soft truncation
  // -------------------------------------------------------------------------
  // Convert fade width from pixels to WebGL units
  `).concat(c?"float fadeWidthWebGL = FADE_WIDTH_PIXELS * u_correctionRatio / u_sizeRatio;":"float fadeWidthWebGL = FADE_WIDTH_PIXELS * pixelToGraph;",`

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
`);return v}function Xd(){var a=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},n=a.hasBorder,e=n===void 0?!1:n,t=`#version 300 es
precision highp float;

in vec2 v_texCoord;
in vec4 v_color;
`.concat(e?"in vec4 v_borderColor;":"",`
in float v_edgeFade;  // 0 = fully visible, 1 = fully faded
in float v_alphaModifier;  // 0-1 based on label visibility ratio
in float v_fontScale;  // Ratio of rendered font size to atlas font size
`).concat(e?"in float v_positionMode;  // Position mode (0=over, 1=above, 2=below, 3=auto)":"",`

uniform sampler2D u_atlas;
uniform float u_gamma;
uniform float u_sdfBuffer;
uniform float u_pixelRatio;
`).concat(e?"uniform float u_borderWidth;  // Border width in SDF units (normalized)":"",`

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

`).concat(e?`  // Fill alpha: fully opaque inside the glyph
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
`);return t}function qd(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,e=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"fixed",t=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_sdfBufferPixels","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"];e==="scaled"&&t.push("u_zoomSizeRatio"),n&&t.push("u_borderWidth");var r=U(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=U(o.uniforms),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;t.includes(u.name)||t.push(u.name)}}catch(d){s.e(d)}finally{s.f()}}}catch(d){r.e(d)}finally{r.f()}return t}function Kd(a){var n=a.hasBorder,e=n===void 0?!1:n,t=a.fontSizeMode,r=t===void 0?"fixed":t;return{vertexShader:jd(a),fragmentShader:Xd({hasBorder:e}),uniforms:qd(a.paths,e,r)}}var Yd=new Map;function Zd(a){switch(a){case"over":return 0;case"above":return 1;case"below":return 2;case"auto":return 3;default:return 0}}function Qd(a,n,e,t){var r=t.color,i=t.margin,o=t.textBorder,s=Co(t),l=s.paths,u=s.fontSizeMode,d=s.minVisibilityThreshold,h=s.fullVisibilityThreshold,c=s.keepUpright,f=!!o,g=Kd({paths:l,hasBorder:f,fontSizeMode:u,minVisibilityThreshold:d,fullVisibilityThreshold:h,keepUpright:c}),b=(function(m){function v(p,y,T){var _;ne(this,v),_=ye(this,v,[p,y,T]),w(_,"atlasTexture",null),w(_,"atlasNeedsUpdate",!1),w(_,"refreshTimeout",null),w(_,"labelGlyphCache",new Map),w(_,"edgeAttributeTexture",null);var S=St();if(_.attributeLayout=je([].concat(Z(l),[S])),_.attrDescriptors=xa([].concat(Z(l),[S]),_.attributeLayout),_.edgeAttributeTexture=new _a(p,_.attributeLayout),_.packedAttributeData=new Float32Array(_.attributeLayout.floatsPerItem),_.atlasManager=new Dt,_.gamma=.025,_.sdfBuffer=Qe.cutoff,_.atlasTexture=p.createTexture(),!_.atlasTexture)throw new Error("EdgeLabelProgram: failed to create atlas texture");p.bindTexture(p.TEXTURE_2D,_.atlasTexture),p.texParameteri(p.TEXTURE_2D,p.TEXTURE_WRAP_S,p.CLAMP_TO_EDGE),p.texParameteri(p.TEXTURE_2D,p.TEXTURE_WRAP_T,p.CLAMP_TO_EDGE),p.texParameteri(p.TEXTURE_2D,p.TEXTURE_MIN_FILTER,p.LINEAR),p.texParameteri(p.TEXTURE_2D,p.TEXTURE_MAG_FILTER,p.LINEAR),p.bindTexture(p.TEXTURE_2D,null),_.atlasManager.on(Dt.ATLAS_UPDATED_EVENT,function(){_.atlasNeedsUpdate=!0,_.refreshTimeout!==null&&clearTimeout(_.refreshTimeout),_.refreshTimeout=setTimeout(function(){_.refreshTimeout=null,_.renderer.refresh()},0)});var D={family:"sans-serif",weight:"normal",style:"normal"};return _.defaultFontKey=_.atlasManager.registerFont(D),_}return _e(v,m),ae(v,[{key:"getDefinition",value:function(){var y=WebGL2RenderingContext,T=y.FLOAT,_=y.UNSIGNED_BYTE,S=y.TRIANGLE_STRIP,D=new Set,A=[],E=U(l),L;try{for(E.s();!(L=E.n()).done;){var z=L.value,C=U(z.attributes),N;try{for(C.s();!(N=C.n()).done;){var k=N.value,F=k.name.startsWith("a_")?k.name:"a_".concat(k.name);D.has(F)||(D.add(F),A.push({name:F,size:k.size,type:k.type}))}}catch(M){C.e(M)}finally{C.f()}}}catch(M){E.e(M)}finally{E.f()}return{VERTICES:4,VERTEX_SHADER_SOURCE:g.vertexShader,FRAGMENT_SHADER_SOURCE:g.fragmentShader,METHOD:S,UNIFORMS:g.uniforms,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:T},{name:"a_edgeAttrIndex",size:1,type:T},{name:"a_baseFontSize",size:1,type:T},{name:"a_charMetrics",size:4,type:T},{name:"a_charDims",size:4,type:T},{name:"a_texCoords",size:4,type:T},{name:"a_labelParams",size:2,type:T},{name:"a_color",size:4,type:_,normalized:!0}].concat(Z(f?[{name:"a_borderColor",size:4,type:_,normalized:!0}]:[]),Z(A.filter(function(M){return!["a_curvature","curvature"].includes(M.name)}))),CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:T}],CONSTANT_DATA:[[0,0],[1,0],[0,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(y,T){if(T.hidden||!T.text){this.labelGlyphCache.delete(y);return}var _=T.text,S=T.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(_,S);var D=[],A=[],E=0,L=U(_),z;try{for(L.s();!(z=L.n()).done;){var C=z.value,N=C.codePointAt(0);if(N===void 0){D.push(void 0),A.push(E);continue}var k=this.atlasManager.getGlyph(N,S);D.push(k),A.push(E),k&&(E+=k.advance)}}catch(F){L.e(F)}finally{L.f()}this.labelGlyphCache.set(y,{glyphs:D,xOffsets:A,totalWidth:E})}},{key:"processCharacter",value:function(y,T,_,S){var D,A,E,L=this.floats,z=this.STRIDE,C=y*z,N=this.labelGlyphCache.get(T.parentKey);if(!N||!N.glyphs[S]){for(var k=0;k<z;k++)L[C+k]=0;return}var F=N.glyphs[S],M=N.xOffsets[S],I=v.labelColor,x=I&&oe(I)==="object"&&"color"in I&&I.color?I.color:T.color,P=$e(x),W=C;L[W++]=T.edgeIndex,L[W++]=(D=(A=this.edgeAttributeTexture)===null||A===void 0?void 0:A.getIndex(T.parentKey))!==null&&D!==void 0?D:0,L[W++]=T.size,L[W++]=M,L[W++]=F.advance,L[W++]=N.totalWidth,L[W++]=Zd(T.position),L[W++]=F.atlasWidth,L[W++]=F.atlasHeight,L[W++]=F.bearingX,L[W++]=-F.bearingY,L[W++]=F.atlasX,L[W++]=F.atlasY,L[W++]=F.atlasWidth,L[W++]=F.atlasHeight;var G=(E=v.labelMargin)!==null&&E!==void 0?E:T.margin;if(L[W++]=G,L[W++]=0,L[W++]=P,f&&o){var R;typeof o.color=="string"?R=o.color:R=o.color.color||"#ffffff",L[W++]=$e(R)}var j=new Set,H=U(l),Y;try{for(H.s();!(Y=H.n()).done;){var Te=Y.value,de=U(Te.attributes),q;try{for(de.s();!(q=de.n()).done;){var Re=q.value,V=Re.name.startsWith("a_")?Re.name.slice(2):Re.name;if(V!=="curvature"&&!j.has(V)){j.add(V);for(var te=0;te<Re.size;te++)L[W++]=0}}}catch(ue){de.e(ue)}finally{de.f()}}}catch(ue){H.e(ue)}finally{H.f()}}},{key:"processEdgeLabel",value:function(y,T,_){if(this.prepareLabelGlyphs(y,_),this.edgeAttributeTexture&&!_.hidden&&_.text){this.edgeAttributeTexture.allocate(y);var S=this.packedAttributeData;Ta(this.attrDescriptors,_.edgeAttributes,S,"",Yd,0),this.edgeAttributeTexture.updateAllAttributes(y,S)}return De(v,"processLabel",this,3)([y,T,_])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var y=this.normalProgram.gl,T=this.atlasManager.getTextures();if(T.length!==0){var _=T[0];y.bindTexture(y.TEXTURE_2D,this.atlasTexture),y.texImage2D(y.TEXTURE_2D,0,y.RGBA,_.width,_.height,0,y.RGBA,y.UNSIGNED_BYTE,_.data),y.bindTexture(y.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(y,T){var _=T.gl,S=T.uniformLocations,D=this.atlasManager.getTextures(),A=D.length>0?[D[0].width,D[0].height]:[1,1];if(_.uniformMatrix3fv(S.u_matrix,!1,y.matrix),_.uniform1f(S.u_sizeRatio,y.sizeRatio),_.uniform1f(S.u_correctionRatio,y.correctionRatio),_.uniform1f(S.u_pixelRatio,y.pixelRatio),_.uniform1f(S.u_cameraAngle,y.cameraAngle),_.uniform1f(S.u_sdfBufferPixels,Qe.buffer),_.uniform2f(S.u_resolution,y.width,y.height),_.uniform2f(S.u_atlasSize,A[0],A[1]),_.uniform1f(S.u_gamma,this.gamma),_.uniform1f(S.u_sdfBuffer,this.sdfBuffer),_.activeTexture(_.TEXTURE0),_.bindTexture(_.TEXTURE_2D,this.atlasTexture),_.uniform1i(S.u_atlas,0),S.u_nodeDataTexture!==void 0&&_.uniform1i(S.u_nodeDataTexture,y.nodeDataTextureUnit),S.u_nodeDataTextureWidth!==void 0&&_.uniform1i(S.u_nodeDataTextureWidth,y.nodeDataTextureWidth),S.u_edgeDataTexture!==void 0&&_.uniform1i(S.u_edgeDataTexture,y.edgeDataTextureUnit),S.u_edgeDataTextureWidth!==void 0&&_.uniform1i(S.u_edgeDataTextureWidth,y.edgeDataTextureWidth),S.u_edgeFrameTexture!==void 0&&_.uniform1i(S.u_edgeFrameTexture,y.edgeFrameTextureUnit),S.u_edgeFrameTextureWidth!==void 0&&_.uniform1i(S.u_edgeFrameTextureWidth,y.edgeFrameTextureWidth),f&&o&&S.u_borderWidth!==void 0){var E=o.width/Qe.buffer*this.sdfBuffer;_.uniform1f(S.u_borderWidth,E)}if(this.edgeAttributeTexture&&S.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(mt),_.uniform1i(S.u_edgeAttributeTexture,mt),_.uniform1i(S.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),_.uniform1i(S.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem())),u==="scaled"&&S.u_zoomSizeRatio!==void 0){var L=this.renderer.getSetting("zoomToSizeRatioFunction"),z=1/L(y.zoomRatio);_.uniform1f(S.u_zoomSizeRatio,z)}}},{key:"renderProgram",value:function(y,T){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),this.edgeAttributeTexture&&this.edgeAttributeTexture.upload(),De(v,"renderProgram",this,3)([y,T])}},{key:"registerFont",value:function(y){var T=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",_=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:y,weight:T,style:_})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"ensureGlyphsReady",value:function(y,T){var _=T||this.defaultFontKey,S=U(y),D;try{for(S.s();!(D=S.n()).done;){var A=D.value;this.atlasManager.ensureGlyphs(A,_)}}catch(E){S.e(E)}finally{S.f()}this.atlasManager.flush()}},{key:"measureLabelAtlasWidth",value:function(y,T){var _=T||this.defaultFontKey;this.atlasManager.ensureGlyphs(y,_),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var S=0,D=U(y),A;try{for(D.s();!(A=D.n()).done;){var E=A.value,L=E.codePointAt(0);if(L!==void 0){var z=this.atlasManager.getGlyph(L,_);z&&(S+=z.advance)}}}catch(C){D.e(C)}finally{D.f()}return S}},{key:"measureLabel",value:function(y,T,_){var S=this.measureLabelAtlasWidth(y,_),D=T/Qe.fontSize;return{width:S*D,height:T,textHeight:T}}},{key:"kill",value:function(){var y=this.normalProgram.gl;this.refreshTimeout!==null&&(clearTimeout(this.refreshTimeout),this.refreshTimeout=null),this.atlasTexture&&(y.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),De(v,"kill",this,3)([])}}])})(_o);return w(b,"labelColor",r),w(b,"labelMargin",i),new b(a,n,e)}function Jd(){var a=`
// No extremity - always returns positive (outside)
float extremity_none(vec2 uv, float lengthRatio, float widthRatio) {
  return 1.0;
}
`;return{name:"none",glsl:a,length:0,widthFactor:1,margin:0,uniforms:[],attributes:[]}}function Lo(a,n,e,t,r,i,o){var s,l,u=Ao(t),d=u.paths,h=u.layers,c=u.defaultHead,f=u.defaultTail,g=[Jd()].concat(Z(u.extremities)),b={},m={};d.forEach(function(C,N){return b[C.name]=N}),g.forEach(function(C,N){return m[C.name]=N});var v=(s=m[c])!==null&&s!==void 0?s:0,p=(l=m[f])!==null&&l!==void 0?l:0,y=null,T=je([].concat(Z(d),Z(h))),_=(function(C){function N(k,F,M){var I;ne(this,N),y||(y=oo({paths:d,extremities:g,layers:h,antialias:r})),I=ye(this,N,[k,F,M]),w(I,"layerLifecycles",new Map),w(I,"needsShaderRegeneration",!1),w(I,"edgeAttributeTexture",null),w(I,"layout",T),w(I,"attrDescriptors",[]),w(I,"lifecycleIndexOffset",d.length),I._pickingBuffer=F,I.edgeAttributeTexture=new _a(k,I.layout),I.packedAttributeData=new Float32Array(I.layout.floatsPerItem),h.forEach(function(P,W){if(P.lifecycle){var G={gl:k,renderer:{refresh:function(){return M.refresh()}},getUniformLocation:function(j){return k.getUniformLocation(I.normalProgram.program,j)},requestShaderRegeneration:function(){I.needsShaderRegeneration=!0},requestRefresh:function(){M.refresh()}};I.layerLifecycles.set(W,P.lifecycle(G))}}),I.layerLifecycles.forEach(function(P){var W;return(W=P.init)===null||W===void 0?void 0:W.call(P)});var x=new Map;return I.layerLifecycles.forEach(function(P,W){P.getAttributeData&&x.set(d.length+W,P)}),I.attrDescriptors=xa([].concat(Z(d),Z(h)),I.layout,x),I}return _e(N,C),ae(N,[{key:"getAttributeTexture",value:function(){return this.edgeAttributeTexture}},{key:"resolveEdgeIds",value:function(F,M,I){var x=F,P=0,W=v,G=p,R=M?x.selfLoopPath:I&&x.parallelPath?x.parallelPath:x.path;R&&b[R]!==void 0&&(P=b[R]),x.head&&x.head!=="none"&&m[x.head]!==void 0&&(W=m[x.head]),x.tail&&x.tail!=="none"&&m[x.tail]!==void 0&&(G=m[x.tail]);var j=g[W],H=g[G];return{pathId:P,headId:W,tailId:G,headLengthRatio:it(j.length)?0:j.length,tailLengthRatio:it(H.length)?0:H.length}}},{key:"getDefinition",value:function(){var F=WebGL2RenderingContext,M=F.TRIANGLE_STRIP,I=M,x=y;return{VERTICES:x.verticesPerEdge,VERTEX_SHADER_SOURCE:x.vertexShader,FRAGMENT_SHADER_SOURCE:x.fragmentShader,METHOD:I,UNIFORMS:x.uniforms,ATTRIBUTES:x.attributes,CONSTANT_ATTRIBUTES:x.constantAttributes,CONSTANT_DATA:x.constantData}}},{key:"maybeRegenerateShaders",value:function(){var F=this;if(this.needsShaderRegeneration){this.needsShaderRegeneration=!1;var M=h.map(function(j,H){var Y=F.layerLifecycles.get(H);return Y!=null&&Y.regenerate?Y.regenerate():j});y=oo({paths:d,extremities:g,layers:M,antialias:this.renderer.getSetting("antialiasEdges")});var I=this.normalProgram.gl,x=this.normalProgram,P=x.program,W=x.buffer,G=x.vertexShader,R=x.fragmentShader;I.deleteProgram(P),I.deleteBuffer(W),I.deleteShader(G),I.deleteShader(R),this.normalProgram=this.getProgramInfo("normal",I,y.vertexShader,y.fragmentShader,this._pickingBuffer)}}},{key:"process",value:function(F,M,I,x,P,W){var G=M*this.STRIDE;if(P.visibility==="hidden"||I.visibility==="hidden"||x.visibility==="hidden"){for(var R=G+this.STRIDE;G<R;G++)this.floats[G]=0;this.floats[M*this.STRIDE]=mo;return}this.processVisibleItem($t(F),G,I,x,P,W)}},{key:"processVisibleItem",value:function(F,M,I,x,P,W){var G,R=this.floats,j=this.ints;R[M++]=W,R[M++]=$e(P.color),j[M++]=F,R[M++]=(G=P.opacity)!==null&&G!==void 0?G:1;var H=this.packedAttributeData;Ta(this.attrDescriptors,P,H,P.color,this.layerLifecycles,this.lifecycleIndexOffset),this.edgeAttributeTexture.updateAllAttributesAtRow(W,H)}},{key:"setUniforms",value:function(F,M){var I=this,x=M.gl,P=M.uniformLocations;P.u_matrix&&x.uniformMatrix3fv(P.u_matrix,!1,F.matrix),P.u_sizeRatio&&x.uniform1f(P.u_sizeRatio,F.sizeRatio),P.u_correctionRatio&&x.uniform1f(P.u_correctionRatio,F.correctionRatio),P.u_zoomRatio&&x.uniform1f(P.u_zoomRatio,F.zoomRatio),P.u_pixelRatio&&x.uniform1f(P.u_pixelRatio,F.pixelRatio),P.u_cameraAngle&&x.uniform1f(P.u_cameraAngle,F.cameraAngle),P.u_feather&&x.uniform1f(P.u_feather,F.antiAliasingFeather),P.u_minEdgeThickness&&x.uniform1f(P.u_minEdgeThickness,F.minEdgeThickness),P.u_pickingPadding&&x.uniform1f(P.u_pickingPadding,F.edgePickingPadding),P.u_nodeDataTexture&&x.uniform1i(P.u_nodeDataTexture,F.nodeDataTextureUnit),P.u_nodeDataTextureWidth&&x.uniform1i(P.u_nodeDataTextureWidth,F.nodeDataTextureWidth),P.u_edgeDataTexture&&x.uniform1i(P.u_edgeDataTexture,F.edgeDataTextureUnit),P.u_edgeDataTextureWidth&&x.uniform1i(P.u_edgeDataTextureWidth,F.edgeDataTextureWidth),P.u_edgeFrameTexture&&x.uniform1i(P.u_edgeFrameTexture,F.edgeFrameTextureUnit),P.u_edgeFrameTextureWidth&&x.uniform1i(P.u_edgeFrameTextureWidth,F.edgeFrameTextureWidth),this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&(this.edgeAttributeTexture.bind(mt),P.u_edgeAttributeTexture&&x.uniform1i(P.u_edgeAttributeTexture,mt),P.u_edgeAttributeTextureWidth&&x.uniform1i(P.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),P.u_edgeAttributeTexelsPerEdge&&x.uniform1i(P.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()));var W=new Set;d.forEach(function(G){G.uniforms.forEach(function(R){W.has(R.name)||(W.add(R.name),I.setTypedUniform(R,M))})}),g.forEach(function(G){G.uniforms.forEach(function(R){W.has(R.name)||(W.add(R.name),I.setTypedUniform(R,M))})}),h.forEach(function(G){G.uniforms.forEach(function(R){W.has(R.name)||(W.add(R.name),I.setTypedUniform(R,M))})})}},{key:"render",value:function(F,M,I){this.maybeRegenerateShaders(),De(N,"render",this,3)([F,M,I])}},{key:"renderProgram",value:function(F,M){this.layerLifecycles.forEach(function(I){var x;return(x=I.beforeRender)===null||x===void 0?void 0:x.call(I)}),De(N,"renderProgram",this,3)([F,M])}},{key:"uploadAttributeTexture",value:function(){this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&this.edgeAttributeTexture.upload()}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(F){var M;return(M=F.kill)===null||M===void 0?void 0:M.call(F)}),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),De(N,"kill",this,3)([])}}])})(Rt),S=g[v],D=g[p],A=O({paths:d,headLengthRatio:it(S.length)?0:S.length,tailLengthRatio:it(D.length)?0:D.length},t.label),E=Co(A),L=function(){return{labelProgram:Qd(a,null,e,A),labelBackgroundProgram:Hd(a,n,e,{shaderConfig:E})}},z=new Id(a,{paths:d,extremities:g,layers:h,nodeShapes:i,nodeLayers:o});return{edgeProgram:new _(a,null,e),createLabelPrograms:L,framePass:z}}function eh(a){return oe(a)==="object"&&"uniforms"in a&&Array.isArray(a.uniforms)}function th(a){return oe(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function nh(a){return oe(a)==="object"&&"uniforms"in a&&"attributes"in a&&"segments"in a}function ah(a){return oe(a)==="object"&&"uniforms"in a&&"attributes"in a&&"length"in a}function rh(a){return oe(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function ih(a){return oe(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function oh(a){return oe(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function sh(a){return oe(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function lh(a){if(eh(a))return a;if(ih(a))return{name:a.name,glsl:a.glsl,inradiusFactor:a.inradiusFactor,uniforms:[]};throw new Error("Invalid node shape specification: ".concat(JSON.stringify(a)))}function uh(a){if(th(a))return a;if(Pi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid node layer specification: ".concat(JSON.stringify(a)))}function dh(a){if(nh(a))return a;if(oh(a))return{name:a.name,glsl:a.glsl,segments:a.segments,uniforms:[],attributes:[]};throw new Error("Invalid edge path specification: ".concat(JSON.stringify(a)))}function hh(a){if(rh(a))return a;if(Fi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid edge layer specification: ".concat(JSON.stringify(a)))}function ch(a){if(ah(a))return a;if(sh(a))return{name:a.name,glsl:a.glsl,length:a.length,widthFactor:a.widthFactor,margin:0,uniforms:[],attributes:[]};throw new Error("Invalid edge extremity specification: ".concat(JSON.stringify(a)))}function ko(a){var n,e,t=(n=a?.shapes)!==null&&n!==void 0?n:pn.shapes,r=(e=a?.layers)!==null&&e!==void 0?e:pn.layers,i=t.map(lh),o=r.map(uh);if(i.length===0)throw new Error("At least one node shape must be specified.");if(o.length===0)throw new Error("At least one node layer must be specified.");return{shapes:i,layers:o}}function fh(a){var n,e,t,r=(n=a?.paths)!==null&&n!==void 0?n:Vt.paths,i=(e=a?.extremities)!==null&&e!==void 0?e:Vt.extremities,o=(t=a?.layers)!==null&&t!==void 0?t:Vt.layers,s=r.map(dh),l=i.map(ch).filter(function(d){return d!==null}),u=o.map(hh);if(s.length===0)throw new Error("At least one edge path must be specified.");if(u.length===0)throw new Error("At least one edge layer must be specified.");return{paths:s,extremities:l,layers:u}}function Po(a,n){var e={},t=U(a),r;try{for(t.s();!(r=t.n()).done;){var i=r.value;i.variables&&Object.assign(e,i.variables)}}catch(o){t.e(o)}finally{t.f()}return Object.assign(e,n||{}),e}function Fo(a,n,e,t,r){var i=ko(t),o=i.shapes,s=i.layers,l=Po(o,t?.variables),u=To(a,n,e,{shapes:o,layers:s,label:t?.label,backdrop:t?.backdrop,labelAttachments:t?.labelAttachments},r);return O(O({},u),{},{variables:l})}function Io(a,n,e,t,r,i){var o=fh(t),s=o.paths,l=o.extremities,u=o.layers,d=ko(i),h=d.shapes,c=d.layers,f=Po(s,t?.variables),g=Lo(a,n,e,{paths:s,extremities:l,layers:u,defaultHead:t?.defaultHead,defaultTail:t?.defaultTail,label:t?.label},r,h,c);return O(O({},g),{},{variables:f,paths:s})}var gh=2;function Go(a,n,e){var t=arguments.length>3&&arguments[3]!==void 0?arguments[3]:gh,r=n.canvas,i=r.width,o=r.height,s=e.x,l=e.y,u=e.rowHeight,d=e.maxRowWidth,h={},c=[],f=U(a),g;try{for(f.s();!(g=f.n()).done;){var b=g.value,m=b.width+t,v=b.height+t;if(m>i||v>o||s+m>i&&l+u+v>o){c.push(b);continue}s+m>i&&(d=Math.max(d,s),s=0,l+=u,u=v),b.draw(n,s,l),h[b.key]={x:s,y:l,width:b.width,height:b.height},s+=m,u=Math.max(u,v)}}catch(p){f.e(p)}finally{f.f()}return d=Math.max(d,s),{atlas:h,cursor:{x:s,y:l,rowHeight:u,maxRowWidth:d},remaining:c}}function ze(a,n,e,t){var r=Object.defineProperty;try{r({},"",{})}catch{r=0}ze=function(i,o,s,l){function u(d,h){ze(i,d,function(c){return this._invoke(d,h,c)})}o?r?r(i,o,{value:s,enumerable:!l,configurable:!l,writable:!l}):i[o]=s:(u("next",0),u("throw",1),u("return",2))},ze(a,n,e,t)}function Ne(){var a,n,e=typeof Symbol=="function"?Symbol:{},t=e.iterator||"@@iterator",r=e.toStringTag||"@@toStringTag";function i(f,g,b,m){var v=g&&g.prototype instanceof s?g:s,p=Object.create(v.prototype);return ze(p,"_invoke",(function(y,T,_){var S,D,A,E=0,L=_||[],z=!1,C={p:0,n:0,v:a,a:N,f:N.bind(a,4),d:function(k,F){return S=k,D=0,A=a,C.n=F,o}};function N(k,F){for(D=k,A=F,n=0;!z&&E&&!M&&n<L.length;n++){var M,I=L[n],x=C.p,P=I[2];k>3?(M=P===F)&&(A=I[(D=I[4])?5:(D=3,3)],I[4]=I[5]=a):I[0]<=x&&((M=k<2&&x<I[1])?(D=0,C.v=F,C.n=I[1]):x<P&&(M=k<3||I[0]>F||F>P)&&(I[4]=k,I[5]=F,C.n=P,D=0))}if(M||k>1)return o;throw z=!0,F}return function(k,F,M){if(E>1)throw TypeError("Generator is already running");for(z&&F===1&&N(F,M),D=F,A=M;(n=D<2?a:A)||!z;){S||(D?D<3?(D>1&&(C.n=-1),N(D,A)):C.n=A:C.v=A);try{if(E=2,S){if(D||(k="next"),n=S[k]){if(!(n=n.call(S,A)))throw TypeError("iterator result is not an object");if(!n.done)return n;A=n.value,D<2&&(D=0)}else D===1&&(n=S.return)&&n.call(S),D<2&&(A=TypeError("The iterator does not provide a '"+k+"' method"),D=1);S=a}else if((n=(z=C.n<0)?A:y.call(T,C))!==o)break}catch(I){S=a,D=1,A=I}finally{E=1}}return{value:n,done:z}}})(f,b,m),!0),p}var o={};function s(){}function l(){}function u(){}n=Object.getPrototypeOf;var d=[][t]?n(n([][t]())):(ze(n={},t,function(){return this}),n),h=u.prototype=s.prototype=Object.create(d);function c(f){return Object.setPrototypeOf?Object.setPrototypeOf(f,u):(f.__proto__=u,ze(f,r,"GeneratorFunction")),f.prototype=Object.create(h),f}return l.prototype=u,ze(h,"constructor",u),ze(u,"constructor",l),l.displayName="GeneratorFunction",ze(u,r,"GeneratorFunction"),ze(h),ze(h,r,"Generator"),ze(h,t,function(){return this}),ze(h,"toString",function(){return"[object Generator]"}),(Ne=function(){return{w:i,m:c}})()}function zo(a,n,e,t,r,i,o){try{var s=a[i](o),l=s.value}catch(u){return void e(u)}s.done?n(l):Promise.resolve(l).then(t,r)}function tn(a){return function(){var n=this,e=arguments;return new Promise(function(t,r){var i=a.apply(n,e);function o(l){zo(i,t,r,o,s,"next",l)}function s(l){zo(i,t,r,o,s,"throw",l)}o(void 0)})}}var Sa=new Map,No=!1;function vh(a){return new Promise(function(n,e){var t=new FileReader;t.onload=function(){return n(t.result)},t.onerror=e,t.readAsDataURL(a)})}function ph(a){if(!Sa.has(a)){var n=fetch(a).then(function(e){return e.blob()}).then(vh).catch(function(){return Sa.delete(a),null});Sa.set(a,n)}return Sa.get(a)}function mh(a){return vr.apply(this,arguments)}function vr(){return vr=tn(Ne().m(function a(n){var e,t,r;return Ne().w(function(i){for(;;)switch(i.n){case 0:return e=document.createElement("div"),e.innerHTML=n,t=Array.from(e.querySelectorAll("img[src]")),r=t.filter(function(o){return!o.getAttribute("src").startsWith("data:")}),i.n=1,Promise.all(r.map((function(){var o=tn(Ne().m(function s(l){var u;return Ne().w(function(d){for(;;)switch(d.n){case 0:return d.n=1,ph(l.getAttribute("src"));case 1:u=d.v,u&&l.setAttribute("src",u);case 2:return d.a(2)}},s)}));return function(s){return o.apply(this,arguments)}})()));case 1:return i.a(2,e.innerHTML)}},a)})),vr.apply(this,arguments)}function Mo(a){return pr.apply(this,arguments)}function pr(){return pr=tn(Ne().m(function a(n){var e,t,r,i,o,s,l,u,d,h,c,f,g,b=arguments;return Ne().w(function(m){for(;;)switch(m.n){case 0:if(e=b.length>1&&b[1]!==void 0?b[1]:1,t=n instanceof SVGElement?new XMLSerializer().serializeToString(n):n,r=new DOMParser,i=r.parseFromString(t,"image/svg+xml"),o=i.querySelector("svg"),o){m.n=1;break}return m.a(2,null);case 1:if(s=parseFloat(o.getAttribute("width")||""),l=parseFloat(o.getAttribute("height")||""),(!(s>0)||!(l>0))&&(u=o.getAttribute("viewBox"),u&&(d=u.trim().split(/[\s,]+/),s=parseFloat(d[2]),l=parseFloat(d[3]))),!(!(s>0)||!(l>0))){m.n=2;break}return console.warn("Sigma: SVG label attachment has no parseable dimensions \u2014 skipped."),m.a(2,null);case 2:return h=Math.ceil(s*e),c=Math.ceil(l*e),f=new Blob([t],{type:"image/svg+xml"}),g=URL.createObjectURL(f),m.a(2,new Promise(function(v){var p=new Image;p.onload=function(){var y=document.createElement("canvas");y.width=h,y.height=c,y.getContext("2d").drawImage(p,0,0,h,c),URL.revokeObjectURL(g);try{y.getContext("2d").getImageData(0,0,1,1)}catch(T){if(T instanceof DOMException&&T.name==="SecurityError"){No||(No=!0,console.warn('Sigma: A label attachment was skipped because the rendered canvas is tainted. SVG with <foreignObject> (used by the "html" attachment type) is blocked in Chromium and Safari. Use type: "canvas" with Canvas 2D rendering instead.')),v(null);return}throw T}v(y)},p.onerror=function(){URL.revokeObjectURL(g),v(null)},p.src=g}))}},a)})),pr.apply(this,arguments)}function bh(a,n){var e=document.createElement("div");e.style.cssText="position:fixed;left:-99999px;top:0;visibility:hidden;width:max-content;height:max-content",e.innerHTML=(n?"<style>".concat(n,"</style>"):"")+a,document.body.appendChild(e);var t=e.getBoundingClientRect(),r=t.width,i=t.height;return document.body.removeChild(e),{width:Math.ceil(r),height:Math.ceil(i)}}function yh(a,n,e,t){return mr.apply(this,arguments)}function mr(){return mr=tn(Ne().m(function a(n,e,t,r){var i,o,s,l,u,d,h,c,f=arguments;return Ne().w(function(g){for(;;)switch(g.n){case 0:return i=f.length>4&&f[4]!==void 0?f[4]:1,o=n instanceof HTMLElement?n.outerHTML:n,g.n=1,mh(o);case 1:return s=g.v,(t==null||r==null)&&(l=bh(s,e),t=t??l.width,r=r??l.height),u=Math.ceil(t*i),d=Math.ceil(r*i),h=e?"<style>".concat(e,"</style>"):"",c='<svg xmlns="http://www.w3.org/2000/svg" '+'width="'.concat(u,'" height="').concat(d,'" viewBox="0 0 ').concat(t," ").concat(r,'">')+'<foreignObject width="'.concat(t,'" height="').concat(r,'">')+'<body xmlns="http://www.w3.org/1999/xhtml" style="margin:0;padding:0">'.concat(h).concat(s,"</body>")+"</foreignObject></svg>",g.a(2,Mo(c,1))}},a)})),mr.apply(this,arguments)}function _h(a){return br.apply(this,arguments)}function br(){return br=tn(Ne().m(function a(n){var e,t=arguments,r;return Ne().w(function(i){for(;;)switch(i.n){case 0:e=t.length>1&&t[1]!==void 0?t[1]:1,r=n.type,i.n=r==="canvas"?1:r==="svg"?2:r==="html"?3:4;break;case 1:return i.a(2,n.canvas);case 2:return i.a(2,Mo(n.svg,e));case 3:return i.a(2,yh(n.html,n.css,n.width,n.height,e));case 4:return i.a(2)}},a)})),br.apply(this,arguments)}var en=2048,Wo=(function(){function a(n,e,t){ne(this,a),w(this,"cache",new Map),w(this,"pending",new Set),w(this,"atlas",{}),w(this,"glTexture",null),w(this,"dirty",!0),this.gl=n,this.renderers=e,this.scheduleRender=t,this.packCanvas=document.createElement("canvas"),this.packCanvas.width=en,this.packCanvas.height=en,this.packCtx=this.packCanvas.getContext("2d")}return ae(a,[{key:"renderAttachment",value:function(e,t,r){var i=this,o="".concat(e,":").concat(t);if(!(this.cache.has(o)||this.pending.has(o))){var s=this.renderers[t];if(s){var l=s(r);if(l){this.pending.add(o);var u=r.pixelRatio;Promise.resolve(l).then((function(){var d=tn(Ne().m(function h(c){var f;return Ne().w(function(g){for(;;)switch(g.n){case 0:if(i.pending.has(o)){g.n=1;break}return g.a(2);case 1:if(i.pending.delete(o),c){g.n=2;break}return g.a(2);case 2:return g.n=3,_h(c,u);case 3:if(f=g.v,!(!f||f.width===0||f.height===0)){g.n=4;break}return g.a(2);case 4:i.cache.set(o,{image:f,width:f.width,height:f.height}),i.dirty=!0,i.scheduleRender();case 5:return g.a(2)}},h)}));return function(h){return d.apply(this,arguments)}})())}}}}},{key:"regenerateAtlas",value:function(){if(this.dirty){this.dirty=!1;var e=[];if(this.cache.forEach(function(u,d){e.push({key:d,width:u.width,height:u.height,draw:function(c,f,g){c.drawImage(u.image,f,g)}})}),e.length===0){this.atlas={},this.deleteGLTexture();return}var t={x:0,y:0,rowHeight:0,maxRowWidth:0};this.packCtx.clearRect(0,0,en,en);var r=Go(e,this.packCtx,t),i=r.atlas,o=r.remaining;this.atlas=i,o.length>0&&console.warn("Sigma: ".concat(o.length," label attachment(s) could not fit in the ").concat(en,"x").concat(en," atlas and will not be rendered.")),this.deleteGLTexture();var s=this.gl,l=s.createTexture();s.activeTexture(s.TEXTURE0+Sn),s.bindTexture(s.TEXTURE_2D,l),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!0),s.texImage2D(s.TEXTURE_2D,0,s.RGBA,s.RGBA,s.UNSIGNED_BYTE,this.packCanvas),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MIN_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MAG_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_S,s.CLAMP_TO_EDGE),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_T,s.CLAMP_TO_EDGE),this.glTexture=l}}},{key:"bindTexture",value:function(e){if(this.glTexture){var t=this.gl;t.activeTexture(t.TEXTURE0+e),t.bindTexture(t.TEXTURE_2D,this.glTexture)}}},{key:"getEntry",value:function(e,t){var r="".concat(e,":").concat(t);return this.atlas[r]||null}},{key:"invalidateNode",value:function(e){var t="".concat(e,":"),r=U(this.cache.keys()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;o.startsWith(t)&&(this.cache.delete(o),this.dirty=!0)}}catch(d){r.e(d)}finally{r.f()}var s=U(this.pending),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;u.startsWith(t)&&this.pending.delete(u)}}catch(d){s.e(d)}finally{s.f()}}},{key:"restore",value:function(){this.glTexture=null,this.dirty=!0}},{key:"clear",value:function(){this.cache.clear(),this.pending.clear(),this.atlas={},this.dirty=!0,this.deleteGLTexture()}},{key:"kill",value:function(){this.clear(),this.packCanvas=null,this.packCtx=null,this.gl=null}},{key:"deleteGLTexture",value:function(){this.glTexture&&(this.gl.deleteTexture(this.glTexture),this.glTexture=null)}}])})(),yr=(function(){function a(n){ne(this,a),w(this,"buckets",new Map),w(this,"keyDepth",new Map);var e=U(n),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;this.buckets.set(r,new Set)}}catch(i){e.e(i)}finally{e.f()}}return ae(a,[{key:"has",value:function(e){return this.keyDepth.has(e)}},{key:"getBucket",value:function(e){return this.buckets.get(e)}},{key:"set",value:function(e,t){var r,i=this.keyDepth.get(e);if(i!==t){var o=this.buckets.get(t);if(!o)throw new Error('Sigma: "'.concat(t,'" is not a declared depth layer'));i!==void 0&&((r=this.buckets.get(i))===null||r===void 0||r.delete(e)),o.add(e),this.keyDepth.set(e,t)}}},{key:"remove",value:function(e){var t=this.keyDepth.get(e);t!==void 0&&(this.buckets.get(t).delete(e),this.keyDepth.delete(e))}},{key:"clearAll",value:function(){var e=U(this.buckets.values()),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;r.clear()}}catch(i){e.e(i)}finally{e.f()}this.keyDepth.clear()}},{key:"getSorted",value:function(e,t){var r=this.buckets.get(e);return!r||r.size===0?[]:Z(r).sort(function(i,o){return t(i)-t(o)})}}])})(),Bo=(function(a){function n(e,t){return ne(this,n),ye(this,n,[e,2,t])}return _e(n,a),ae(n,[{key:"updateNode",value:function(t,r,i,o,s,l,u,d){var h=this.indexMap.get(t);if(h===void 0)throw new Error('Node "'.concat(t,'" not allocated in NodeDataTexture'));var c=gt(d),f=le(c,4),g=f[0],b=f[1],m=f[2],v=f[3],p=h*2*4;this.data[p]=r,this.data[p+1]=i,this.data[p+2]=o,this.data[p+3]=s,this.data[p+4]=l,this.data[p+5]=u,this.data[p+6]=g*65536+b*256+m,this.data[p+7]=v/255,this.markDirty(h)}}])})(ya),xh=1024,Th=1.5,Eh=4096,_r=(function(){function a(n,e){var t;if(ne(this,a),w(this,"texture",null),w(this,"framebuffer",null),!n.getExtension("EXT_color_buffer_float"))throw new Error("sigma: EXT_color_buffer_float is required for label/edge placement but is unavailable in this WebGL2 context.");this.gl=n,this.channels=e.channels,this.capacity=this.roundUpToPowerOfTwo((t=e.initialCapacity)!==null&&t!==void 0?t:xh);var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.create()}return ae(a,[{key:"roundUpToPowerOfTwo",value:function(e){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,e))))}},{key:"computeDimensions",value:function(e){var t=Math.min(e,Eh),r=Math.ceil(e/t);return{width:t,height:r}}},{key:"create",value:function(){var e=this.gl,t=this.channels===1?e.R32F:e.RGBA32F,r=this.channels===1?e.RED:e.RGBA;if(this.texture=e.createTexture(),e.activeTexture(e.TEXTURE0),e.bindTexture(e.TEXTURE_2D,this.texture),e.texImage2D(e.TEXTURE_2D,0,t,this.textureWidth,this.textureHeight,0,r,e.FLOAT,null),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE),this.framebuffer=e.createFramebuffer(),e.bindFramebuffer(e.FRAMEBUFFER,this.framebuffer),e.framebufferTexture2D(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,this.texture,0),e.checkFramebufferStatus(e.FRAMEBUFFER)!==e.FRAMEBUFFER_COMPLETE)throw new Error("sigma: float framebuffer for frame-pass placement is incomplete.");e.bindFramebuffer(e.FRAMEBUFFER,null),e.bindTexture(e.TEXTURE_2D,null)}},{key:"ensureCapacity",value:function(e){if(!(e<=this.capacity)){var t=this.gl;this.capacity=this.roundUpToPowerOfTwo(Math.ceil(e*Th));var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.texture&&t.deleteTexture(this.texture),this.framebuffer&&t.deleteFramebuffer(this.framebuffer),this.create()}}},{key:"bindAsRenderTarget",value:function(){var e=this.gl;e.bindFramebuffer(e.FRAMEBUFFER,this.framebuffer),e.viewport(0,0,this.textureWidth,this.textureHeight)}},{key:"bind",value:function(e){var t=this.gl;t.activeTexture(t.TEXTURE0+e),t.bindTexture(t.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"restore",value:function(){this.gl.getExtension("EXT_color_buffer_float"),this.create()}},{key:"kill",value:function(){var e=this.gl;this.texture&&(e.deleteTexture(this.texture),this.texture=null),this.framebuffer&&(e.deleteFramebuffer(this.framebuffer),this.framebuffer=null)}}])})(),Uo=(function(a){function n(e,t){return ne(this,n),ye(this,n,[e,2,t])}return _e(n,a),ae(n,[{key:"updateEdge",value:function(t,r,i,o,s,l){var u=arguments.length>6&&arguments[6]!==void 0?arguments[6]:0,d=arguments.length>7&&arguments[7]!==void 0?arguments[7]:0,h=arguments.length>8&&arguments[8]!==void 0?arguments[8]:0,c=this.indexMap.get(t);if(c===void 0)throw new Error('Edge "'.concat(t,'" not allocated in EdgeDataTexture'));var f=c*this.TEXELS_PER_ITEM*4;this.data[f+0]=r,this.data[f+1]=i,this.data[f+2]=o,this.data[f+3]=0,this.data[f+4]=s,this.data[f+5]=l,this.data[f+6]=u,this.data[f+7]=(d&15)<<4|h&15,this.markDirty(c)}}])})(ya);var Sf=lt(sn()),Af=lt(hn());function Sh(a,n){if(a==null)return{};var e={};for(var t in a)if({}.hasOwnProperty.call(a,t)){if(n.indexOf(t)!==-1)continue;e[t]=a[t]}return e}function Oo(a,n){if(a==null)return{};var e,t,r=Sh(a,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(a);for(t=0;t<i.length;t++)e=i[t],n.indexOf(e)===-1&&{}.propertyIsEnumerable.call(a,e)&&(r[e]=a[e])}return r}var Ah=["factor"],Dh=["factor"],Ho=1.5,xr={x:.5,y:.5,angle:0,ratio:1},$o=(function(a){function n(){var e;ne(this,n);for(var t=arguments.length,r=new Array(t),i=0;i<t;i++)r[i]=arguments[i];return e=ye(this,n,[].concat(r)),w(e,"minRatio",null),w(e,"maxRatio",null),w(e,"enabled",!0),w(e,"enabledZooming",!0),w(e,"enabledPanning",!0),w(e,"enabledRotation",!0),w(e,"constrainState",null),w(e,"state",O({},xr)),w(e,"previousState",O({},xr)),w(e,"nextFrame",null),w(e,"currentAnimationFrom",null),w(e,"currentAnimationTo",null),w(e,"animationCallback",null),e}return _e(n,a),ae(n,[{key:"x",get:function(){return this.state.x}},{key:"y",get:function(){return this.state.y}},{key:"angle",get:function(){return this.state.angle}},{key:"ratio",get:function(){return this.state.ratio}},{key:"getState",value:function(){return O({},this.state)}},{key:"getPreviousState",value:function(){return O({},this.previousState)}},{key:"getBoundedRatio",value:function(t){var r=t;return typeof this.minRatio=="number"&&(r=Math.max(r,this.minRatio)),typeof this.maxRatio=="number"&&(r=Math.min(r,this.maxRatio)),r}},{key:"validateState",value:function(t){var r=this.getState();return this.enabledPanning&&typeof t.x=="number"&&(r.x=t.x),this.enabledPanning&&typeof t.y=="number"&&(r.y=t.y),this.enabledZooming&&typeof t.ratio=="number"&&(r.ratio=this.getBoundedRatio(t.ratio)),this.enabledRotation&&typeof t.angle=="number"&&(r.angle=t.angle),this.constrainState?this.constrainState(r):r}},{key:"isAnimating",value:function(){return this.nextFrame!==null}},{key:"setState",value:function(t){if(!this.enabled)return this;var r=this.validateState(t);return qi(this.state,r)?this:(this.previousState=this.state,this.state=r,this.emit("updated",this.getState()),this)}},{key:"updateState",value:function(t){return this.setState(t(this.getState())),this}},{key:"animate",value:function(t,r){var i=this;return this.enabled?new Promise(function(o){return i.runAnimation(t,O(O({},Yi),r),o)}):Promise.resolve()}},{key:"cancelAnimation",value:function(){return this.nextFrame!==null&&(cancelAnimationFrame(this.nextFrame),this.nextFrame=null),this.resolveAnimation(),this.emitAnimationEnd(!1),this}},{key:"zoomIn",value:function(){var t=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=t.factor,i=r===void 0?Ho:r,o=Oo(t,Ah);return this.animate({ratio:this.ratio/i},o)}},{key:"zoomOut",value:function(){var t=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=t.factor,i=r===void 0?Ho:r,o=Oo(t,Dh);return this.animate({ratio:this.ratio*i},o)}},{key:"reset",value:function(t){return this.animate(xr,t)}},{key:"runAnimation",value:function(t,r,i){var o=this,s=na(r.easing),l=Date.now(),u=this.getState(),d=this.validateState(t),h=function(){if(!o.enabled){o.cancelAnimation();return}var f=r.duration>0?(Date.now()-l)/r.duration:1;if(f>=1){o.nextFrame=null,o.setState(d),o.resolveAnimation(),o.emitAnimationEnd(!0);return}var g=s(f);o.setState({x:u.x+(d.x-u.x)*g,y:u.y+(d.y-u.y)*g,angle:u.angle+(d.angle-u.angle)*g,ratio:u.ratio+(d.ratio-u.ratio)*g}),o.nextFrame=requestAnimationFrame(h)};this.cancelAnimation(),this.currentAnimationFrom=u,this.currentAnimationTo=d,this.animationCallback=i,this.emit("animationStart",{from:u,to:d}),h()}},{key:"resolveAnimation",value:function(){var t=this.animationCallback;this.animationCallback=null,t&&t()}},{key:"emitAnimationEnd",value:function(t){var r=this.currentAnimationFrom,i=this.currentAnimationTo;!r||!i||(this.currentAnimationFrom=null,this.currentAnimationTo=null,this.emit("animationEnd",{from:r,to:i,completed:t}))}}],[{key:"from",value:function(t){var r=new n;return r.setState(t)}}])})(oa);function Me(a,n){var e=n.getBoundingClientRect();return{x:a.clientX-e.left,y:a.clientY-e.top}}function Je(a,n){var e=O(O({},Me(a,n)),{},{sigmaDefaultPrevented:!1,preventSigmaDefault:function(){e.sigmaDefaultPrevented=!0},original:a});return e}function Ct(a){var n="x"in a?a:O(O({},a.touches[0]||a.previousTouches[0]),{},{original:a.original,sigmaDefaultPrevented:a.sigmaDefaultPrevented,preventSigmaDefault:function(){a.sigmaDefaultPrevented=!0,n.sigmaDefaultPrevented=!0}});return n}function wh(a,n){var e=Je(a,n);return e.delta=is(a),e}var Rh=2;function Da(a){for(var n=[],e=0,t=Math.min(a.length,Rh);e<t;e++)n.push(a[e]);return n}function Dn(a,n,e){var t={touches:Da(a.touches).map(function(r){return Me(r,e)}),previousTouches:n.map(function(r){return Me(r,e)}),sigmaDefaultPrevented:!1,preventSigmaDefault:function(){t.sigmaDefaultPrevented=!0},original:a};return t}function is(a){if(typeof a.deltaY<"u")return a.deltaY*-3/360;if(typeof a.detail<"u")return a.detail/-9;throw new Error("Captor: could not extract delta from event.")}var os=(function(a){function n(e,t){var r;return ne(this,n),r=ye(this,n),r.container=e,r.renderer=t,r}return _e(n,a),ae(n)})(oa),Ch=["doubleClickTimeout","doubleClickZoomingDuration","doubleClickZoomingRatio","dragTimeout","draggedEventsTolerance","enableCameraMouseRotation","gestureTarget","inertiaDuration","inertiaRatio","zoomDuration","zoomingRatio"],Lh=Ch.reduce(function(a,n){return O(O({},a),{},w({},n,ua[n]))},{});function wn(a){return a instanceof PointerEvent&&a.pointerType==="touch"}var kh=(function(a){function n(e,t){var r;return ne(this,n),r=ye(this,n,[e,t]),w(r,"enabled",!0),w(r,"draggedEvents",0),w(r,"downStartTime",null),w(r,"lastMouseX",null),w(r,"lastMouseY",null),w(r,"isMouseDown",!1),w(r,"isMoving",!1),w(r,"isPanningStage",!1),w(r,"movingTimeout",null),w(r,"startCameraState",null),w(r,"clicks",0),w(r,"doubleClickTimeout",null),w(r,"isRightMouseDown",!1),w(r,"startRotationAngle",null),w(r,"startCameraAngle",null),w(r,"currentWheelDirection",0),w(r,"lastWheelAnimationId",0),w(r,"settings",Lh),r.handleRightClick=r.handleRightClick.bind(r),r.handleDown=r.handleDown.bind(r),r.handleUp=r.handleUp.bind(r),r.handleMove=r.handleMove.bind(r),r.handleWheel=r.handleWheel.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleEnter=r.handleEnter.bind(r),e.addEventListener("contextmenu",r.handleRightClick,{capture:!1}),e.addEventListener("pointerdown",r.handleDown,{capture:!1}),e.addEventListener("wheel",r.handleWheel,{capture:!1,passive:!1}),e.addEventListener("pointerleave",r.handleLeave,{capture:!1}),e.addEventListener("pointerenter",r.handleEnter,{capture:!1}),document.addEventListener("pointermove",r.handleMove,{capture:!1}),document.addEventListener("pointerup",r.handleUp,{capture:!1}),r}return _e(n,a),ae(n,[{key:"kill",value:function(){var t=this.container;t.removeEventListener("contextmenu",this.handleRightClick),t.removeEventListener("pointerdown",this.handleDown),t.removeEventListener("wheel",this.handleWheel),t.removeEventListener("pointerleave",this.handleLeave),t.removeEventListener("pointerenter",this.handleEnter),document.removeEventListener("pointermove",this.handleMove),document.removeEventListener("pointerup",this.handleUp)}},{key:"handleClick",value:function(t){var r=this;if(this.enabled){if(this.clicks++,this.clicks===2)return this.clicks=0,typeof this.doubleClickTimeout=="number"&&(clearTimeout(this.doubleClickTimeout),this.doubleClickTimeout=null),this.handleDoubleClick(t);setTimeout(function(){r.clicks=0,r.doubleClickTimeout=null},this.settings.doubleClickTimeout),this.draggedEvents<this.settings.draggedEventsTolerance&&this.emit("click",Je(t,this.container))}}},{key:"handleRightClick",value:function(t){this.enabled&&(this.settings.enableCameraMouseRotation&&t.preventDefault(),this.emit("rightClick",Je(t,this.container)))}},{key:"handleDoubleClick",value:function(t){if(this.enabled){t.preventDefault(),t.stopPropagation();var r=Je(t,this.container);if(this.emit("doubleClick",r),!r.sigmaDefaultPrevented){var i=this.renderer.getCamera(),o=i.getBoundedRatio(i.getState().ratio/this.settings.doubleClickZoomingRatio);i.animate(this.renderer.getViewportZoomedState(Me(t,this.container),o),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}}},{key:"handleDown",value:function(t){if(!(!this.enabled||wn(t))){if(t.button===0){this.startCameraState=this.renderer.getCamera().getState();var r=Me(t,this.container),i=r.x,o=r.y;this.lastMouseX=i,this.lastMouseY=o,this.draggedEvents=0,this.downStartTime=Date.now(),this.isMouseDown=!0}if(t.button===2&&this.settings.enableCameraMouseRotation){var s=Me(t,this.container),l=s.x,u=s.y,d=this.container.offsetWidth/2,h=this.container.offsetHeight/2;this.startRotationAngle=Math.atan2(u-h,l-d),this.startCameraAngle=this.renderer.getCamera().getState().angle,this.isRightMouseDown=!0}this.emit("mousedown",Je(t,this.container))}}},{key:"handleUp",value:function(t){var r=this;if(!(!this.enabled||wn(t)||!this.isMouseDown&&!this.isRightMouseDown)){if(this.isRightMouseDown){this.isRightMouseDown=!1,this.startRotationAngle=null,this.startCameraAngle=null,this.emit("mouseup",Je(t,this.container));return}var i=this.renderer.getCamera();this.isMouseDown=!1,this.isPanningStage&&(this.isPanningStage=!1,this.renderer._setPanning(!1)),typeof this.movingTimeout=="number"&&(clearTimeout(this.movingTimeout),this.movingTimeout=null);var o=Me(t,this.container),s=o.x,l=o.y,u=i.getState(),d=i.getPreviousState();this.isMoving?i.animate({x:u.x+this.settings.inertiaRatio*(u.x-d.x),y:u.y+this.settings.inertiaRatio*(u.y-d.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"}):(this.lastMouseX!==s||this.lastMouseY!==l)&&i.setState({x:u.x,y:u.y}),this.isMoving=!1,setTimeout(function(){var h=r.draggedEvents>0;r.draggedEvents=0;var c=r.renderer.getSetting("hideEdgesOnMove")||r.renderer.getSetting("hideLabelsOnMove");h&&c&&r.renderer.refresh()},0),this.emit("mouseup",Je(t,this.container)),(t.target===this.container||t.composedPath()[0]===this.container)&&this.handleClick(t)}}},{key:"handleMove",value:function(t){var r=this;if(!(!this.enabled||wn(t))){var i=Je(t,this.container);if(this.emit("mousemovebody",i),(t.target===this.container||t.composedPath()[0]===this.container)&&this.emit("mousemove",i),this.isMouseDown&&this.draggedEvents++,!i.sigmaDefaultPrevented){if(this.isMouseDown){this.isMoving=!0,typeof this.movingTimeout=="number"&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.movingTimeout=null,r.isMoving=!1},this.settings.dragTimeout);var o=this.renderer.getCamera(),s=Me(t,this.container),l=s.x,u=s.y,d=this.renderer.viewportToFramedGraph({x:this.lastMouseX,y:this.lastMouseY}),h=this.renderer.viewportToFramedGraph({x:l,y:u}),c=d.x-h.x,f=d.y-h.y,g=o.getState(),b=g.x+c,m=g.y+f;this.isPanningStage||(this.isPanningStage=!0,this.renderer._setPanning(!0)),o.setState({x:b,y:m}),this.lastMouseX=l,this.lastMouseY=u,t.preventDefault(),t.stopPropagation()}if(this.isRightMouseDown){var v=Me(t,this.container),p=v.x,y=v.y,T=this.container.offsetWidth/2,_=this.container.offsetHeight/2,S=Math.atan2(y-_,p-T),D=S-this.startRotationAngle,A=this.renderer.getCamera();A.setState({angle:this.startCameraAngle+D}),t.preventDefault(),t.stopPropagation()}}}}},{key:"handleLeave",value:function(t){wn(t)||this.emit("mouseleave",Je(t,this.container))}},{key:"handleEnter",value:function(t){wn(t)||this.emit("mouseenter",Je(t,this.container))}},{key:"handleWheel",value:function(t){var r=this,i=this.renderer.getCamera();if(!(!this.enabled||!i.enabledZooming)){var o=wh(t,this.container);if(this.emit("wheel",o),!o.sigmaDefaultPrevented){var s=this.settings.gestureTarget;if(s!=="page"){if(s==="shared"&&!t.ctrlKey&&!t.metaKey){this.renderer._showGestureHint("wheel");return}var l=is(t);if(l){t.preventDefault(),t.stopPropagation();var u=i.getState().ratio,d=l>0?1/this.settings.zoomingRatio:this.settings.zoomingRatio,h=i.getBoundedRatio(u*d),c=l>0?1:-1,f=Date.now();if(u!==h&&!(this.currentWheelDirection===c&&this.lastWheelTriggerTime&&f-this.lastWheelTriggerTime<this.settings.zoomDuration/5)){var g=++this.lastWheelAnimationId;i.animate(this.renderer.getViewportZoomedState(Me(t,this.container),h),{easing:"quadraticOut",duration:this.settings.zoomDuration}).then(function(){r.lastWheelAnimationId===g&&(r.currentWheelDirection=0)}),this.currentWheelDirection=c,this.lastWheelTriggerTime=f}}}}}}},{key:"setSettings",value:function(t){this.settings=t}}])})(os),Ph=["dragTimeout","gestureTarget","inertiaDuration","inertiaRatio","doubleClickTimeout","doubleClickZoomingRatio","doubleClickZoomingDuration","tapMoveTolerance"],Fh=Ph.reduce(function(a,n){return O(O({},a),{},w({},n,ua[n]))},{}),Ih=(function(a){function n(e,t){var r;return ne(this,n),r=ye(this,n,[e,t]),w(r,"enabled",!0),w(r,"isMoving",!1),w(r,"hasMoved",!1),w(r,"isPanningStage",!1),w(r,"isZoomingStage",!1),w(r,"touchMode",0),w(r,"startTouchesPositions",[]),w(r,"lastTouches",[]),w(r,"lastTap",null),w(r,"settings",Fh),r.handleStart=r.handleStart.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleMove=r.handleMove.bind(r),e.addEventListener("touchstart",r.handleStart,{capture:!1}),e.addEventListener("touchcancel",r.handleLeave,{capture:!1}),document.addEventListener("touchend",r.handleLeave,{capture:!1,passive:!1}),document.addEventListener("touchmove",r.handleMove,{capture:!1,passive:!1}),r}return _e(n,a),ae(n,[{key:"kill",value:function(){var t=this.container;t.removeEventListener("touchstart",this.handleStart),t.removeEventListener("touchcancel",this.handleLeave),document.removeEventListener("touchend",this.handleLeave),document.removeEventListener("touchmove",this.handleMove)}},{key:"getDimensions",value:function(){return{width:this.container.offsetWidth,height:this.container.offsetHeight}}},{key:"doesCapture",value:function(t){var r=this.settings.gestureTarget;return r==="graph"?!0:r==="page"?!1:t>=2}},{key:"syncStageFlags",value:function(){var t=this.touchMode===1&&this.hasMoved,r=this.touchMode===2;t!==this.isPanningStage&&(this.isPanningStage=t,this.renderer._setPanning(t)),r!==this.isZoomingStage&&(this.isZoomingStage=r,this.renderer._setZooming(r))}},{key:"handleStart",value:function(t){var r=this;if(this.enabled){var i=Da(t.touches);if(this.touchMode=i.length,this.startCameraState=this.renderer.getCamera().getState(),this.startTouchesPositions=i.map(function(f){return Me(f,r.container)}),this.touchMode===2){var o=le(this.startTouchesPositions,2),s=o[0],l=s.x,u=s.y,d=o[1],h=d.x,c=d.y;this.startTouchesAngle=Math.atan2(c-u,h-l),this.startTouchesDistance=Math.sqrt(Math.pow(h-l,2)+Math.pow(c-u,2))}this.syncStageFlags(),this.emit("touchdown",Dn(t,this.lastTouches,this.container)),this.lastTouches=i,this.lastTouchesPositions=this.startTouchesPositions,t.cancelable&&(this.doesCapture(t.touches.length)||this.renderer._hasNodeDrag())&&t.preventDefault()}}},{key:"handleLeave",value:function(t){if(!(!this.enabled||!this.startTouchesPositions.length)){switch(t.cancelable&&this.doesCapture(this.touchMode)&&t.preventDefault(),this.movingTimeout&&(this.isMoving=!1,clearTimeout(this.movingTimeout)),this.touchMode){case 2:if(t.touches.length===1){this.handleStart(t);break}case 1:if(this.isMoving){var r=this.renderer.getCamera(),i=r.getState(),o=r.getPreviousState();r.animate({x:i.x+this.settings.inertiaRatio*(i.x-o.x),y:i.y+this.settings.inertiaRatio*(i.y-o.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"})}this.hasMoved=!1,this.isMoving=!1,this.touchMode=0;break}if(this.syncStageFlags(),this.emit("touchup",Dn(t,this.lastTouches,this.container)),!t.touches.length){var s=Me(this.lastTouches[0],this.container),l=this.startTouchesPositions[0],u=Math.pow(s.x-l.x,2)+Math.pow(s.y-l.y,2);if(!t.touches.length&&u<Math.pow(this.settings.tapMoveTolerance,2))if(this.lastTap&&Date.now()-this.lastTap.time<this.settings.doubleClickTimeout){var d=Dn(t,this.lastTouches,this.container);if(this.emit("doubletap",d),this.lastTap=null,!d.sigmaDefaultPrevented&&this.settings.gestureTarget!=="page"){var h=this.renderer.getCamera(),c=h.getBoundedRatio(h.getState().ratio/this.settings.doubleClickZoomingRatio);h.animate(this.renderer.getViewportZoomedState(s,c),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}else{var f=Dn(t,this.lastTouches,this.container);this.emit("tap",f),this.lastTap={time:Date.now(),position:f.touches[0]||f.previousTouches[0]}}}this.lastTouches=Da(t.touches),this.startTouchesPositions=[]}}},{key:"handleMove",value:function(t){var r=this;if(!(!this.enabled||!this.startTouchesPositions.length)){var i=this.doesCapture(t.touches.length);i&&t.preventDefault();var o=Da(t.touches),s=o.map(function(R){return Me(R,r.container)}),l=this.lastTouches;this.lastTouches=o,this.lastTouchesPositions=s;var u=Dn(t,l,this.container);if(this.emit("touchmove",u),!u.sigmaDefaultPrevented&&(this.hasMoved||(this.hasMoved=s.some(function(R,j){var H=r.startTouchesPositions[j];return H&&(R.x!==H.x||R.y!==H.y)})),!!this.hasMoved)){if(!i){this.settings.gestureTarget==="shared"&&this.renderer._showGestureHint("touch");return}this.isMoving=!0,this.syncStageFlags(),this.movingTimeout&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.isMoving=!1},this.settings.dragTimeout);var d=this.renderer.getCamera(),h=this.startCameraState,c=this.renderer.getSetting("stagePadding");switch(this.touchMode){case 1:{var f=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0]),g=f.x,b=f.y,m=this.renderer.viewportToFramedGraph(s[0]),v=m.x,p=m.y;d.setState({x:h.x+g-v,y:h.y+b-p});break}case 2:{var y={x:.5,y:.5,angle:0,ratio:1},T=s[0],_=T.x,S=T.y,D=s[1],A=D.x,E=D.y,L=Math.atan2(E-S,A-_)-this.startTouchesAngle,z=Math.hypot(E-S,A-_)/this.startTouchesDistance,C=d.getBoundedRatio(h.ratio/z);y.ratio=C,y.angle=h.angle+L;var N=this.getDimensions(),k=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0],{cameraState:h}),F=Math.min(N.width,N.height)-2*c,M=F/N.width,I=F/N.height,x=C/F,P=_-F/2/M,W=S-F/2/I,G=Ba({x:P,y:W},y.angle);P=G.x,W=G.y,y.x=k.x-P*x,y.y=k.y+W*x,d.setState(y);break}}}}}},{key:"setSettings",value:function(t){this.settings=t}}])})(os),Gh=(function(){function a(n,e,t,r){ne(this,a),w(this,"pendingNode",null),w(this,"session",null),this.graph=n,this.viewportToGraph=e,this.setNodesState=t,this.emit=r}return ae(a,[{key:"start",value:function(e,t,r,i,o){var s=r(e),l=!1;if(this.emit("nodeDragStart",{node:e,allDraggedNodes:s,event:t,preventSigmaDefault:function(){l=!0}}),l)return!1;var u=new Map,d=U(s),h;try{for(d.s();!(h=d.n()).done;){var c=h.value;u.set(c,{x:this.graph.getNodeAttribute(c,i),y:this.graph.getNodeAttribute(c,o)})}}catch(f){d.e(f)}finally{d.f()}return this.session={node:e,allNodes:s,startPosition:this.viewportToGraph(t),startNodePositions:u,xAttr:i,yAttr:o},this.setNodesState(s,{isDragged:!0}),!0}},{key:"applyMove",value:function(e,t){var r=this.session,i=r.allNodes,o=r.startNodePositions,s=r.startPosition,l=r.xAttr,u=r.yAttr,d=this.viewportToGraph(e),h={x:d.x-s.x,y:d.y-s.y},c=U(i),f;try{for(c.s();!(f=c.n()).done;){var g=f.value,b=o.get(g);if(!(!b||!this.graph.hasNode(g))){var m={x:b.x+h.x,y:b.y+h.y};t?this.graph.mergeNodeAttributes(g,t(m,g)):(this.graph.setNodeAttribute(g,l,m.x),this.graph.setNodeAttribute(g,u,m.y))}}}catch(v){c.e(v)}finally{c.f()}}},{key:"end",value:function(){if(this.pendingNode=null,!this.session)return null;var e=this.session,t=e.node,r=e.allNodes;return this.setNodesState(r,{isDragged:!1}),this.session=null,{node:t,allNodes:r}}},{key:"removeNode",value:function(e){e===this.pendingNode&&(this.pendingNode=null),this.session&&(this.session.node===e?(this.setNodesState(this.session.allNodes,{isDragged:!1}),this.session=null):this.session.allNodes.includes(e)&&(this.session.allNodes=this.session.allNodes.filter(function(t){return t!==e}),this.session.startNodePositions.delete(e)))}},{key:"clear",value:function(){this.pendingNode=null,this.session=null}}])})();function Vo(a,n){return a<n?"".concat(a,"\0").concat(n):"".concat(n,"\0").concat(a)}var zh=(function(){function a(n,e){ne(this,a),w(this,"groups",new Map),w(this,"edgeToGroupKey",new Map),this.graph=n,this.onGroupChanged=e}return ae(a,[{key:"getGroupKey",value:function(e){return Vo(this.graph.source(e),this.graph.target(e))}},{key:"sortGroup",value:function(e,t){var r=this,i=t.split("\0")[0];e.sort(function(o,s){var l=r.graph.source(o)===i||!r.graph.isDirected(o)?0:1,u=r.graph.source(s)===i||!r.graph.isDirected(s)?0:1;return l-u})}},{key:"register",value:function(e){var t=this.getGroupKey(e),r=this.groups.get(t);r||(r=[],this.groups.set(t,r)),r.includes(e)||r.push(e),this.edgeToGroupKey.set(e,t),r.length>1&&this.sortGroup(r,t),this.onGroupChanged(r,r.length)}},{key:"unregister",value:function(e){var t=this.edgeToGroupKey.get(e);if(t){this.edgeToGroupKey.delete(e);var r=this.groups.get(t);if(r){var i=r.indexOf(e);i!==-1&&r.splice(i,1),r.length===0?this.groups.delete(t):this.onGroupChanged(r,r.length)}}}},{key:"getGroup",value:function(e){var t,r=this.edgeToGroupKey.get(e);return r?(t=this.groups.get(r))!==null&&t!==void 0?t:[]:[]}},{key:"getSiblings",value:function(e){return this.getGroup(e).filter(function(t){return t!==e})}},{key:"rebuild",value:function(){var e=this;this.groups.clear(),this.edgeToGroupKey.clear(),this.graph.forEachEdge(function(l,u,d,h){var c=Vo(d,h),f=e.groups.get(c);f||(f=[],e.groups.set(c,f)),f.push(l),e.edgeToGroupKey.set(l,c)});var t=U(this.groups),r;try{for(t.s();!(r=t.n()).done;){var i=le(r.value,2),o=i[0],s=i[1];s.length>1&&this.sortGroup(s,o),this.onGroupChanged(s,s.length)}}catch(l){t.e(l)}finally{t.f()}}},{key:"clear",value:function(){this.groups.clear(),this.edgeToGroupKey.clear()}}])})();function jo(a,n){var e;if(a===!1||a==="extend"||a==="separate")return a;var t=a[n];return t!==void 0?t:(e=a.default)!==null&&e!==void 0?e:!1}function Xo(a){return a===!1?!1:a==="extend"||a==="separate"?!0:Object.values(a).some(function(n){return n==="extend"||n==="separate"})}function qo(a){return a==="separate"?!0:a===!1||a==="extend"?!1:a.default==="separate"?!0:Object.values(a).includes("separate")}var Pe={node:{parent:null,eventSuffix:"Node",payloadKey:"node",isEnabled:function(){return!0},writesPickingThisFrame:function(){return!0},setHover:function(n,e,t){return n.setNodeState(e,{isHovered:t})},getCursor:function(n,e){var t;return(t=n.nodeDataCache[e])===null||t===void 0?void 0:t.cursor},isHitValid:function(n,e){var t;return((t=n.nodeDataCache[e])===null||t===void 0?void 0:t.visibility)!=="hidden"}},edge:{parent:null,eventSuffix:"Edge",payloadKey:"edge",isEnabled:function(n){return!!n.settings.enableEdgeEvents},writesPickingThisFrame:function(n){return!!n.settings.enableEdgeEvents},setHover:function(n,e,t){return n.setEdgeState(e,{isHovered:t})},getCursor:function(n,e){var t;return(t=n.edgeDataCache[e])===null||t===void 0?void 0:t.cursor}},nodeLabel:{parent:"node",eventSuffix:"NodeLabel",payloadKey:"node",isEnabled:function(n){return qo(n.settings.nodeLabelEvents)},writesPickingThisFrame:function(n){return Xo(n.settings.nodeLabelEvents)},setHover:function(n,e,t){return n.setNodeState(e,{isLabelHovered:t})},getCursor:function(n,e){var t;return(t=n.nodeDataCache[e])===null||t===void 0?void 0:t.labelCursor},resolveForVerb:function(n,e,t){var r=jo(t.settings.nodeLabelEvents,e);return r===!1?null:r==="extend"?{kind:"node",key:n.key}:n}},edgeLabel:{parent:"edge",eventSuffix:"EdgeLabel",payloadKey:"edge",isEnabled:function(n){return qo(n.settings.edgeLabelEvents)},writesPickingThisFrame:function(n){return Xo(n.settings.edgeLabelEvents)},setHover:function(n,e,t){return n.setEdgeState(e,{isLabelHovered:t})},getCursor:function(n,e){var t;return(t=n.edgeDataCache[e])===null||t===void 0?void 0:t.labelCursor},resolveForVerb:function(n,e,t){var r=jo(t.settings.edgeLabelEvents,e);return r===!1?null:r==="extend"?{kind:"edge",key:n.key}:n}}},Pt=["node","edge","nodeLabel","edgeLabel"];function Nh(){return{lookup:[null],idsByKind:{node:new Map,edge:new Map,nodeLabel:new Map,edgeLabel:new Map},nextId:1}}function kt(a,n,e){var t;return(t=a.idsByKind[n].get(e))!==null&&t!==void 0?t:0}function Tr(a,n){var e=U(Pt),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;if(r===n||Pe[r].parent===n){var i=U(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(c){i.e(c)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(c){e.e(c)}finally{e.f()}var l=1,u=U(Pt),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;if(h===n)break;l+=a.idsByKind[h].size}}catch(c){u.e(c)}finally{u.f()}a.nextId=l}function Ko(a,n,e){var t=a.nextId++;return a.idsByKind[n].set(e,t),a.lookup[t]={kind:n,key:e},t}function Mh(a,n){var e=U(Pt),t;try{for(e.s();!(t=e.n()).done;){var r=t.value;if(Pe[r].parent!==null){var i=U(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(_){i.e(_)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(_){e.e(_)}finally{e.f()}a.nextId=1;var l=U(Pt),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;if(Pe[d].parent===null){var h=U(a.idsByKind[d].values()),c;try{for(h.s();!(c=h.n()).done;){var f=c.value;f>=a.nextId&&(a.nextId=f+1)}}catch(_){h.e(_)}finally{h.f()}}}}catch(_){l.e(_)}finally{l.f()}var g=U(Pt),b;try{for(g.s();!(b=g.n()).done;){var m=b.value,v=Pe[m];if(!(v.parent===null||!v.isEnabled(n))){var p=U(a.idsByKind[v.parent].keys()),y;try{for(p.s();!(y=p.n()).done;){var T=y.value;a.idsByKind[m].set(T,a.nextId),a.lookup[a.nextId]={kind:m,key:T},a.nextId++}}catch(_){p.e(_)}finally{p.f()}}}}catch(_){g.e(_)}finally{g.f()}}function Wh(a,n){var e=U(Pt),t;try{for(e.s();!(t=e.n()).done;){var r=t.value,i=a[r],o=n[r];if(i!==o){if(i.size!==o.size)return!1;var s=U(i),l;try{for(s.s();!(l=s.n()).done;){var u=le(l.value,2),d=u[0],h=u[1];if(o.get(d)!==h)return!1}}catch(c){s.e(c)}finally{s.f()}}}}catch(c){e.e(c)}finally{e.f()}return!0}function Yo(a,n,e){Pe[n.kind].setHover(a,n.key,e)}function Bh(a,n){return Pe[n.kind].getCursor(a,n.key)}function Rn(a,n){return"".concat(n).concat(Pe[a].eventSuffix)}function Cn(a,n){return O(O({},n),{},w({},Pe[a.kind].payloadKey,a.key))}function Uh(a,n,e){var t=Pe[n.kind].resolveForVerb;return t?t(n,e,a):n}function Oh(a,n){var e=Pe[n.kind].isHitValid;return e?e(a,n.key):!0}function Er(a){return Pt.filter(function(n){return n===a||Pe[n].parent===a})}function ss(a,n,e){return n&&(n=Uh(a,n,e)),n&&!Oh(a,n)&&(n=null),n}function Sr(a,n,e){var t=e==="wheel"?a.stateManager.hovered:a.getHitAtPosition(n);return ss(a,t,e)}function Hh(a,n){return!a||!n?a===n:a.kind===n.kind&&a.key===n.key}function ls(a,n,e){var t={event:e,preventSigmaDefault:function(){return e.preventSigmaDefault()}},r=a.dragManager.session,i=r?{kind:"node",key:r.node}:ss(a,n,"enter"),o=a.stateManager,s=o.hovered;Hh(s,i)||(s&&(Yo(a,s,!1),a.emit(Rn(s.kind,"leave"),Cn(s,t))),o.setHovered(i),i&&(Yo(a,i,!0),a.emit(Rn(i.kind,"enter"),Cn(i,t))),a.updateContainerCursor())}function $h(a,n,e,t){t.handleResize=function(){return a.scheduleRefresh()},window.addEventListener("resize",t.handleResize),t.handleMove=function(i){a.hoverResolver.pointerMoved(Ct(i))},t.handleMoveBody=function(i){var o=Ct(i),s=a.dragManager;if(s.pendingNode&&!s.session){var l=a.nodeStyleAnalysis,u=l.xAttribute,d=l.yAttribute,h=a.settings;s.start(s.pendingNode,o,h.getDraggedNodes,u||"x",d||"y"),s.pendingNode=null}s.session&&(s.applyMove(o,a.settings.dragPositionToAttributes),a.emit("nodeDrag",{node:s.session.node,allDraggedNodes:s.session.allNodes,event:o}),o.preventSigmaDefault()),a.emit("moveBody",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})},t.handleLeave=function(i){var o=Ct(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}};a.hoverResolver.pointerLeft(),ls(a,null,o),a.emit("leaveStage",s)},t.handleEnter=function(i){var o=Ct(i);a.emit("enterStage",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})};var r=function(o){return function(s){var l=Ct(s),u={event:l,preventSigmaDefault:function(){return l.preventSigmaDefault()}},d=Sr(a,l,o);if(d){a.emit(Rn(d.kind,o),Cn(d,u));return}a.emit("".concat(o,"Stage"),u)}};t.handleClick=r("click"),t.handleRightClick=r("rightClick"),t.handleDoubleClick=r("doubleClick"),t.handleWheel=r("wheel"),t.handleDown=function(i){var o=Ct(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=Sr(a,o,"down");if(l){l.kind==="node"&&a.settings.enableNodeDrag&&(a.dragManager.pendingNode=l.key),a.emit(Rn(l.kind,"down"),Cn(l,s));return}a.emit("downStage",s)},t.handleUp=function(i){var o=Ct(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=a.dragManager.end();l&&a.emit("nodeDragEnd",O({node:l.node,allDraggedNodes:l.allNodes},s));var u=Sr(a,o,"up");if(u){a.emit(Rn(u.kind,"up"),Cn(u,s));return}a.emit("upStage",s)},n.on("mousemove",t.handleMove),n.on("mousemovebody",t.handleMoveBody),n.on("click",t.handleClick),n.on("rightClick",t.handleRightClick),n.on("doubleClick",t.handleDoubleClick),n.on("wheel",t.handleWheel),n.on("mousedown",t.handleDown),n.on("mouseup",t.handleUp),n.on("mouseleave",t.handleLeave),n.on("mouseenter",t.handleEnter),e.on("touchdown",t.handleDown),e.on("touchdown",t.handleMove),e.on("touchup",t.handleUp),e.on("touchmove",t.handleMove),e.on("tap",t.handleClick),e.on("doubletap",t.handleDoubleClick),e.on("touchmove",t.handleMoveBody)}function Vh(a,n){var e=a.graph,t=new Set(["x","y","zIndex","type"]);n.eachNodeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=!o||o.some(function(l){return t.has(l)});a.refresh({partialGraph:{nodes:e.nodes()},skipIndexation:!s,schedule:!0})},n.eachEdgeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=o&&["zIndex","type"].some(function(l){return o?.includes(l)});a.refresh({partialGraph:{edges:e.edges()},skipIndexation:!s,schedule:!0})},n.addNodeGraphUpdate=function(r){a.addNode(r.key),a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.updateNodeGraphUpdate=function(r){a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.dropNodeGraphUpdate=function(r){a.removeNode(r.key),a.refresh({schedule:!0})},n.addEdgeGraphUpdate=function(r){var i=r.key;a.edgeGroups.register(i),a.addEdge(i);var o=a.edgeGroups.getSiblings(i),s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;a.addEdge(u)}}catch(d){s.e(d)}finally{s.f()}a.refresh({partialGraph:{edges:[i].concat(Z(o))},schedule:!0})},n.updateEdgeGraphUpdate=function(r){a.refresh({partialGraph:{edges:[r.key]},skipIndexation:!1,schedule:!0})},n.dropEdgeGraphUpdate=function(r){var i=r.key,o=a.edgeGroups.getSiblings(i);a.edgeGroups.unregister(i),a.removeEdge(i);var s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;a.addEdge(u)}}catch(d){s.e(d)}finally{s.f()}a.refresh({schedule:!0})},n.clearEdgesGraphUpdate=function(){a.clearEdgeState(),a.clearEdgeIndices(),a.refresh({schedule:!0})},n.clearGraphUpdate=function(){a.clearEdgeState(),a.clearNodeState(),a.clearEdgeIndices(),a.clearNodeIndices(),a.refresh({schedule:!0})},e.on("nodeAdded",n.addNodeGraphUpdate),e.on("nodeDropped",n.dropNodeGraphUpdate),e.on("nodeAttributesUpdated",n.updateNodeGraphUpdate),e.on("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),e.on("edgeAdded",n.addEdgeGraphUpdate),e.on("edgeDropped",n.dropEdgeGraphUpdate),e.on("edgeAttributesUpdated",n.updateEdgeGraphUpdate),e.on("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),e.on("edgesCleared",n.clearEdgesGraphUpdate),e.on("cleared",n.clearGraphUpdate)}function jh(a,n){a.removeListener("nodeAdded",n.addNodeGraphUpdate),a.removeListener("nodeDropped",n.dropNodeGraphUpdate),a.removeListener("nodeAttributesUpdated",n.updateNodeGraphUpdate),a.removeListener("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),a.removeListener("edgeAdded",n.addEdgeGraphUpdate),a.removeListener("edgeDropped",n.dropEdgeGraphUpdate),a.removeListener("edgeAttributesUpdated",n.updateEdgeGraphUpdate),a.removeListener("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),a.removeListener("edgesCleared",n.clearEdgesGraphUpdate),a.removeListener("cleared",n.clearGraphUpdate)}var Xh={x:.5,y:.5,angle:0,ratio:1},qh=8,Kh=.001;function Yh(a){var n=a.coords,e=a.nodeData,t=a.dimensions,r=a.stagePadding,i=a.zoomToSizeRatioFunction,o=a.itemSizesReference,s=a.fitLabels,l=a.nodeLabelBox,u=Object.keys(n);if(!u.length)return a.extent;for(var d=i(1)||1,h=o==="positions",c=t.width,f=t.height,g=a.extent,b=0;b<qh;b++){for(var m=la(g),v={width:g.x[1]-g.x[0]||1,height:g.y[1]-g.y[0]||1},p=ft(Xh,t,v,r),y=Zo(p,m({x:0,y:0}),c,f),T=Zo(p,m({x:1,y:0}),c,f),_=Math.hypot(T.x-y.x,T.y-y.y)||1,S=1/0,D=-1/0,A=1/0,E=-1/0,L=0,z=u.length;L<z;L++){var C=u[L],N=e[C],k=N.size/d*(h?_:1),F=-k,M=k,I=-k,x=k;if(s){var P=l(N,k);P&&(P.minX<F&&(F=P.minX),P.maxX>M&&(M=P.maxX),P.minY<I&&(I=P.minY),P.maxY>x&&(x=P.maxY))}var W=n[C],G=W.x,R=W.y;S=Math.min(S,G+F/_),D=Math.max(D,G+M/_),A=Math.min(A,R-x/_),E=Math.max(E,R-I/_)}var j={x:[S,D],y:[A,E]},H=Math.max(j.x[1]-j.x[0],j.y[1]-j.y[0])||1,Y=Math.max(Math.abs(j.x[0]-g.x[0]),Math.abs(j.x[1]-g.x[1]),Math.abs(j.y[0]-g.y[0]),Math.abs(j.y[1]-g.y[1]));if(g=j,Y/H<Kh)break}return g}function Zo(a,n,e,t){var r=Ht(a,n);return{x:(1+r.x)*e/2,y:(1-r.y)*t/2}}var Zh=typeof navigator<"u"&&/Mac|iP(hone|ad|od)/.test(navigator.platform),Qh=1500,Jh=`.sigma-gesture-hint {
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
}`;function Qo(a){var n,e,t=qt("style");t.textContent=a,document.head.appendChild(t);var r=((n=(e=t.sheet)===null||e===void 0?void 0:e.cssRules.length)!==null&&n!==void 0?n:0)>0;return t.remove(),r}var Lt=null;function ec(){return Lt===null&&(Lt=Jh,Qo("@scope {}")&&(Lt=`@scope {
`.concat(Lt,`
}`)),Qo("@layer sigma-gesture-hint {}")&&(Lt=`@layer sigma-gesture-hint {
`.concat(Lt,`
}`))),Lt}var tc=(function(){function a(n){ne(this,a),w(this,"hideTimeout",null),this.styleElement=qt("style"),this.styleElement.textContent=ec(),n.appendChild(this.styleElement),this.element=qt("div",{opacity:"0"},{class:"sigma-gesture-hint"}),n.appendChild(this.element)}return ae(a,[{key:"show",value:function(e,t){var r=this,i=e==="touch"?t.sharedGestureTouchMessage:Zh?t.sharedGestureAppleWheelMessage:t.sharedGestureWheelMessage;i&&(this.element.textContent=i,this.element.style.opacity="1",typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.hideTimeout=window.setTimeout(function(){r.hideTimeout=null,r.element.style.opacity="0"},Qh))}},{key:"kill",value:function(){typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.styleElement.remove(),this.element.remove()}}])})(),nc=(function(){function a(n){ne(this,a),w(this,"readBuffer",null),w(this,"fence",null),w(this,"result",new Uint8Array(4)),w(this,"generation",0),w(this,"readGeneration",0),w(this,"lastEvent",null),w(this,"active",!1),w(this,"dirty",!1),w(this,"rafId",null),w(this,"killed",!1),this.options=n}return ae(a,[{key:"pointerMoved",value:function(e){this.lastEvent=e,this.active=!0,!this.options.isRenderPending()&&this.request()}},{key:"pointerLeft",value:function(){this.active=!1,this.dirty=!1}},{key:"frameRendered",value:function(){this.request()}},{key:"invalidate",value:function(){this.generation++}},{key:"request",value:function(){if(!(this.killed||!this.active||!this.lastEvent)){if(this.fence){this.dirty=!0;return}this.startRead()}}},{key:"startRead",value:function(){var e=this.options,t=e.gl,r=e.getFrameBuffer,i=e.getPixelRatio,o=e.getDownSizingRatio;this.readGeneration=this.generation;var s=this.lastEvent,l=Ua(t,s.x,s.y,i(),o()),u=le(l,2),d=u[0],h=u[1];this.readBuffer||(this.readBuffer=t.createBuffer()),t.bindFramebuffer(t.FRAMEBUFFER,r()),t.bindBuffer(t.PIXEL_PACK_BUFFER,this.readBuffer),t.bufferData(t.PIXEL_PACK_BUFFER,4,t.STREAM_READ),t.readPixels(d,h,1,1,t.RGBA,t.UNSIGNED_BYTE,0),t.bindBuffer(t.PIXEL_PACK_BUFFER,null),t.bindFramebuffer(t.FRAMEBUFFER,null),this.fence=t.fenceSync(t.SYNC_GPU_COMMANDS_COMPLETE,0),t.flush(),this.schedulePoll(s)}},{key:"schedulePoll",value:function(e){var t=this;this.rafId=requestAnimationFrame(function(){t.rafId=null,t.poll(e)})}},{key:"poll",value:function(e){var t=this.options,r=t.gl,i=t.onIndex;if(!(this.killed||!this.fence)){var o=r.clientWaitSync(this.fence,0,0);if(o===r.TIMEOUT_EXPIRED){this.schedulePoll(e);return}if(r.deleteSync(this.fence),this.fence=null,o!==r.WAIT_FAILED&&this.readGeneration===this.generation){r.bindBuffer(r.PIXEL_PACK_BUFFER,this.readBuffer),r.getBufferSubData(r.PIXEL_PACK_BUFFER,0,this.result),r.bindBuffer(r.PIXEL_PACK_BUFFER,null);var s=le(this.result,4),l=s[0],u=s[1],d=s[2],h=s[3];this.active&&i(Oa(l,u,d,h),e)}this.dirty&&(this.dirty=!1,this.active&&this.lastEvent&&!this.options.isRenderPending()&&this.startRead())}}},{key:"reset",value:function(){this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null,this.fence=null,this.readBuffer=null,this.dirty=!1}},{key:"kill",value:function(){this.killed=!0,this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null;var e=this.options.gl;this.fence&&e.deleteSync(this.fence),this.fence=null,this.readBuffer&&e.deleteBuffer(this.readBuffer),this.readBuffer=null}}])})(),Jo=(function(){function a(n,e){ne(this,a),this.key=n,this.size=e}return ae(a,null,[{key:"compare",value:function(e,t){return e.size>t.size?-1:e.size<t.size||e.key>t.key?1:-1}}])})(),Aa=(function(){function a(){ne(this,a),w(this,"width",0),w(this,"height",0),w(this,"cellSize",0),w(this,"columns",0),w(this,"rows",0),w(this,"cells",{})}return ae(a,[{key:"resizeAndClear",value:function(e,t){this.width=e.width,this.height=e.height,this.cellSize=t,this.columns=Math.ceil(e.width/t),this.rows=Math.ceil(e.height/t),this.cells={}}},{key:"getIndex",value:function(e){var t=Math.floor(e.x/this.cellSize),r=Math.floor(e.y/this.cellSize);return r*this.columns+t}},{key:"add",value:function(e,t,r){var i=new Jo(e,t),o=this.getIndex(r),s=this.cells[o];s||(s=[],this.cells[o]=s),s.push(i)}},{key:"organize",value:function(){for(var e in this.cells){var t=this.cells[e];t.sort(Jo.compare)}}},{key:"getLabelsToDisplay",value:function(e,t,r){var i=this.cellSize*this.cellSize,o=i/e/e,s=o*t/i,l=Math.ceil(s),u=[];if(r)for(var d=Math.max(0,Math.floor(r.x1/this.cellSize)),h=Math.min(this.columns-1,Math.floor(r.x2/this.cellSize)),c=Math.max(0,Math.floor(r.y1/this.cellSize)),f=Math.min(this.rows-1,Math.floor(r.y2/this.cellSize)),g=c;g<=f;g++)for(var b=d;b<=h;b++){var m=g*this.columns+b,v=this.cells[m];if(v)for(var p=0;p<Math.min(l,v.length);p++)u.push(v[p].key)}else for(var y in this.cells)for(var T=this.cells[y],_=0;_<Math.min(l,T.length);_++)u.push(T[_].key);return u}}])})();function ac(a){var n=a.graph,e=a.hoveredNode,t=a.highlightedNodes,r=a.displayedNodeLabels,i=new Set,o=new Set(t);e&&o.add(e);var s=U(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;n.hasNode(u)&&n.forEachEdge(u,function(f){i.add(f)})}}catch(f){s.e(f)}finally{s.f()}var d=U(r),h;try{var c=function(){var g=h.value;if(o.has(g))return 1;n.forEachEdge(g,function(b,m,v,p){var y=v===g?p:v;r.has(y)&&i.add(b)})};for(d.s();!(h=d.n()).done;)c()}catch(f){d.e(f)}finally{d.f()}return Array.from(i)}var nn=150,an=50,rc={both:0,node:1,label:2},ic={over:0,above:1,below:2,auto:3},es=12,oc=3,sc=(function(){function a(n){ne(this,a),w(this,"labelGrid",new Aa),w(this,"edgeAnchorGrid",new Aa),w(this,"displayedNodeLabels",new Set),w(this,"displayedEdgeLabels",new Set),w(this,"edgeLabelCandidates",[]),w(this,"renderedNodeLabels",new Set),w(this,"labelSizeCache",new Map),w(this,"framePassPoints",new Float32Array(0)),w(this,"labelsDirty",!0),this.internals=n}return ae(a,[{key:"resetFrame",value:function(){this.displayedNodeLabels=new Set,this.renderedNodeLabels=new Set,this.labelSizeCache.clear()}},{key:"clearEdgeLabels",value:function(){this.displayedEdgeLabels=new Set}},{key:"resetLabelGrid",value:function(){this.labelGrid=new Aa,this.edgeAnchorGrid=new Aa}},{key:"processWebGLLabels",value:function(e){for(var t,r=this.internals,i=r.labelProgram,o=r.primitives,s=r.nodeDataCache,l=(o==null||(t=o.nodes)===null||t===void 0||(t=t.label)===null||t===void 0||(t=t.font)===null||t===void 0?void 0:t.family)||"sans-serif",u=new Map,d=0,h=e.length;d<h;d++){var c=e[d],f=s[c];if(!(f.visibility==="hidden"||!f.label)){var g=f.labelFont||l,b=u.get(g);b?b.push(f.label):u.set(g,[f.label])}}var m=U(u),v;try{for(m.s();!(v=m.n()).done;){var p=le(v.value,2),y=p[0],T=p[1],_=ha(y),S=_.family,D=_.weight,A=_.style,E=i.registerFont(S,D,A);i.ensureGlyphsReady(T,E)}}catch(L){m.e(L)}finally{m.f()}}},{key:"measureNodeLabel",value:function(e){var t,r;if(!e.label)return{width:0,height:0,textHeight:0};var i=this.internals,o=i.labelProgram,s=i.primitives,l=(t=e.labelSize)!==null&&t!==void 0?t:14,u=e.labelFont||(s==null||(r=s.nodes)===null||r===void 0||(r=r.label)===null||r===void 0||(r=r.font)===null||r===void 0?void 0:r.family)||"sans-serif",d="".concat(e.label,"|").concat(l,"|").concat(u);if(this.labelSizeCache.has(d))return this.labelSizeCache.get(d);var h=ha(u),c=h.family,f=h.weight,g=h.style,b=o.registerFont(c,f,g),m=o.measureLabel(e.label,l,b);return this.labelSizeCache.set(d,m),m}},{key:"nodeLabelBox",value:function(e,t){var r,i,o,s,l;if(e.visibility==="hidden"||e.labelVisibility==="hidden")return null;var u=this.measureNodeLabel(e),d=u.width,h=u.height,c=u.textHeight;if(!d)return null;var f=(r=(i=this.internals.primitives)===null||i===void 0||(i=i.nodes)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:Zt,g=t+f,b=e.labelBackgroundColor?(o=e.labelBackgroundPadding)!==null&&o!==void 0?o:dr:0,m=d/2,v=h/2,p=m+b,y=v+b,T=c/2,_=0,S=0;switch((s=e.labelPosition)!==null&&s!==void 0?s:"right"){case"left":_=-(g+m);break;case"above":S=-(g+T);break;case"below":S=g+T;break;case"over":break;default:_=g+m}var D=(l=e.labelAngle)!==null&&l!==void 0?l:0,A=Ba({x:_,y:S},D),E=A.x,L=A.y,z=Math.abs(Math.cos(D)),C=Math.abs(Math.sin(D)),N=z*p+C*y,k=C*p+z*y;return{minX:E-N,maxX:E+N,minY:L-k,maxY:L+k}}},{key:"computeDisplayedNodeLabels",value:function(){var e=this.internals.getCameraState(),t=this.internals.getDimensions(),r=t.width,i=t.height,o=this.internals.viewportToFramedGraph({x:-nn,y:-an}),s=this.internals.viewportToFramedGraph({x:r+nn,y:-an}),l=this.internals.viewportToFramedGraph({x:-nn,y:i+an}),u=this.internals.viewportToFramedGraph({x:r+nn,y:i+an}),d=Math.min(o.x,s.x,l.x,u.x),h=Math.max(o.x,s.x,l.x,u.x),c=Math.min(o.y,s.y,l.y,u.y),f=Math.max(o.y,s.y,l.y,u.y),g=ft({x:.5,y:.5,ratio:1,angle:0},{width:r,height:i},this.internals.getGraphDimensions(),this.internals.getStagePadding()),b=function(P){var W=Ht(g,P);return{x:(1+W.x)*r/2,y:(1-W.y)*i/2}},m=b({x:d,y:c}),v=b({x:h,y:c}),p=b({x:d,y:f}),y=b({x:h,y:f}),T={x1:Math.min(m.x,v.x,p.x,y.x),y1:Math.min(m.y,v.y,p.y,y.y),x2:Math.max(m.x,v.x,p.x,y.x),y2:Math.max(m.y,v.y,p.y,y.y)},_=this.internals,S=_.settings,D=_.nodeDataCache,A=_.nodesWithForcedLabels,E=this.labelGrid.getLabelsToDisplay(e.ratio,S.labelDensity,T);er(E,A);for(var L=0,z=E.length;L<z;L++){var C=E[L],N=D[C];if(!this.displayedNodeLabels.has(C)&&!(N.visibility==="hidden"||N.labelVisibility==="hidden")&&N.label&&!(N.x<d||N.x>h||N.y<c||N.y>f)){var k=this.internals.framedGraphToViewport(N),F=k.x,M=k.y,I=this.internals.scaleSize(N.size);!vt(N)&&I<S.labelRenderedSizeThreshold||F<-nn-I||F>r+nn+I||M<-an-I||M>i+an+I||this.displayedNodeLabels.add(C)}}}},{key:"buildFramePassPoints",value:function(){var e=this.internals,t=e.nodeDataCache,r=e.nodeDataTexture;if(!r)return{data:this.framePassPoints,count:0};var i=this.displayedNodeLabels.size*3;this.framePassPoints.length<i&&(this.framePassPoints=new Float32Array(i));var o=this.framePassPoints,s=0,l=U(this.displayedNodeLabels),u;try{for(l.s();!(u=l.n()).done;){var d,h,c=u.value,f=t[c];if(f){var g=r.getIndex(c);g<0||(o[s*3]=g,o[s*3+1]=(d=Yt[f.labelPosition||ot.labelPosition])!==null&&d!==void 0?d:0,o[s*3+2]=(h=f.labelAngle)!==null&&h!==void 0?h:0,s++)}}}catch(b){l.e(b)}finally{l.f()}return{data:o,count:s}}},{key:"renderWebGLLabels",value:function(e,t){var r=this.internals,i=r.nodeDataCache,o=r.labelProgram,s=r.primitives,l=r.nodeDataTexture,u=[],d=U(this.displayedNodeLabels),h;try{for(d.s();!(h=d.n()).done;){var c=h.value;if(!this.renderedNodeLabels.has(c)){var f=i[c];t&&f.labelDepth!==t||(this.renderedNodeLabels.add(c),u.push(c))}}}catch(de){d.e(de)}finally{d.f()}if(u.length!==0){if(this.labelsDirty){for(var g,b,m,v=0,p=0,y=u.length;p<y;p++){var T=i[u[p]];v+=T.label.length}o.reallocate(v);for(var _=14,S=(g=s==null||(b=s.nodes)===null||b===void 0||(b=b.label)===null||b===void 0?void 0:b.margin)!==null&&g!==void 0?g:Zt,D=ot.labelPosition,A=(s==null||(m=s.nodes)===null||m===void 0||(m=m.label)===null||m===void 0||(m=m.font)===null||m===void 0?void 0:m.family)||"sans-serif",E=new Map,L=0,z=0,C=u.length;z<C;z++){var N,k,F,M,I=u[z],x=i[I],P=x.labelFont||A,W=E.get(P);if(W===void 0){var G=ha(P),R=G.family,j=G.weight,H=G.style;W=o.registerFont(R,j,H),E.set(P,W)}var Y={text:x.label,x:x.x,y:x.y,size:(N=x.labelSize)!==null&&N!==void 0?N:_,color:x.labelColor,nodeSize:x.size,margin:S,position:(k=x.labelPosition)!==null&&k!==void 0?k:D,hidden:!1,forceLabel:vt(x),type:"default",zIndex:(F=x.zIndex)!==null&&F!==void 0?F:0,parentType:"node",parentKey:I,fontKey:W,labelAngle:(M=x.labelAngle)!==null&&M!==void 0?M:0,nodeIndex:l.getIndex(I)},Te=o.processLabel(I,L,Y);L+=Te}o.invalidateBuffers()}o.render(e)}}},{key:"renderBackdrops",value:function(e,t){var r=this.internals,i=r.backdropProgram,o=r.nodeDataCache,s=r.nodesWithBackdrop,l=r.attachmentManager,u=r.pixelRatio,d=r.nodeDataTexture,h=[],c=U(s),f;try{for(c.s();!(f=c.n()).done;){var g,b=f.value,m=o[b];if(!(!m||m.visibility==="hidden")&&!(t&&m.depth!==t)){var v=(g=d?.getIndex(b))!==null&&g!==void 0?g:-1;v<0||h.push({key:b,nodeIndex:v})}}}catch(K){c.e(K)}finally{c.f()}if(h.length!==0){i.reallocate(h.length);for(var p=0;p<h.length;p++){var y,T,_,S,D,A,E,L,z=h[p],C=z.key,N=z.nodeIndex,k=o[C],F=this.displayedNodeLabels.has(C),M=F?this.measureNodeLabel(k):{width:0,height:0,textHeight:0},I=M.textHeight,x=M.width,P=M.height,W=0,G=0;if(F&&k.labelAttachment&&l){var R=l.getEntry(C,k.labelAttachment);if(R){var j=k.labelAttachmentPlacement||"below";if(j==="below"||j==="above"){var H=R.height/u;P+=H+bt,x=Math.max(x,R.width/u),G=j==="below"?(H+bt)/2:-(H+bt)/2}else{var Y=R.width/u;x+=Y+bt,P=Math.max(P,R.height/u),W=j==="right"?(Y+bt)/2:-(Y+bt)/2}}}var Te=k.backdropColor?gt(k.backdropColor):[255,255,255,255],de=k.backdropShadowColor?gt(k.backdropShadowColor):[0,0,0,128],q=Te.map(function(K){return K/255}),Re=de.map(function(K){return K/255}),V=(y=k.backdropShadowBlur)!==null&&y!==void 0?y:12,te=(T=k.backdropPadding)!==null&&T!==void 0?T:6,ue=k.backdropBorderColor?gt(k.backdropBorderColor):[0,0,0,0],pe=ue.map(function(K){return K/255}),Le=(_=k.backdropBorderWidth)!==null&&_!==void 0?_:0,se=(S=k.backdropCornerRadius)!==null&&S!==void 0?S:0,ee=(D=k.backdropLabelPadding)!==null&&D!==void 0?D:-1,ie=ee<0?te:ee,Fe=(A=rc[(E=k.backdropArea)!==null&&E!==void 0?E:"both"])!==null&&A!==void 0?A:0,J={key:C,nodeIndex:N,label:k.label,labelWidth:x,labelHeight:P,textHeight:I,type:"default",position:k.labelPosition||ot.labelPosition,labelAngle:(L=k.labelAngle)!==null&&L!==void 0?L:0,backdropColor:q,backdropShadowColor:Re,backdropShadowBlur:V,backdropPadding:te,backdropBorderColor:pe,backdropBorderWidth:Le,backdropCornerRadius:se,backdropLabelPadding:ie,backdropArea:Fe,labelBoxOffset:[W,G]};i.processBackdrop(p,J)}i.invalidateBuffers(),i.render(e)}}},{key:"renderLabelBackgrounds",value:function(e,t){var r=this.internals,i=r.labelBackgroundProgram,o=r.nodeDataCache,s=r.nodeDataTexture,l=Pe.nodeLabel.writesPickingThisFrame(this.internals),u=[],d=U(this.displayedNodeLabels),h;try{for(d.s();!(h=d.n()).done;){var c=h.value,f=o[c];!f||f.visibility==="hidden"||t&&f.labelDepth!==t||!l&&!f.labelBackgroundColor||u.push(c)}}catch(N){d.e(N)}finally{d.f()}if(u.length!==0){i.reallocate(u.length);for(var g=0;g<u.length;g++){var b,m,v,p,y=u[g],T=o[y],_=(b=s?.getIndex(y))!==null&&b!==void 0?b:-1;if(!(_<0)){var S=this.measureNodeLabel(T),D=S.width,A=S.height,E=S.textHeight,L=kt(this.internals.pickingState,"nodeLabel",y)||kt(this.internals.pickingState,"node",y),z=T.labelBackgroundColor?$e(T.labelBackgroundColor):$e("transparent"),C={nodeIndex:_,id:$t(L),color:z,labelWidth:D,labelHeight:A,textHeight:E,positionMode:(m=Yt[T.labelPosition||ot.labelPosition])!==null&&m!==void 0?m:0,labelAngle:(v=T.labelAngle)!==null&&v!==void 0?v:0,padding:(p=T.labelBackgroundPadding)!==null&&p!==void 0?p:dr};i.processLabelBackground(g,C)}}i.invalidateBuffers(),i.render(e)}}},{key:"cacheAttachments",value:function(e){var t=this.internals,r=t.attachmentManager,i=t.pixelRatio,o=t.nodeDataCache,s=t.nodesWithBackdrop,l=t.graph;if(r){var u=U(s),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;if(this.displayedNodeLabels.has(h)){var c=o[h];if(!(!c||c.visibility==="hidden")&&!(e&&c.depth!==e)&&c.labelAttachment){var f=l.getNodeAttributes(h),g=this.measureNodeLabel(c),b=g.width,m=g.height,v={node:h,attributes:f,pixelRatio:i,labelWidth:b,labelHeight:m};r.renderAttachment(h,c.labelAttachment,v)}}}}catch(p){u.e(p)}finally{u.f()}r.regenerateAtlas()}}},{key:"renderAttachments",value:function(e,t){var r=this.internals,i=r.attachmentManager,o=r.attachmentProgram,s=r.nodeDataCache,l=r.nodesWithBackdrop,u=r.pixelRatio,d=r.nodeDataTexture;if(!(!i||!o)){var h=0;o.reallocateAttachments(l.size);var c=U(l),f;try{for(c.s();!(f=c.n()).done;){var g,b,m,v,p=f.value;if(this.displayedNodeLabels.has(p)){var y=s[p];if(!(!y||y.visibility==="hidden")&&!(t&&y.labelDepth!==t)&&y.labelAttachment){var T=i.getEntry(p,y.labelAttachment);if(T){var _=(g=d?.getIndex(p))!==null&&g!==void 0?g:-1;if(!(_<0)){var S=this.measureNodeLabel(y),D=S.width,A=S.height,E=S.textHeight,L=(b=bo[y.labelAttachmentPlacement||"below"])!==null&&b!==void 0?b:0;o.processAttachment(h,{nodeIndex:_,atlasX:T.x,atlasY:T.y,atlasW:T.width,atlasH:T.height,attachWidth:T.width/u,attachHeight:T.height/u,positionMode:(m=Yt[y.labelPosition||ot.labelPosition])!==null&&m!==void 0?m:0,attachmentPlacement:L,labelWidth:D,labelHeight:A,textHeight:E,labelAngle:(v=y.labelAngle)!==null&&v!==void 0?v:0}),h++}}}}}}catch(z){c.e(z)}finally{c.f()}h!==0&&(o.reallocateAttachments(h),i.bindTexture(Sn),o.invalidateBuffers(),o.render(e))}}},{key:"computeDisplayedEdgeLabels",value:function(){var e=this.internals,t=e.graph,r=e.stateManager,i=e.settings,o=e.edgesWithForcedLabels,s=r.getHighlightedNodes(),l=i.edgeLabelAnchors==="allNodes"?new Set(this.edgeAnchorGrid.getLabelsToDisplay(this.internals.getCameraState().ratio,i.labelDensity)):this.displayedNodeLabels,u=r.hovered,d=ac({graph:t,hoveredNode:u?.kind==="node"?u.key:null,displayedNodeLabels:l,highlightedNodes:s});er(d,o),this.edgeLabelCandidates=d,this.displayedEdgeLabels=new Set}},{key:"filterEdgeLabelsForDepth",value:function(e){for(var t=this.internals,r=t.graph,i=t.nodeDataCache,o=t.edgeDataCache,s=[],l=new Set,u=0,d=this.edgeLabelCandidates.length;u<d;u++){var h=this.edgeLabelCandidates[u];if(!l.has(h)){l.add(h);var c=r.extremities(h),f=i[c[0]],g=i[c[1]],b=o[h];!b||!f||!g||b.visibility==="hidden"||b.labelVisibility==="hidden"||f.visibility==="hidden"||g.visibility==="hidden"||e&&b.labelDepth!==e||b.label&&s.push(h)}}return s}},{key:"renderEdgeLabels",value:function(e,t){var r=this.internals,i=r.graph,o=r.nodeDataCache,s=r.edgeDataCache,l=r.primitives,u=r.edgeLabelProgram,d=r.nodeDataTexture,h=r.edgeDataTexture;if(u){var c=this.filterEdgeLabelsForDepth(t),f=U(c),g;try{for(f.s();!(g=f.n()).done;){var b=g.value;this.displayedEdgeLabels.add(b)}}catch(Y){f.e(Y)}finally{f.f()}if(c.length!==0){if(this.labelsDirty){var m,v,p=0,y=U(c),T;try{for(y.s();!(T=y.n()).done;){var _=T.value;p+=s[_].label.length}}catch(Y){y.e(Y)}finally{y.f()}u.reallocate(p);var S=(m=l==null||(v=l.edges)===null||v===void 0||(v=v.label)===null||v===void 0?void 0:v.margin)!==null&&m!==void 0?m:5,D="over",A=0,E=U(c),L;try{for(E.s();!(L=E.n()).done;){var z,C,N=L.value,k=i.extremities(N),F=k[0],M=k[1],I=o[F],x=o[M],P=s[N],W=d.getIndex(F),G=d.getIndex(M),R=h.getIndex(N),j={text:P.label,x:(I.x+x.x)/2,y:(I.y+x.y)/2,size:es,color:P.labelColor,nodeSize:0,nodeIndex:-1,margin:S,position:(z=P.labelPosition)!==null&&z!==void 0?z:D,hidden:!1,forceLabel:vt(P),type:"default",zIndex:(C=P.zIndex)!==null&&C!==void 0?C:0,parentType:"edge",parentKey:N,fontKey:"",labelAngle:0,sourceX:I.x,sourceY:I.y,targetX:x.x,targetY:x.y,sourceSize:I.size,targetSize:x.size,sourceShape:I.shape||"circle",targetShape:x.shape||"circle",edgeSize:P.size,offset:0,edgeAttributes:P,sourceNodeIndex:W,targetNodeIndex:G,edgeIndex:R},H=u.processEdgeLabel(N,A,j);A+=H}}catch(Y){E.e(Y)}finally{E.f()}u.invalidateBuffers()}u.render(e)}}}},{key:"renderEdgeLabelBackgrounds",value:function(e,t){var r,i,o=this.internals,s=o.edgeLabelBackgroundProgram,l=o.edgeLabelProgram,u=o.edgeDataCache,d=o.primitives,h=o.edgeDataTexture;if(!(!h||!l||!s)){var c=Pe.edgeLabel.writesPickingThisFrame(this.internals),f=(r=d==null||(i=d.edges)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:5,g="over",b=this.filterEdgeLabelsForDepth(t),m=[],v=U(b),p;try{for(v.s();!(p=v.n()).done;){var y=p.value;!c&&!u[y].labelBackgroundColor||m.push(y)}}catch(I){v.e(I)}finally{v.f()}if(m.length!==0){s.reallocate(m.length);for(var T=0;T<m.length;T++){var _,S,D,A=m[T],E=u[A],L=E.label,z=l.measureLabelAtlasWidth(L),C=(_=E.labelPosition)!==null&&_!==void 0?_:g,N=typeof C=="string"&&(S=ic[C])!==null&&S!==void 0?S:0,k=kt(this.internals.pickingState,"edgeLabel",A)||kt(this.internals.pickingState,"edge",A),F=E.labelBackgroundColor?$e(E.labelBackgroundColor):$e("transparent"),M={edgeIndex:h.getIndex(A),baseFontSize:es,totalTextWidth:z,positionMode:N,margin:f,padding:(D=E.labelBackgroundPadding)!==null&&D!==void 0?D:oc,color:F,id:$t(k),edgeAttributes:E};s.processEdgeLabelBackground(T,A,M)}s.invalidateBuffers(),s.render(e)}}}}])})(),lc=(function(){function a(n,e,t,r){ne(this,a),w(this,"nodeStates",new Map),w(this,"edgeStates",new Map),w(this,"hovered",null),w(this,"dirtyNodes",new Set),w(this,"dirtyEdges",new Set),w(this,"graphStateChanged",!1),w(this,"graphStateFlagsDirty",!1),this.scheduleRefresh=n,this.customNodeStateDefaults=e,this.customEdgeStateDefaults=t,this.customGraphStateDefaults=r,this.graphState=Xa(r)}return ae(a,[{key:"getNodeState",value:function(e){e=""+e;var t=this.nodeStates.get(e);return t||(t=Oi(this.customNodeStateDefaults),this.nodeStates.set(e,t)),t}},{key:"getEdgeState",value:function(e){e=""+e;var t=this.edgeStates.get(e);return t||(t=Hi(this.customEdgeStateDefaults),this.edgeStates.set(e,t)),t}},{key:"getGraphState",value:function(){return this.flushGraphStateFlags(),this.graphState}},{key:"getHighlightedNodes",value:function(){var e=new Set,t=U(this.nodeStates),r;try{for(t.s();!(r=t.n()).done;){var i=le(r.value,2),o=i[0],s=i[1];s.isHighlighted&&e.add(o)}}catch(l){t.e(l)}finally{t.f()}return e}},{key:"setNodeState",value:function(e,t){e=""+e;var r=this.getNodeState(e);if(Xt(r,t)){var i=O(O({},r),t);this.nodeStates.set(e,i),this.dirtyNodes.add(e),this.updateHoveredNodeTracking(e,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setEdgeState",value:function(e,t){e=""+e;var r=this.getEdgeState(e);if(Xt(r,t)){var i=O(O({},r),t);this.edgeStates.set(e,i),this.dirtyEdges.add(e),this.updateHoveredEdgeTracking(e,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setGraphState",value:function(e){if(Xt(this.graphState,e)){var t=O(O({},this.graphState),e);t.isIdle=!t.isPanning&&!t.isZooming&&!t.isDragging,this.graphState=t,this.graphStateChanged=!0,this.scheduleRefresh()}}},{key:"setNodesState",value:function(e,t){var r=!1,i=U(e),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getNodeState(s);if(Xt(l,t)){var u=O(O({},l),t);this.nodeStates.set(s,u),this.dirtyNodes.add(s),this.updateHoveredNodeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"setEdgesState",value:function(e,t){var r=!1,i=U(e),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getEdgeState(s);if(Xt(l,t)){var u=O(O({},l),t);this.edgeStates.set(s,u),this.dirtyEdges.add(s),this.updateHoveredEdgeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"removeNode",value:function(e){this.nodeStates.delete(e),this.dirtyNodes.delete(e),this.clearHoveredFor("node",e)}},{key:"removeEdge",value:function(e){this.edgeStates.delete(e),this.dirtyEdges.delete(e),this.clearHoveredFor("edge",e)}},{key:"pruneNodes",value:function(e){var t=U(this.nodeStates.keys()),r;try{for(t.s();!(r=t.n()).done;){var i=r.value;e(i)||this.removeNode(i)}}catch(o){t.e(o)}finally{t.f()}}},{key:"pruneEdges",value:function(e){var t=U(this.edgeStates.keys()),r;try{for(t.s();!(r=t.n()).done;){var i=r.value;e(i)||this.removeEdge(i)}}catch(o){t.e(o)}finally{t.f()}}},{key:"clearNodes",value:function(){this.nodeStates.clear(),this.dirtyNodes.clear(),this.clearHoveredForKinds(Er("node"))}},{key:"clearEdges",value:function(){this.edgeStates.clear(),this.dirtyEdges.clear(),this.clearHoveredForKinds(Er("edge"))}},{key:"resetGraphState",value:function(){this.graphState=Xa(this.customGraphStateDefaults),this.graphStateChanged=!1,this.graphStateFlagsDirty=!1}},{key:"clearDirtyTracking",value:function(){this.dirtyNodes.clear(),this.dirtyEdges.clear(),this.graphStateChanged=!1}},{key:"setHovered",value:function(e){this.hovered=e}},{key:"clearHoveredFor",value:function(e,t){this.hovered&&this.hovered.key===t&&Er(e).includes(this.hovered.kind)&&(this.hovered=null)}},{key:"clearHoveredForKinds",value:function(e){this.hovered&&e.includes(this.hovered.kind)&&(this.hovered=null)}},{key:"updateGraphStateFromNodes",value:function(){var e,t=!1,r=!1,i=!1,o=U(this.nodeStates),s;try{for(o.s();!(s=o.n()).done;){var l=le(s.value,2),u=l[1];if(u.isHovered&&(t=!0),u.isHighlighted&&(r=!0),u.isDragged&&(i=!0),t&&r&&i)break}}catch(h){o.e(h)}finally{o.f()}!t&&((e=this.hovered)===null||e===void 0?void 0:e.kind)==="edge"&&(t=!0);var d=!this.graphState.isPanning&&!this.graphState.isZooming&&!i;(this.graphState.hasHovered!==t||this.graphState.hasHighlighted!==r||this.graphState.isDragging!==i||this.graphState.isIdle!==d)&&(this.graphStateChanged=!0),this.graphState=O(O({},this.graphState),{},{hasHovered:t,hasHighlighted:r,isDragging:i,isIdle:d})}},{key:"updateGraphStateFromEdges",value:function(){var e,t=((e=this.hovered)===null||e===void 0?void 0:e.kind)==="node";if(!t){var r=U(this.edgeStates),i;try{for(r.s();!(i=r.n()).done;){var o=le(i.value,2),s=o[1];if(s.isHovered){t=!0;break}}}catch(l){r.e(l)}finally{r.f()}}this.graphState.hasHovered!==t&&(this.graphStateChanged=!0),this.graphState=O(O({},this.graphState),{},{hasHovered:t})}},{key:"flushGraphStateFlags",value:function(){this.graphStateFlagsDirty&&(this.updateGraphStateFromNodes(),this.updateGraphStateFromEdges(),this.graphStateFlagsDirty=!1)}},{key:"updateHoveredNodeTracking",value:function(e,t,r){var i;if(t.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="node"&&this.hovered.key!==e){var s=this.hovered.key,l=this.getNodeState(s);this.nodeStates.set(s,O(O({},l),{},{isHovered:!1})),this.dirtyNodes.add(s)}this.hovered={kind:"node",key:e}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="node"&&this.hovered.key===e&&(this.hovered=null)}},{key:"updateHoveredEdgeTracking",value:function(e,t,r){var i;if(t.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="edge"&&this.hovered.key!==e){var s=this.hovered.key,l=this.getEdgeState(s);this.edgeStates.set(s,O(O({},l),{},{isHovered:!1})),this.dirtyEdges.add(s)}this.hovered={kind:"edge",key:e}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="edge"&&this.hovered.key===e&&(this.hovered=null)}}])})(),ts=1,ns=2,as=3,rs=4,uc=(function(a){function n(e,t){var r,i,o,s,l,u=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};ne(this,n),l=ye(this,n),w(l,"nodeReducer",null),w(l,"edgeReducer",null),w(l,"stageCanvas",null),w(l,"mouseLayer",null),w(l,"gestureHint",null),w(l,"extraElements",{}),w(l,"webGLContext",null),w(l,"pickingFrameBuffer",null),w(l,"pickingTexture",null),w(l,"pickingDepthBuffer",null),w(l,"activeListeners",{}),w(l,"nodeVariableEntries",[]),w(l,"edgeVariableEntries",[]),w(l,"edgePathsByName",new Map),w(l,"nodeProgramIndex",{}),w(l,"edgeProgramIndex",{}),w(l,"edgeTextureIndexCache",{}),w(l,"nodeGraphCoords",{}),w(l,"nodeExtent",{x:[0,1],y:[0,1]}),w(l,"matrix",He()),w(l,"invMatrix",He()),w(l,"correctionRatio",1),w(l,"frameId",0),w(l,"customBBox",null),w(l,"gpuTimerExt",void 0),w(l,"activeGpuTimerQuery",null),w(l,"pendingGpuTimerQueries",[]),w(l,"normalizationFunction",la({x:[0,1],y:[0,1]})),w(l,"graphToViewportRatio",1),w(l,"pickingState",Nh()),w(l,"prevNodeVisibilities",{}),w(l,"width",0),w(l,"height",0),w(l,"autoRescaleFrozen",!1),w(l,"stylesDeclaration",null),w(l,"resolvedStageStyle",{}),w(l,"renderFrame",null),w(l,"contextLost",!1),w(l,"pendingProcess","full"),w(l,"needToRefreshState",!1),w(l,"checkEdgesEventsFrame",null),w(l,"edgeStyleAnalysis",{dependency:"static",xAttribute:null,yAttribute:null}),w(l,"depthLayers",Z(ta)),w(l,"customLayerPrograms",new Map),w(l,"nodeShapeSlug",null),w(l,"sdfAtlas",null),w(l,"depthRanges",{nodes:{},edges:{}}),w(l,"nodeBaseDepth",{}),w(l,"edgeBaseDepth",{});var d=u.primitives,h=u.styles,c=u.settings,f=c===void 0?{}:c,g=u.nodeReducer,b=u.edgeReducer,m=u.customNodeState,v=u.customEdgeState,p=u.customGraphState;l.stateManager=new lc(function(){return l.scheduleStateRefresh()},m,v,p);var y=d??ji;l.stylesDeclaration=h?{nodes:(r=h.nodes)!==null&&r!==void 0?r:ra.nodes,edges:(i=h.edges)!==null&&i!==void 0?i:ra.edges,stage:h.stage}:ra,l.nodeReducer=g??null,l.edgeReducer=b??null;var T=qa(l.stylesDeclaration.nodes);l.nodeReducer&&(T.dependency="graph-state"),l.edgeStyleAnalysis=qa(l.stylesDeclaration.edges),l.edgeReducer&&(l.edgeStyleAnalysis.dependency="graph-state"),l.stylesDeclaration.stage&&(l.resolvedStageStyle=Ja(l.stylesDeclaration.stage,l.stateManager.graphState));var _=Qi(f);if(da(_),_.enableNodeDrag){var S=T.xAttribute,D=T.yAttribute;if((!S||!D)&&!_.dragPositionToAttributes)throw new Error('Sigma: `enableNodeDrag` is true but position attribute names could not be inferred from styles. Either use attribute bindings for x/y in your node styles (e.g. `x: { attribute: "x" }`), or provide a `dragPositionToAttributes` setting.')}if(Zi(e),!(t instanceof HTMLElement))throw new Error("Sigma: container should be an html element.");l.container=t,l.edgeGroups=new zh(e,function(x,P){for(var W=0;W<x.length;W++){var G=l.stateManager.getEdgeState(x[W]);G.parallelIndex=W,G.parallelCount=P}});var A=new Gh(e,l.viewportToGraph.bind(l),l.setNodesState.bind(l),function(x,P){return l.emit(x,P)});if(l.depthLayers=(o=y.depthLayers)!==null&&o!==void 0?o:Z(ta),!(h!=null&&h.nodes)&&!Jn.every(function(x){return l.depthLayers.includes(x)}))throw new Error("Sigma: depthLayers must include ".concat(Jn.join(", ")," for the built-in node styles."));if(!(h!=null&&h.edges)&&!ea.every(function(x){return l.depthLayers.includes(x)}))throw new Error("Sigma: depthLayers must include ".concat(ea.join(", ")," for the built-in edge styles."));l.itemBuckets={nodes:new yr(l.depthLayers),edges:new yr(l.depthLayers)},l.initWebGLContext(),l.mouseLayer=qt("div",{position:"absolute",touchAction:"none",userSelect:"none"},{class:"sigma-mouse"}),l.container.appendChild(l.mouseLayer),l.resolvedStageStyle.background&&(l.container.style.backgroundColor=l.resolvedStageStyle.background),l.resolvedStageStyle.cursor&&(l.container.style.cursor=l.resolvedStageStyle.cursor);var E=new Bo(l.webGLContext),L=new _r(l.webGLContext,{channels:1}),z=new Uo(l.webGLContext),C=new _r(l.webGLContext,{channels:4}),N=l.webGLContext,k=new nc({gl:N,getFrameBuffer:function(){return l.pickingFrameBuffer},getPixelRatio:function(){return l.internals.pixelRatio},getDownSizingRatio:function(){return l.internals.settings.pickingDownSizingRatio},isRenderPending:function(){return l.renderFrame!==null},onIndex:function(P,W){var G;return ls(l.internals,(G=l.pickingState.lookup[P])!==null&&G!==void 0?G:null,W)}}),F=l.initPrograms(y,_),M=y==null||(s=y.nodes)===null||s===void 0?void 0:s.labelAttachments,I=null;return M&&Object.keys(M).length>0&&(I=new Wo(N,M,function(){return l.scheduleRender()})),l.internals=O(O({nodeDataCache:{},edgeDataCache:{},nodesWithForcedLabels:new Set,nodesWithBackdrop:new Set,edgesWithForcedLabels:new Set,settings:_,primitives:y,pixelRatio:mn(),graph:e,stateManager:l.stateManager,dragManager:A,hoverResolver:k,nodeStyleAnalysis:T,pickingState:l.pickingState},F),{},{attachmentManager:I,nodeDataTexture:E,nodeFrameTexture:L,edgeDataTexture:z,edgeFrameTexture:C,getDimensions:function(){return l.getDimensions()},getGraphDimensions:function(){return l.getGraphDimensions()},getStagePadding:function(){return l.getStagePadding()},getCameraState:function(){return l.camera.getState()},getHitAtPosition:function(P){return l.getHitAtPosition(P)},setNodeState:function(P,W){return l.setNodeState(P,W)},setEdgeState:function(P,W){return l.setEdgeState(P,W)},updateContainerCursor:function(){return l.updateContainerCursor()},scheduleRefresh:function(){return l.scheduleRefresh()},viewportToFramedGraph:function(P){return l.viewportToFramedGraph(P)},viewportToGraph:function(P){return l.viewportToGraph(P)},framedGraphToViewport:function(P){return l.framedGraphToViewport(P)},scaleSize:function(P){return l.scaleSize(P)},emit:function(P,W){return l.emit(P,W)}}),l.labelRenderer=new sc(l.internals),l.resize(),l.initializeWebGLLabels(),l.camera=new $o,l.bindCameraHandlers(),l.mouseCaptor=new kh(l.mouseLayer,l),l.mouseCaptor.setSettings(l.internals.settings),l.touchCaptor=new Ih(l.mouseLayer,l),l.touchCaptor.setSettings(l.internals.settings),l.bindEventHandlers(),l.bindGraphHandlers(),l.handleSettingsUpdate(),l.refresh(),l}return _e(n,a),ae(n,[{key:"initializeWebGLLabels",value:function(){this.sdfAtlas=new Dt,this.sdfAtlas.registerFont({family:"sans-serif",weight:"normal",style:"normal"})}},{key:"resetWebGLTexture",value:function(){var t=this.webGLContext;if(!this.pickingFrameBuffer)return this;var r=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),i=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);t.bindFramebuffer(t.FRAMEBUFFER,this.pickingFrameBuffer),this.pickingTexture&&t.deleteTexture(this.pickingTexture);var o=t.createTexture();o&&(t.bindTexture(t.TEXTURE_2D,o),t.texImage2D(t.TEXTURE_2D,0,t.RGBA,r,i,0,t.RGBA,t.UNSIGNED_BYTE,null),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.framebufferTexture2D(t.FRAMEBUFFER,t.COLOR_ATTACHMENT0,t.TEXTURE_2D,o,0),this.pickingTexture=o),this.pickingDepthBuffer&&t.deleteRenderbuffer(this.pickingDepthBuffer);var s=t.createRenderbuffer();return s&&(t.bindRenderbuffer(t.RENDERBUFFER,s),t.renderbufferStorage(t.RENDERBUFFER,t.DEPTH_COMPONENT16,r,i),t.framebufferRenderbuffer(t.FRAMEBUFFER,t.DEPTH_ATTACHMENT,t.RENDERBUFFER,s),this.pickingDepthBuffer=s),t.bindFramebuffer(t.FRAMEBUFFER,null),this}},{key:"bindCameraHandlers",value:function(){var t=this;return this.activeListeners.camera=function(){t.refreshMatrices(),t.labelRenderer.labelsDirty=!0,t.scheduleRender()},this.activeListeners.cameraAnimationStart=function(r){var i=r.from,o=r.to;i.ratio!==o.ratio&&t.stateManager.setGraphState({isZooming:!0})},this.activeListeners.cameraAnimationEnd=function(){t.stateManager.setGraphState({isZooming:!1})},this.camera.on("updated",this.activeListeners.camera),this.camera.on("animationStart",this.activeListeners.cameraAnimationStart),this.camera.on("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"unbindCameraHandlers",value:function(){return this.camera.removeListener("updated",this.activeListeners.camera),this.camera.removeListener("animationStart",this.activeListeners.cameraAnimationStart),this.camera.removeListener("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"getHitAtPosition",value:function(t){var r;if(this.contextLost)return null;var i=this.webGLContext;i.bindFramebuffer(i.FRAMEBUFFER,this.pickingFrameBuffer);var o=Ti(i,this.pickingFrameBuffer,t.x,t.y,this.internals.pixelRatio,this.internals.settings.pickingDownSizingRatio),s=Oa.apply(void 0,Z(o));return(r=this.pickingState.lookup[s])!==null&&r!==void 0?r:null}},{key:"bindEventHandlers",value:function(){return $h(this.internals,this.mouseCaptor,this.touchCaptor,this.activeListeners),this}},{key:"bindGraphHandlers",value:function(){return Vh({graph:this.internals.graph,edgeGroups:this.edgeGroups,addNode:this.addNode.bind(this),updateNode:this.updateNode.bind(this),removeNode:this.removeNode.bind(this),addEdge:this.addEdge.bind(this),updateEdge:this.updateEdge.bind(this),removeEdge:this.removeEdge.bind(this),clearEdgeState:this.clearEdgeState.bind(this),clearNodeState:this.clearNodeState.bind(this),clearEdgeIndices:this.clearEdgeIndices.bind(this),clearNodeIndices:this.clearNodeIndices.bind(this),refresh:this.refresh.bind(this)},this.activeListeners),this}},{key:"unbindGraphHandlers",value:function(){jh(this.internals.graph,this.activeListeners)}},{key:"getNodeShapeId",value:function(t){return this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&t.shape&&t.shape in this.internals.nodeShapeMap?this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[t.shape]]:Tn(t.shape||"circle")}},{key:"processNodes",value:function(){var t=this,r=this.internals.graph,i=this.internals.settings,o=this.getDimensions(),s=i.autoRescale,l=i.autoRescaleContent,u=this.nodeExtent;if(s===!1){var d=o.width,h=o.height;u={x:[-d/2,d/2],y:[-h/2,h/2]}}else(s!=="once"||!this.autoRescaleFrozen)&&(u=this.computeNodeExtent(),l!=="positions"&&!this.customBBox&&(u=Yh({extent:u,coords:this.nodeGraphCoords,nodeData:this.internals.nodeDataCache,dimensions:o,stagePadding:this.getStagePadding(),zoomToSizeRatioFunction:i.zoomToSizeRatioFunction,itemSizesReference:i.itemSizesReference,fitLabels:l==="labels",nodeLabelBox:function(G,R){return t.labelRenderer.nodeLabelBox(G,R)}})),s==="once"&&(this.autoRescaleFrozen=!0));this.nodeExtent=u,this.normalizationFunction=la(this.customBBox||this.nodeExtent);var c=new $o,f=ft(c.getState(),o,this.getGraphDimensions(),this.getStagePadding());this.labelRenderer.labelGrid.resizeAndClear(o,i.labelGridCellSize),this.labelRenderer.edgeAnchorGrid.resizeAndClear(o,i.labelGridCellSize);var g=i.renderEdgeLabels&&i.edgeLabelAnchors==="allNodes",b=!1;Tr(this.pickingState,"node");for(var m=r.nodes(),v=0,p=m.length;v<p;v++){var y=m[v],T=this.internals.nodeDataCache[y],_=this.nodeGraphCoords[y];T.x=_.x,T.y=_.y,this.normalizationFunction.applyTo(T),T.visibility!==this.prevNodeVisibilities[y]&&(b=!0),typeof T.label=="string"&&T.visibility!=="hidden"&&T.labelVisibility!=="hidden"&&this.labelRenderer.labelGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:f})),g&&T.visibility!=="hidden"&&this.labelRenderer.edgeAnchorGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:f}))}this.labelRenderer.labelGrid.organize(),this.labelRenderer.edgeAnchorGrid.organize(),this.nodeProgram.reallocate(m.length);var S=0;this.depthRanges.nodes={},this.nodeBaseDepth={};var D=this.internals.nodeDataCache,A=U(this.depthLayers),E;try{for(A.s();!(E=A.n()).done;){var L=E.value,z=this.itemBuckets.nodes.getSorted(L,function(W){return D[W].zIndex});if(z.length!==0){this.depthRanges.nodes[L]=[{offset:S,count:z.length}];var C=U(z),N;try{for(C.s();!(N=C.n()).done;){var k=N.value;this.nodeBaseDepth[k]=L,this.nodeProgram.allocateNode(k),Ko(this.pickingState,"node",k),this.addNodeToProgram(k,S++)}}catch(W){C.e(W)}finally{C.f()}}}}catch(W){A.e(W)}finally{A.f()}this.nodeProgram.invalidateBuffers();for(var F=0,M=m.length;F<M;F++)this.prevNodeVisibilities[m[F]]=this.internals.nodeDataCache[m[F]].visibility;this.labelRenderer.processWebGLLabels(m);var I=U(this.customLayerPrograms.values()),x;try{for(I.s();!(x=I.n()).done;){var P=x.value.program;P.cacheData&&P.cacheData()}}catch(W){I.e(W)}finally{I.f()}return b}},{key:"processEdges",value:function(){var t=this.internals.graph,r=t.edges();this.edgeProgram.reallocate(r.length);var i=0;Tr(this.pickingState,"edge"),this.depthRanges.edges={},this.edgeBaseDepth={};var o=this.internals.edgeDataCache,s=U(this.depthLayers),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=this.itemBuckets.edges.getSorted(u,function(g){return o[g].zIndex});if(d.length!==0){this.depthRanges.edges[u]=[{offset:i,count:d.length}];var h=U(d),c;try{for(h.s();!(c=h.n()).done;){var f=c.value;this.edgeBaseDepth[f]=u,Ko(this.pickingState,"edge",f),this.addEdgeToProgram(f,i++)}}catch(g){h.e(g)}finally{h.f()}}}}catch(g){s.e(g)}finally{s.f()}this.edgeProgram.invalidateBuffers()}},{key:"updateNodeDepthRanges",value:function(t,r,i){var o=this.nodeProgramIndex[t];o!==void 0&&(nr(this.depthRanges.nodes,r,o),ar(this.depthRanges.nodes,i,o))}},{key:"updateEdgeDepthRanges",value:function(t,r,i){var o=this.edgeProgramIndex[t];o!==void 0&&(nr(this.depthRanges.edges,r,o),ar(this.depthRanges.edges,i,o))}},{key:"handleSettingsUpdate",value:function(){var t=this,r=this.internals.settings;return this.camera.minRatio=r.minCameraRatio,this.camera.maxRatio=r.maxCameraRatio,this.camera.enabledZooming=r.enableCameraZooming,this.camera.enabledPanning=r.enableCameraPanning,this.camera.enabledRotation=r.enableCameraRotation,r.cameraPanBoundaries?this.camera.constrainState=function(i){return t.cleanCameraState(i,r.cameraPanBoundaries&&oe(r.cameraPanBoundaries)==="object"?r.cameraPanBoundaries:{})}:this.camera.constrainState=null,this.camera.setState(this.camera.getState()),this.mouseLayer.style.touchAction=r.gestureTarget==="graph"?"none":r.gestureTarget==="shared"?"pan-x pan-y":"auto",r.gestureTarget!=="shared"&&this.gestureHint&&(this.gestureHint.kill(),this.gestureHint=null),this.mouseCaptor.setSettings(this.internals.settings),this.touchCaptor.setSettings(this.internals.settings),this}},{key:"cleanCameraState",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=r.tolerance,o=i===void 0?0:i,s=r.boundaries,l=O({},t),u=s||this.nodeExtent,d=le(u.x,2),h=d[0],c=d[1],f=le(u.y,2),g=f[0],b=f[1],m=[this.graphToViewport({x:h,y:g},{cameraState:t}),this.graphToViewport({x:c,y:g},{cameraState:t}),this.graphToViewport({x:h,y:b},{cameraState:t}),this.graphToViewport({x:c,y:b},{cameraState:t})],v=1/0,p=-1/0,y=1/0,T=-1/0;m.forEach(function(k){var F=k.x,M=k.y;v=Math.min(v,F),p=Math.max(p,F),y=Math.min(y,M),T=Math.max(T,M)});var _=p-v,S=T-y,D=this.getDimensions(),A=D.width,E=D.height,L=0,z=0;if(_>=A?p<A-o?L=p-(A-o):v>o&&(L=v-o):p>A+o?L=p-(A+o):v<-o&&(L=v+o),S>=E?T<E-o?z=T-(E-o):y>o&&(z=y-o):T>E+o?z=T-(E+o):y<-o&&(z=y+o),L||z){var C=this.viewportToFramedGraph({x:0,y:0},{cameraState:t}),N=this.viewportToFramedGraph({x:L,y:z},{cameraState:t});L=N.x-C.x,z=N.y-C.y,l.x+=L,l.y+=z}return l}},{key:"refreshMatrices",value:function(){var t=this.camera.getState(),r=this.getDimensions(),i=this.getGraphDimensions(),o=this.getStagePadding();this.matrix=ft(t,r,i,o),this.invMatrix=ft(t,r,i,o,!0),this.correctionRatio=bi(this.matrix,t,r),this.graphToViewportRatio=this.getGraphToViewportRatio()}},{key:"processData",value:function(){var t;this.emit("beforeProcess"),(t=this.internals.attachmentManager)===null||t===void 0||t.clear();var r=O({},this.pickingState.idsByKind),i=this.processNodes();(this.pendingProcess==="full"||i)&&this.processEdges(),Mh(this.pickingState,this.internals),Wh(r,this.pickingState.idsByKind)||this.internals.hoverResolver.invalidate(),this.pendingProcess="none",this.emit("afterProcess")}},{key:"render",value:function(){var t=this;if(this.contextLost)return this;this.emit("beforeRender");var r=function(){return t.internals.hoverResolver.frameRendered(),t.emit("afterRender"),t};if(this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null),this.resize(),(this.pendingProcess!=="none"||this.needToRefreshState)&&(this.labelRenderer.labelsDirty=!0),this.pendingProcess!=="none"&&this.processData(),this.needToRefreshState&&this.refreshState(),this.needToRefreshState=!1,this.stateManager.clearDirtyTracking(),this.pendingProcess!=="none"&&this.processData(),this.clear(),this.resetWebGLTexture(),!this.internals.graph.order)return r();var i=this.mouseCaptor,o=this.camera.isAnimating()||i.isMoving||i.draggedEvents||i.currentWheelDirection;if(this.refreshMatrices(),this.frameId++,this.internals.settings.renderEdgeLabels&&!this.internals.edgeLabelProgram){var s=this.createEdgeLabelPrograms(),l=s.labelProgram,u=s.labelBackgroundProgram;this.internals.edgeLabelProgram=l,this.internals.edgeLabelBackgroundProgram=u}var d=this.internals.settings.DEBUG_logRenderStats?this.getDebugPrograms():null;d&&d.forEach(function(Y){return Y.resetDebugStats()}),this.internals.settings.DEBUG_gpuTimerQueries&&this.beginGpuTimerQuery(),this.internals.nodeFrameTexture.ensureCapacity(this.internals.nodeDataTexture.getCapacity()),this.internals.edgeFrameTexture.ensureCapacity(this.internals.edgeDataTexture.getCapacity());var h=this.getRenderParams(),c=Pe.edge.writesPickingThisFrame(this.internals)?h:O(O({},h),{},{pickingFrameBuffer:null});this.labelRenderer.resetFrame();var f=this.webGLContext,g=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),b=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);if(f.bindFramebuffer(f.FRAMEBUFFER,this.pickingFrameBuffer),f.viewport(0,0,g,b),f.clear(f.COLOR_BUFFER_BIT|f.DEPTH_BUFFER_BIT),f.bindFramebuffer(f.FRAMEBUFFER,null),f.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),f.clear(f.COLOR_BUFFER_BIT|f.DEPTH_BUFFER_BIT),this.internals.nodeDataTexture.upload(),this.internals.edgeDataTexture.upload(),this.nodeProgram.uploadLayerTexture(),this.edgeProgram.uploadAttributeTexture(),this.emit("afterTexturesUpload"),this.internals.nodeDataTexture.bind(as),this.internals.edgeDataTexture.bind(rs),this.edgeFramePass.run(h,this.internals.edgeFrameTexture,this.internals.edgeDataTexture.getHighWaterMark(),this.edgeProgram.getAttributeTexture(),this.nodeProgram.getAttributeTexture()),this.internals.edgeFrameTexture.bind(ts),this.internals.settings.renderLabels){this.labelRenderer.computeDisplayedNodeLabels();var m=this.labelRenderer.buildFramePassPoints(),v=m.data,p=m.count;this.nodeFramePass.run(v,p,this.internals.nodeFrameTexture,h,this.nodeProgram.getAttributeTexture()),this.internals.nodeFrameTexture.bind(ns)}this.internals.settings.renderEdgeLabels&&this.labelRenderer.computeDisplayedEdgeLabels();var y=U(this.customLayerPrograms.values()),T;try{for(y.s();!(T=y.n()).done;){var _=T.value.program;_.preRender&&_.preRender(h)}}catch(Y){y.e(Y)}finally{y.f()}f.bindFramebuffer(f.FRAMEBUFFER,null),f.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),f.blendFunc(f.ONE,f.ONE_MINUS_SRC_ALPHA);var S=!this.internals.settings.hideLabelsOnMove||!o,D=U(this.depthLayers),A;try{for(D.s();!(A=D.n()).done;){var E=A.value,L=U(this.customLayerPrograms.values()),z;try{for(L.s();!(z=L.n()).done;){var C=z.value;C.depth===E&&C.program.render(h)}}catch(Y){L.e(Y)}finally{L.f()}var N=this.depthRanges.edges[E];if(N&&(!this.internals.settings.hideEdgesOnMove||!o)){var k=U(N),F;try{for(k.s();!(F=k.n()).done;){var M=F.value,I=M.offset,x=M.count;x>0&&this.edgeProgram.render(c,I,x)}}catch(Y){k.e(Y)}finally{k.f()}}this.internals.settings.renderEdgeLabels&&S&&(this.labelRenderer.renderEdgeLabelBackgrounds(Pe.edgeLabel.writesPickingThisFrame(this.internals)?h:O(O({},h),{},{pickingFrameBuffer:null}),E),this.labelRenderer.renderEdgeLabels(h,E)),this.labelRenderer.cacheAttachments(E),this.labelRenderer.renderBackdrops(O(O({},h),{},{pickingFrameBuffer:null}),E);var P=this.depthRanges.nodes[E];if(P){var W=U(P),G;try{for(W.s();!(G=W.n()).done;){var R=G.value,j=R.offset,H=R.count;H>0&&this.nodeProgram.render(h,j,H)}}catch(Y){W.e(Y)}finally{W.f()}}S&&(this.labelRenderer.renderAttachments(O(O({},h),{},{pickingFrameBuffer:null}),E),this.labelRenderer.renderLabelBackgrounds(Pe.nodeLabel.writesPickingThisFrame(this.internals)?h:O(O({},h),{},{pickingFrameBuffer:null}),E),this.internals.settings.renderLabels&&this.labelRenderer.renderWebGLLabels(h,E))}}catch(Y){D.e(Y)}finally{D.f()}return this.labelRenderer.labelsDirty=!1,this.internals.settings.DEBUG_displayPickingLayer&&(f.bindFramebuffer(f.READ_FRAMEBUFFER,this.pickingFrameBuffer),f.bindFramebuffer(f.DRAW_FRAMEBUFFER,null),f.blitFramebuffer(0,0,g,b,0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio,f.COLOR_BUFFER_BIT,f.NEAREST)),this.internals.settings.DEBUG_gpuTimerQueries&&this.endGpuTimerQuery(),this.pollGpuTimerQueries(),d&&this.logRenderStats(d),r()}},{key:"getDebugPrograms",value:function(){return[this.nodeProgram,this.edgeProgram,this.internals.labelProgram,this.internals.edgeLabelProgram,this.internals.edgeLabelBackgroundProgram,this.internals.backdropProgram,this.internals.labelBackgroundProgram,this.internals.attachmentProgram].filter(function(t){return t!==null})}},{key:"logRenderStats",value:function(t){var r={},i=U(t),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;r[s.constructor.name]=O({},s.debugStats)}}catch(l){i.e(l)}finally{i.f()}console.log("[sigma] DEBUG_logRenderStats: frame #".concat(this.frameId)),console.table(r)}},{key:"getGpuTimerExtension",value:function(){return this.gpuTimerExt===void 0&&(this.gpuTimerExt=this.webGLContext.getExtension("EXT_disjoint_timer_query_webgl2"),this.gpuTimerExt||console.warn("Sigma: DEBUG_gpuTimerQueries is enabled, but this browser/driver doesn't support EXT_disjoint_timer_query_webgl2.")),this.gpuTimerExt}},{key:"beginGpuTimerQuery",value:function(){var t=this.getGpuTimerExtension();if(t){var r=this.webGLContext,i=r.createQuery();i&&(r.beginQuery(t.TIME_ELAPSED_EXT,i),this.activeGpuTimerQuery=i)}}},{key:"endGpuTimerQuery",value:function(){!this.gpuTimerExt||!this.activeGpuTimerQuery||(this.webGLContext.endQuery(this.gpuTimerExt.TIME_ELAPSED_EXT),this.pendingGpuTimerQueries.push({query:this.activeGpuTimerQuery,frameId:this.frameId}),this.activeGpuTimerQuery=null)}},{key:"pollGpuTimerQueries",value:function(){if(this.pendingGpuTimerQueries.length!==0){var t=this.webGLContext,r=this.gpuTimerExt;if(!r){this.pendingGpuTimerQueries.forEach(function(h){var c=h.query;return t.deleteQuery(c)}),this.pendingGpuTimerQueries=[];return}var i=t.getParameter(r.GPU_DISJOINT_EXT),o=[],s=U(this.pendingGpuTimerQueries),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if(!t.getQueryParameter(u.query,t.QUERY_RESULT_AVAILABLE)){o.push(u);continue}if(!i){var d=t.getQueryParameter(u.query,t.QUERY_RESULT);console.log("[sigma] DEBUG_gpuTimerQueries: frame #".concat(u.frameId," GPU time = ").concat((d/1e6).toFixed(2),"ms"))}t.deleteQuery(u.query)}}catch(h){s.e(h)}finally{s.f()}this.pendingGpuTimerQueries=o}}},{key:"postEvaluateNode",value:function(t,r,i){var o=t;o.x===void 0&&(o.x=r.x),o.y===void 0&&(o.y=r.y),t.highlighted=i.isHighlighted;for(var s=0,l=this.nodeVariableEntries.length;s<l;s++){var u,d,h=le(this.nodeVariableEntries[s],2),c=h[0],f=h[1];o[c]=(u=(d=o[c])!==null&&d!==void 0?d:r[c])!==null&&u!==void 0?u:f.default}}},{key:"addNode",value:function(t){var r=this.internals.graph.getNodeAttributes(t),i=this.stateManager.getNodeState(t),o=this.internals.nodeDataCache[t]||{};if(Za(this.stylesDeclaration.nodes,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateNode(o,r,i),this.nodeReducer){var s=this.nodeReducer(t,o,r,i,this.stateManager.graphState,this.internals.graph);o=O(O({},o),s)}if(typeof o.x!="number"||typeof o.y!="number")throw new Error('Sigma: could not find a valid position (x, y) for node "'.concat(t,'". ')+"Provide coordinates via node attributes, styles, or a nodeReducer.");this.internals.nodeShapeMap?(!o.shape||!(o.shape in this.internals.nodeShapeMap))&&(o.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(o.shape=this.nodeShapeSlug),this.internals.nodeDataCache[t]=o,this.nodeGraphCoords[t]={x:o.x,y:o.y},At(this.internals.nodesWithForcedLabels,t,vt(o)),At(this.internals.nodesWithBackdrop,t,tr(o)),this.itemBuckets.nodes.set(t,o.depth)}},{key:"updateNode",value:function(t){this.addNode(t);var r=this.internals.nodeDataCache[t];this.normalizationFunction.applyTo(r)}},{key:"removeNode",value:function(t){this.itemBuckets.nodes.remove(t),delete this.internals.nodeDataCache[t],delete this.nodeGraphCoords[t],delete this.nodeProgramIndex[t],this.internals.dragManager.removeNode(t),this.stateManager.removeNode(t),this.internals.nodesWithForcedLabels.delete(t),this.internals.nodesWithBackdrop.delete(t)}},{key:"postEvaluateEdge",value:function(t,r){for(var i=t,o=0,s=this.edgeVariableEntries.length;o<s;o++){var l,u,d=le(this.edgeVariableEntries[o],2),h=d[0],c=d[1];i[h]=(l=(u=i[h])!==null&&u!==void 0?u:r[h])!==null&&l!==void 0?l:c.default}}},{key:"applyEdgeSpread",value:function(t,r,i){var o;if(!(i.parallelCount<=1)){var s=this.internals.graph.source(t),l=this.internals.graph.target(t),u=s===l,d=u?r.selfLoopPath||r.path:r.parallelPath||r.path,h=d?this.edgePathsByName.get(d):void 0;if(h!=null&&h.spread){var c=(o=r.parallelSpread)!==null&&o!==void 0?o:.25,f=h.spread.compute(i.parallelIndex,i.parallelCount,c);!u&&this.internals.graph.isDirected(t)&&s>l&&(f=-f),r[h.spread.variable]=f}}}},{key:"addEdge",value:function(t){var r=this.internals.graph.getEdgeAttributes(t),i=this.stateManager.getEdgeState(t),o={};if(Qa(this.stylesDeclaration.edges,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateEdge(o,r),this.edgeReducer){var s=this.edgeReducer(t,o,r,i,this.stateManager.graphState,this.internals.graph);o=O(O({},o),s)}this.applyEdgeSpread(t,o,i),this.internals.edgeDataCache[t]=o,At(this.internals.edgesWithForcedLabels,t,vt(o)),this.itemBuckets.edges.set(t,o.depth)}},{key:"updateEdge",value:function(t){this.addEdge(t)}},{key:"removeEdge",value:function(t){this.itemBuckets.edges.remove(t),delete this.internals.edgeDataCache[t],delete this.edgeProgramIndex[t],delete this.edgeTextureIndexCache[t],this.internals.edgeDataTexture.free(t),this.stateManager.removeEdge(t),this.internals.edgesWithForcedLabels.delete(t)}},{key:"clearNodeIndices",value:function(){this.labelRenderer.resetLabelGrid(),this.autoRescaleFrozen||(this.nodeExtent={x:[0,1],y:[0,1]}),this.internals.nodeDataCache={},this.nodeGraphCoords={},this.edgeProgramIndex={},this.internals.nodesWithForcedLabels.clear(),this.internals.nodesWithBackdrop.clear(),this.prevNodeVisibilities={},this.itemBuckets.nodes.clearAll(),this.depthRanges.nodes={},this.nodeBaseDepth={}}},{key:"clearEdgeIndices",value:function(){this.internals.edgeDataCache={},this.edgeProgramIndex={},this.edgeTextureIndexCache={},this.internals.edgesWithForcedLabels.clear(),Tr(this.pickingState,"edge"),this.itemBuckets.edges.clearAll(),this.depthRanges.edges={},this.edgeBaseDepth={},this.edgeGroups.clear()}},{key:"clearIndices",value:function(){this.clearEdgeIndices(),this.clearNodeIndices()}},{key:"clearNodeState",value:function(){this.labelRenderer.resetFrame(),this.internals.nodesWithBackdrop.clear(),this.internals.dragManager.clear(),this.autoRescaleFrozen=!1,this.stateManager.clearNodes()}},{key:"clearEdgeState",value:function(){this.labelRenderer.clearEdgeLabels(),this.stateManager.clearEdges()}},{key:"clearState",value:function(){this.clearEdgeState(),this.clearNodeState(),this.stateManager.resetGraphState()}},{key:"addNodeToProgram",value:function(t,r){var i,o=this.internals.nodeDataCache[t];this.internals.nodeDataTexture.allocate(t),(i=this.internals.nodeDataTexture).updateNode.apply(i,[t,o.x,o.y,o.size,this.getNodeShapeId(o)].concat(Z(sa(o)),[o.color]));var s=this.internals.nodeDataTexture.getIndex(t);this.nodeProgram.process(kt(this.pickingState,"node",t),r,o,s,t),this.nodeProgramIndex[t]=r}},{key:"addEdgeToProgram",value:function(t,r){var i,o,s=this.internals.edgeDataCache[t],l=this.internals.graph.source(t),u=this.internals.graph.target(t),d=this.internals.edgeDataTexture.allocate(t);this.edgeTextureIndexCache[t]=d;var h=l===u,c=!h&&((i=(o=this.stateManager.getEdgeState(t))===null||o===void 0?void 0:o.parallelCount)!==null&&i!==void 0?i:1)>1,f=this.edgeProgram.resolveEdgeIds(s,h,c),g=f.pathId,b=f.headId,m=f.tailId,v=f.headLengthRatio,p=f.tailLengthRatio;this.internals.edgeDataTexture.updateEdge(t,this.internals.nodeDataTexture.getIndex(l),this.internals.nodeDataTexture.getIndex(u),s.size,v,p,g,b,m),this.edgeProgram.process(kt(this.pickingState,"edge",t),r,this.internals.nodeDataCache[l],this.internals.nodeDataCache[u],s,d),this.edgeProgramIndex[t]=r}},{key:"getRenderParams",value:function(){return{frameId:this.frameId,matrix:this.matrix,invMatrix:this.invMatrix,width:this.width,height:this.height,pixelRatio:this.internals.pixelRatio,zoomRatio:this.camera.ratio,cameraAngle:this.camera.angle,sizeRatio:1/this.scaleSize(),correctionRatio:this.correctionRatio,downSizingRatio:this.internals.settings.pickingDownSizingRatio,minEdgeThickness:this.internals.settings.minEdgeThickness,antiAliasingFeather:this.internals.settings.antiAliasingFeather,nodePickingPadding:this.internals.settings.nodePickingPadding,edgePickingPadding:this.internals.settings.edgePickingPadding,labelPickingPadding:this.internals.settings.labelPickingPadding,nodeDataTextureUnit:as,nodeDataTextureWidth:this.internals.nodeDataTexture.getTextureWidth(),nodeFrameTextureUnit:ns,nodeFrameTextureWidth:this.internals.nodeFrameTexture.getTextureWidth(),edgeDataTextureUnit:rs,edgeDataTextureWidth:this.internals.edgeDataTexture.getTextureWidth(),edgeFrameTextureUnit:ts,edgeFrameTextureWidth:this.internals.edgeFrameTexture.getTextureWidth(),pickingFrameBuffer:this.pickingFrameBuffer,labelPixelSnapping:this.internals.settings.labelPixelSnapping?1:0}}},{key:"getStagePadding",value:function(){var t=this.internals.settings,r=t.stagePadding,i=t.autoRescale;return i&&r||0}},{key:"getLayerElement",value:function(t){if(t==="mouse")return this.mouseLayer;var r=this.extraElements[t];if(!r)throw new Error('Sigma: layer "'.concat(t,'" does not exist'));return r}},{key:"initWebGLContext",value:function(){var t=this,r=this.createWebGLContext("stage");this.stageCanvas=this.extraElements.stage,this.webGLContext=r,this.stageCanvas.addEventListener("webglcontextlost",function(i){t.webGLContext&&(i.preventDefault(),t.contextLost=!0,t.renderFrame&&(cancelAnimationFrame(t.renderFrame),t.renderFrame=null),t.gpuTimerExt=void 0,t.activeGpuTimerQuery=null,t.pendingGpuTimerQueries=[],t.emit("webglContextLost"))}),this.stageCanvas.addEventListener("webglcontextrestored",function(){requestAnimationFrame(function(){t.webGLContext&&!t.webGLContext.isContextLost()&&t.restoreWebGLContext()})}),this.initPickingFramebuffer()}},{key:"initPickingFramebuffer",value:function(){var t=this.webGLContext,r=t.createFramebuffer();if(!r)throw new Error("Sigma: cannot create picking frame buffer");t.bindFramebuffer(t.FRAMEBUFFER,r);var i=t.createTexture();if(!i)throw new Error("Sigma: cannot create picking texture");t.bindTexture(t.TEXTURE_2D,i),t.texImage2D(t.TEXTURE_2D,0,t.RGBA,1,1,0,t.RGBA,t.UNSIGNED_BYTE,null),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_S,t.CLAMP_TO_EDGE),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_T,t.CLAMP_TO_EDGE),t.framebufferTexture2D(t.FRAMEBUFFER,t.COLOR_ATTACHMENT0,t.TEXTURE_2D,i,0);var o=t.createRenderbuffer();if(!o)throw new Error("Sigma: cannot create picking depth buffer");if(t.bindRenderbuffer(t.RENDERBUFFER,o),t.renderbufferStorage(t.RENDERBUFFER,t.DEPTH_COMPONENT16,1,1),t.framebufferRenderbuffer(t.FRAMEBUFFER,t.DEPTH_ATTACHMENT,t.RENDERBUFFER,o),t.checkFramebufferStatus(t.FRAMEBUFFER)!==t.FRAMEBUFFER_COMPLETE)throw new Error("Sigma: picking framebuffer is not complete");t.bindFramebuffer(t.FRAMEBUFFER,null),this.pickingFrameBuffer=r,this.pickingTexture=i,this.pickingDepthBuffer=o}},{key:"initPrograms",value:function(t,r){var i=this,o=this.webGLContext,s=Fo(o,this.pickingFrameBuffer,i,t?.nodes,r.antialiasNodes),l=s.nodeProgram,u=s.labelProgram,d=s.backdropProgram,h=s.labelBackgroundProgram,c=s.attachmentProgram,f=s.framePass,g=s.shapeSlug,b=s.shapeNameToIndex,m=s.shapeGlobalIds,v=s.variables;this.nodeProgram=l,this.nodeFramePass=f,this.nodeVariableEntries=Object.entries(v),g&&(this.nodeShapeSlug=g);var p=Io(o,this.pickingFrameBuffer,i,t?.edges,r.antialiasEdges,t?.nodes),y=p.edgeProgram,T=p.createLabelPrograms,_=p.framePass,S=p.variables,D=p.paths;return this.createEdgeLabelPrograms=T,this.edgeProgram=y,this.edgeFramePass=_,this.edgeVariableEntries=Object.entries(S),this.edgePathsByName=new Map(D.map(function(A){return[A.name,A]})),{labelProgram:u,edgeLabelProgram:null,edgeLabelBackgroundProgram:null,backdropProgram:d,labelBackgroundProgram:h,attachmentProgram:c,nodeShapeMap:b??null,nodeGlobalShapeIds:m??null}}},{key:"restoreWebGLContext",value:function(){var t,r,i,o,s,l=this.internals;this.initPickingFramebuffer(),(t=l.nodeDataTexture)===null||t===void 0||t.restore(),(r=l.nodeFrameTexture)===null||r===void 0||r.restore(),(i=l.edgeDataTexture)===null||i===void 0||i.restore(),(o=l.edgeFrameTexture)===null||o===void 0||o.restore(),(s=l.attachmentManager)===null||s===void 0||s.restore(),l.hoverResolver.reset(),Object.assign(l,this.initPrograms(l.primitives,l.settings));var u=U(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var h=d.value;h.program.kill(),h.program=h.factory(this.webGLContext)}}catch(c){u.e(c)}finally{u.f()}this.contextLost=!1,this.emit("webglContextRestored"),this.refresh()}},{key:"createLayer",value:function(t,r){var i=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};if(this.extraElements[t])throw new Error('Sigma: a layer named "'.concat(t,'" already exists'));var o=qt(r,{position:"absolute"},{class:"sigma-".concat(t)});return i.style&&Object.assign(o.style,i.style),this.extraElements[t]=o,"beforeLayer"in i&&i.beforeLayer?this.getLayerElement(i.beforeLayer).before(o):"afterLayer"in i&&i.afterLayer?this.getLayerElement(i.afterLayer).after(o):this.container.appendChild(o),o}},{key:"createCanvas",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.createLayer(t,"canvas",r)}},{key:"createWebGLContext",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=this.createCanvas(t,r);r.hidden&&i.remove();var o=i.getContext("webgl2",O({preserveDrawingBuffer:!1,antialias:!1,depth:!0},r));if(!o)throw new Error("Sigma: WebGL 2 is not supported by your browser. Please use a modern browser (Chrome 56+, Firefox 51+, Safari 15+, Edge 79+).");return o.blendFunc(o.ONE,o.ONE_MINUS_SRC_ALPHA),o}},{key:"killLayer",value:function(t){if(t==="stage"||t==="mouse")throw new Error('Sigma: cannot kill built-in layer "'.concat(t,'"'));var r=this.extraElements[t];if(!r)throw new Error("Sigma: cannot kill layer ".concat(t,", which does not exist"));return r.remove(),delete this.extraElements[t],this}},{key:"getWebGLContext",value:function(){if(!this.webGLContext)throw new Error("Sigma: WebGL context is not available");return this.webGLContext}},{key:"getNodeDataTexture",value:function(){if(!this.internals.nodeDataTexture)throw new Error("Sigma: node data texture is not available");return this.internals.nodeDataTexture}},{key:"getNormalizationFunction",value:function(){return this.normalizationFunction}},{key:"addCustomLayerProgram",value:function(t,r,i){var o;if(!this.depthLayers.includes(r))throw new Error('Sigma: cannot add custom layer program at depth "'.concat(r,'", ')+"it must be declared in primitives.depthLayers. Current layers: ".concat(this.depthLayers.join(", ")));(o=this.customLayerPrograms.get(t))===null||o===void 0||o.program.kill();var s=i(this.webGLContext);return this.customLayerPrograms.set(t,{depth:r,factory:i,program:s}),s.cacheData&&s.cacheData(),this.scheduleRender(),this}},{key:"removeCustomLayerProgram",value:function(t){var r=this.customLayerPrograms.get(t);return r&&(r.program.kill(),this.customLayerPrograms.delete(t),this.scheduleRender()),this}},{key:"getCamera",value:function(){return this.camera}},{key:"setCamera",value:function(t){return this.camera.cancelAnimation(),this.unbindCameraHandlers(),this.camera=t,this.bindCameraHandlers(),this.scheduleRender(),this}},{key:"getContainer",value:function(){return this.container}},{key:"getGraph",value:function(){return this.internals.graph}},{key:"setGraph",value:function(t){return t===this.internals.graph?this:(this.stateManager.pruneNodes(function(r){return t.hasNode(r)}),this.stateManager.pruneEdges(function(r){return t.hasEdge(r)}),this.unbindGraphHandlers(),this.checkEdgesEventsFrame!==null&&(cancelAnimationFrame(this.checkEdgesEventsFrame),this.checkEdgesEventsFrame=null),this.internals.graph=t,this.autoRescaleFrozen=!1,this.bindGraphHandlers(),this.refresh(),this)}},{key:"getMouseCaptor",value:function(){return this.mouseCaptor}},{key:"getTouchCaptor",value:function(){return this.touchCaptor}},{key:"getDimensions",value:function(){return{width:this.width,height:this.height}}},{key:"getGraphDimensions",value:function(){var t=this.customBBox||this.nodeExtent;return{width:t.x[1]-t.x[0]||1,height:t.y[1]-t.y[0]||1}}},{key:"getNodeDisplayData",value:function(t){var r=this.internals.nodeDataCache[t];return r?Object.assign({},r):void 0}},{key:"getEdgeDisplayData",value:function(t){var r=this.internals.edgeDataCache[t];return r?Object.assign({},r):void 0}},{key:"getNodeState",value:function(t){return this.stateManager.getNodeState(t)}},{key:"getEdgeState",value:function(t){return this.stateManager.getEdgeState(t)}},{key:"getGraphState",value:function(){return this.stateManager.getGraphState()}},{key:"setNodeState",value:function(t,r){return this.stateManager.setNodeState(t,r),this}},{key:"setEdgeState",value:function(t,r){return this.stateManager.setEdgeState(t,r),this}},{key:"setGraphState",value:function(t){return this.stateManager.setGraphState(t),this}},{key:"_setPanning",value:function(t){this.stateManager.setGraphState({isPanning:t})}},{key:"_setZooming",value:function(t){this.stateManager.setGraphState({isZooming:t})}},{key:"_hasNodeDrag",value:function(){var t=this.internals.dragManager;return!!(t.pendingNode||t.session)}},{key:"_showGestureHint",value:function(t){var r=this.internals.settings;r.gestureTarget==="shared"&&(this.gestureHint=this.gestureHint||new tc(this.container),this.gestureHint.show(t,r))}},{key:"setNodesState",value:function(t,r){return this.stateManager.setNodesState(t,r),this}},{key:"setEdgesState",value:function(t,r){return this.stateManager.setEdgesState(t,r),this}},{key:"updateContainerCursor",value:function(){var t=this.stateManager.hovered,r=this.resolvedStageStyle.cursor||"";this.container.style.cursor=t&&Bh(this.internals,t)||r}},{key:"refreshStageStyle",value:function(){this.resolvedStageStyle=Ja(this.stylesDeclaration.stage,this.stateManager.graphState),this.resolvedStageStyle.background!==void 0&&(this.container.style.backgroundColor=this.resolvedStageStyle.background),this.updateContainerCursor()}},{key:"getNodeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedNodeLabels)}},{key:"getEdgeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedEdgeLabels)}},{key:"getSettings",value:function(){return O({},this.internals.settings)}},{key:"getSetting",value:function(t){return this.internals.settings[t]}},{key:"getStyles",value:function(){return O({},this.stylesDeclaration)}},{key:"getPrimitives",value:function(){return O({},this.internals.primitives)}},{key:"setSetting",value:function(t,r){return this.internals.settings[t]=r,da(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"updateSetting",value:function(t,r){return this.setSetting(t,r(this.internals.settings[t])),this}},{key:"setSettings",value:function(t){return this.internals.settings=O(O({},this.internals.settings),t),da(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"resize",value:function(t){var r=this.width,i=this.height;if(this.width=this.container.offsetWidth,this.height=this.container.offsetHeight,this.internals.pixelRatio=mn(),this.width===0)if(this.internals.settings.allowInvalidContainer)this.width=1;else throw new Error("Sigma: Container has no width. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(this.height===0)if(this.internals.settings.allowInvalidContainer)this.height=1;else throw new Error("Sigma: Container has no height. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(!t&&r===this.width&&i===this.height)return this;this.labelRenderer.labelsDirty=!0;for(var o=0,s=[this.mouseLayer].concat(Z(Object.values(this.extraElements)));o<s.length;o++){var l=s[o];l.style.width=this.width+"px",l.style.height=this.height+"px"}return this.webGLContext&&(this.stageCanvas.setAttribute("width",this.width*this.internals.pixelRatio+"px"),this.stageCanvas.setAttribute("height",this.height*this.internals.pixelRatio+"px"),this.webGLContext.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio)),this.emit("resize"),this}},{key:"clear",value:function(){return this.emit("beforeClear"),this.webGLContext.bindFramebuffer(WebGLRenderingContext.FRAMEBUFFER,null),this.webGLContext.clear(WebGLRenderingContext.COLOR_BUFFER_BIT),this.emit("afterClear"),this}},{key:"scheduleStateRefresh",value:function(){this.needToRefreshState=!0,this.scheduleRender()}},{key:"refreshState",value:function(){var t=this,r;this.stateManager.flushGraphStateFlags();var i=this.stateManager.graphStateChanged&&this.internals.nodeStyleAnalysis.dependency==="graph-state",o=this.stateManager.graphStateChanged&&this.edgeStyleAnalysis.dependency==="graph-state",s=!1;if(i)this.internals.graph.forEachNode(function(g){t.refreshNodeState(g)&&(s=!0)});else if(this.internals.nodeStyleAnalysis.dependency!=="static"){var l=U(this.stateManager.dirtyNodes),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;this.refreshNodeState(d)&&(s=!0)}}catch(g){l.e(g)}finally{l.f()}}if(o)this.internals.graph.forEachEdge(function(g){t.refreshEdgeState(g)&&(s=!0)});else if(this.edgeStyleAnalysis.dependency!=="static"){var h=U(this.stateManager.dirtyEdges),c;try{for(h.s();!(c=h.n()).done;){var f=c.value;this.refreshEdgeState(f)&&(s=!0)}}catch(g){h.e(g)}finally{h.f()}}this.stateManager.graphStateChanged&&(r=this.stylesDeclaration)!==null&&r!==void 0&&r.stage&&this.refreshStageStyle(),this.stateManager.clearDirtyTracking(),s&&(this.pendingProcess="full")}},{key:"refreshNodeState",value:function(t){var r=this.internals.nodeDataCache[t];if(!r||this.nodeReducer){var i,o,s,l,u=(i=this.internals.nodeDataCache[t])===null||i===void 0?void 0:i.depth,d=(o=this.internals.nodeDataCache[t])===null||o===void 0?void 0:o.zIndex,h=(s=this.internals.nodeDataCache[t])===null||s===void 0?void 0:s.labelAttachment;this.updateNode(t);var c=this.internals.nodeDataCache[t];this.internals.attachmentManager&&c.labelAttachment!==h&&this.internals.attachmentManager.invalidateNode(t);var f;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&c.shape&&c.shape in this.internals.nodeShapeMap?f=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[c.shape]]:f=Tn(c.shape||"circle"),(l=this.internals.nodeDataTexture).updateNode.apply(l,[t,c.x,c.y,c.size,f].concat(Z(sa(c)),[c.color])),u&&c.depth!==u&&this.updateNodeDepthRanges(t,u,c.depth);var g=this.nodeProgramIndex[t];return g!==void 0&&(this.addNodeToProgram(t,g),this.nodeProgram.invalidateBuffers()),d!==void 0&&c.zIndex!==d}var b=this.internals.graph.getNodeAttributes(t),m=this.stateManager.getNodeState(t),v=r.size,p=r.shape,y=r.depth,T=r.zIndex,_=r.labelAttachment,S=r.rotationAlignment,D=r.labelRotationAlignment,A=r.color;Za(this.stylesDeclaration.nodes,b,m,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateNode(r,b,m);var E=this.nodeGraphCoords[t],L=r.x!==E.x||r.y!==E.y;if(L&&(E.x=r.x,E.y=r.y),this.normalizationFunction.applyTo(r),this.internals.nodeShapeMap?(!r.shape||!(r.shape in this.internals.nodeShapeMap))&&(r.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(r.shape=this.nodeShapeSlug),this.internals.attachmentManager&&r.labelAttachment!==_&&this.internals.attachmentManager.invalidateNode(t),At(this.internals.nodesWithForcedLabels,t,vt(r)),At(this.internals.nodesWithBackdrop,t,tr(r)),L||r.size!==v||r.shape!==p||r.rotationAlignment!==S||r.labelRotationAlignment!==D||r.color!==A){var z,C;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&r.shape&&r.shape in this.internals.nodeShapeMap?C=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[r.shape]]:C=Tn(r.shape||"circle"),(z=this.internals.nodeDataTexture).updateNode.apply(z,[t,r.x,r.y,r.size,C].concat(Z(sa(r)),[r.color]))}this.itemBuckets.nodes.set(t,r.depth),r.depth!==y&&this.updateNodeDepthRanges(t,y,r.depth);var N=this.nodeProgramIndex[t];return N!==void 0&&(this.addNodeToProgram(t,N),this.nodeProgram.invalidateBuffers()),r.zIndex!==T}},{key:"refreshEdgeState",value:function(t){var r=this.internals.edgeDataCache[t];if(!r||this.edgeReducer){var i=r?.depth,o=r?.zIndex;this.updateEdge(t);var s=this.internals.edgeDataCache[t];i&&s.depth!==i&&this.updateEdgeDepthRanges(t,i,s.depth);var l=this.edgeProgramIndex[t];return l!==void 0&&(this.addEdgeToProgram(t,l),this.edgeProgram.invalidateBuffers()),o!==void 0&&s.zIndex!==o}var u=this.internals.graph.getEdgeAttributes(t),d=this.stateManager.getEdgeState(t),h=r.depth,c=r.zIndex,f=r.size,g=r.path,b=r.selfLoopPath,m=r.parallelPath,v=r.head,p=r.tail;Qa(this.stylesDeclaration.edges,u,d,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateEdge(r,u),this.applyEdgeSpread(t,r,d),At(this.internals.edgesWithForcedLabels,t,vt(r)),this.itemBuckets.edges.set(t,r.depth),r.depth!==h&&this.updateEdgeDepthRanges(t,h,r.depth);var y=this.edgeProgramIndex[t];if(y!==void 0){var T=r.size!==f||r.path!==g||r.selfLoopPath!==b||r.parallelPath!==m||r.head!==v||r.tail!==p;if(T)this.addEdgeToProgram(t,y),this.edgeProgram.invalidateBuffers();else{var _=this.internals.graph.source(t),S=this.internals.graph.target(t),D=this.internals.nodeDataCache[_],A=this.internals.nodeDataCache[S],E=this.edgeTextureIndexCache[t];this.edgeProgram.process(kt(this.pickingState,"edge",t),y,D,A,r,E),this.edgeProgram.invalidateBuffers()}}return r.zIndex!==c}},{key:"refresh",value:function(t){var r=this,i=t?.skipIndexation!==void 0?t?.skipIndexation:!1,o=t?.schedule!==void 0?t.schedule:!1,s=!t||!t.partialGraph;if(s)this.clearEdgeIndices(),this.clearNodeIndices(),this.internals.graph.forEachNode(function(D){return r.addNode(D)}),this.edgeGroups.rebuild(),this.internals.graph.forEachEdge(function(D){return r.addEdge(D)}),this.pendingProcess="full";else{for(var l,u,d=((l=t.partialGraph)===null||l===void 0?void 0:l.nodes)||[],h=0,c=d?.length||0;h<c;h++){var f,g,b=""+d[h],m=(f=this.internals.nodeDataCache[b])===null||f===void 0?void 0:f.labelAttachment;if(this.updateNode(b),this.internals.attachmentManager&&((g=this.internals.nodeDataCache[b])!==null&&g!==void 0&&g.labelAttachment||m)&&this.internals.attachmentManager.invalidateNode(b),i){var v=this.nodeProgramIndex[b];if(v===void 0)throw new Error('Sigma: node "'.concat(b,`" can't be repaint`));this.addNodeToProgram(b,v)}}i&&d.length>0&&(this.nodeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0);for(var p=(t==null||(u=t.partialGraph)===null||u===void 0?void 0:u.edges)||[],y=0,T=p.length;y<T;y++){var _=""+p[y];if(this.updateEdge(_),i){var S=this.edgeProgramIndex[_];if(S===void 0)throw new Error('Sigma: edge "'.concat(_,`" can't be repaint`));this.addEdgeToProgram(_,S)}}i&&p.length>0&&(this.edgeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0),!i&&this.pendingProcess!=="full"&&(this.pendingProcess=p.length>0?"full":"nodes")}return o?this.scheduleRender():this.render(),this}},{key:"scheduleRender",value:function(){var t=this;return this.renderFrame||(this.renderFrame=requestAnimationFrame(function(){t.render()})),this}},{key:"scheduleRefresh",value:function(t){return this.refresh(O(O({},t),{},{schedule:!0}))}},{key:"getViewportZoomedState",value:function(t,r){var i=this.camera.getState(),o=i.ratio,s=i.angle,l=i.x,u=i.y,d=this.internals.settings,h=d.minCameraRatio,c=d.maxCameraRatio;typeof c=="number"&&(r=Math.min(r,c)),typeof h=="number"&&(r=Math.max(r,h));var f=r/o,g={x:this.width/2,y:this.height/2},b=this.viewportToFramedGraph(t),m=this.viewportToFramedGraph(g);return{angle:s,x:(b.x-m.x)*(1-f)+l,y:(b.y-m.y)*(1-f)+u,ratio:r}}},{key:"viewRectangle",value:function(){var t=this.viewportToFramedGraph({x:0,y:0}),r=this.viewportToFramedGraph({x:this.width,y:0}),i=this.viewportToFramedGraph({x:0,y:this.height});return{x1:t.x,y1:t.y,x2:r.x,y2:r.y,height:r.y-i.y}}},{key:"framedGraphToViewport",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?ft(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding()):this.matrix),s=Ht(o,t);return{x:(1+s.x)*this.width/2,y:(1-s.y)*this.height/2}}},{key:"viewportToFramedGraph",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?ft(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding(),!0):this.invMatrix),s=Ht(o,{x:t.x/this.width*2-1,y:1-t.y/this.height*2});return isNaN(s.x)&&(s.x=0),isNaN(s.y)&&(s.y=0),s}},{key:"viewportToGraph",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.normalizationFunction.inverse(this.viewportToFramedGraph(t,r))}},{key:"graphToViewport",value:function(t){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.framedGraphToViewport(this.normalizationFunction(t),r)}},{key:"graphToFramedGraph",value:function(t){return this.normalizationFunction(t)}},{key:"framedGraphToGraph",value:function(t){return this.normalizationFunction.inverse(t)}},{key:"getGraphToViewportRatio",value:function(){var t={x:0,y:0},r={x:1,y:1},i=Math.sqrt(Math.pow(t.x-r.x,2)+Math.pow(t.y-r.y,2)),o=this.graphToViewport(t),s=this.graphToViewport(r),l=Math.sqrt(Math.pow(o.x-s.x,2)+Math.pow(o.y-s.y,2));return l/i}},{key:"computeNodeExtent",value:function(){var t=this.nodeGraphCoords,r=Object.keys(t);if(!r.length)return{x:[0,1],y:[0,1]};for(var i=1/0,o=-1/0,s=1/0,l=-1/0,u=0,d=r.length;u<d;u++){var h=t[r[u]],c=h.x,f=h.y;c<i&&(i=c),c>o&&(o=c),f<s&&(s=f),f>l&&(l=f)}return{x:[i,o],y:[s,l]}}},{key:"getBBox",value:function(){return this.nodeExtent}},{key:"getCustomBBox",value:function(){return this.customBBox}},{key:"setCustomBBox",value:function(t){return this.customBBox=t,this.scheduleRender(),this}},{key:"kill",value:function(){var t,r,i,o,s;this.emit("kill"),this.removeAllListeners(),this.camera.cancelAnimation(),this.unbindCameraHandlers(),window.removeEventListener("resize",this.activeListeners.handleResize),this.mouseCaptor.kill(),this.touchCaptor.kill(),this.internals.hoverResolver.kill(),this.unbindGraphHandlers(),this.clearIndices(),this.clearState(),this.internals.nodeDataCache={},this.internals.edgeDataCache={},this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null);for(var l=this.container;l.firstChild;)l.removeChild(l.firstChild);this.nodeProgram.kill(),this.nodeFramePass.kill(),this.edgeProgram.kill(),this.edgeFramePass.kill(),this.internals.labelProgram.kill(),(t=this.internals.edgeLabelProgram)===null||t===void 0||t.kill(),(r=this.internals.edgeLabelBackgroundProgram)===null||r===void 0||r.kill(),this.internals.backdropProgram.kill(),this.internals.labelBackgroundProgram.kill(),(i=this.internals.attachmentProgram)===null||i===void 0||i.kill(),(o=this.internals.attachmentManager)===null||o===void 0||o.kill(),this.internals.attachmentProgram=null,this.internals.attachmentManager=null;var u=U(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var h=d.value.program;h.kill()}}catch(g){u.e(g)}finally{u.f()}if(this.customLayerPrograms.clear(),this.sdfAtlas&&(this.sdfAtlas=null),this.internals.nodeDataTexture&&(this.internals.nodeDataTexture.kill(),this.internals.nodeDataTexture=null),this.internals.nodeFrameTexture&&(this.internals.nodeFrameTexture.kill(),this.internals.nodeFrameTexture=null),this.internals.edgeDataTexture&&(this.internals.edgeDataTexture.kill(),this.internals.edgeDataTexture=null),this.internals.edgeFrameTexture&&(this.internals.edgeFrameTexture.kill(),this.internals.edgeFrameTexture=null),this.webGLContext){var c;(c=this.webGLContext.getExtension("WEBGL_lose_context"))===null||c===void 0||c.loseContext(),this.webGLContext=null}(s=this.gestureHint)===null||s===void 0||s.kill(),this.gestureHint=null,this.mouseLayer.remove();for(var f in this.extraElements)this.extraElements[f].remove();this.extraElements={}}},{key:"scaleSize",value:function(){var t=arguments.length>0&&arguments[0]!==void 0?arguments[0]:1,r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:this.camera.ratio;return t/this.internals.settings.zoomToSizeRatioFunction(r)*(this.getSetting("itemSizesReference")==="positions"?r*this.graphToViewportRatio:1)}},{key:"getStageCanvas",value:function(){return this.stageCanvas}},{key:"getMouseLayer",value:function(){return this.mouseLayer}}])})(oa),us=uc;var ds=["#4f7cff","#ff3d8b","#12c99b","#ff9f1a","#a259ff","#ff5c39","#1bb8f0","#8bc34a","#e040fb","#00bfa5"],Dr=a=>getComputedStyle(document.documentElement).getPropertyValue(a).trim(),dc="Segoe UI Variable Text, Segoe UI, system-ui, sans-serif",wr="wisply.graph.v3:",Ar=3e3,hc=900,cc=["note","project","session"],fc=10,gc=14,vc=3,pc=15;function mc(a){try{return JSON.parse(localStorage.getItem(wr+a)??"{}")}catch{return{}}}function hs(a,n){let e={};n.forEachNode((t,r)=>{e[t]=[Math.round(r.x*10)/10,Math.round(r.y*10)/10]});try{localStorage.setItem(wr+a,JSON.stringify(e))}catch{localStorage.removeItem(wr+a)}}function bc(a,n){let e=0;return a.forEachNode((t,r)=>{let i=n[t];if(i){r.x=i[0],r.y=i[1];return}e++;let o=a.neighbors(t).map(s=>n[s]).filter(s=>!!s);o.length&&(r.x=o.reduce((s,l)=>s+l[0],0)/o.length+(Math.random()-.5)*4,r.y=o.reduce((s,l)=>s+l[1],0)/o.length+(Math.random()-.5)*4)}),e}function yc(a){let n=!1;return()=>{n||(n=!0,requestAnimationFrame(()=>{n=!1,a()}))}}function _c(a){let n=new Map(a.nodes().map(e=>[e,e]));for(let e=0;e<pc;e++){let t=!1;if(a.forEachNode(r=>{let i=new Map;a.forEachNeighbor(r,l=>{let u=n.get(l);i.set(u,(i.get(u)??0)+1)});let o=n.get(r),s=0;for(let[l,u]of i)(u>s||u===s&&l<o)&&(o=l,s=u);s>0&&o!==n.get(r)&&(n.set(r,o),t=!0)}),!t)break}return n}function xc(a){let n=new ve({type:"undirected",multi:!1,allowSelfLoops:!1}),e=new Array(a.nodes.length).fill(0);for(let[u,d]of a.edges)e[u]++,e[d]++;a.nodes.forEach(([u,d,h,c,f],g)=>{let b=h;n.mergeNode(u,{label:d||u,kind:b,group:c,cluster:"",t:f,x:(Math.random()-.5)*100,y:(Math.random()-.5)*100,size:b==="project"?3+Math.sqrt(e[g])*.5:1.2+Math.sqrt(e[g])*.5,color:Dr("--g-other"),deg:e[g]})});for(let[u,d]of a.edges){let h=a.nodes[u][0],c=a.nodes[d][0];h!==c&&!n.hasEdge(h,c)&&n.addEdge(h,c)}let r=new Set(a.nodes.map(u=>u[3]).filter(Boolean)).size>=2?null:_c(n);n.forEachNode((u,d)=>{d.cluster=r?r.get(u):d.group});let i=new Map;n.forEachNode((u,d)=>i.set(d.cluster,(i.get(d.cluster)??0)+1));let o=[...i.entries()].filter(([u,d])=>u&&d>=vc).sort((u,d)=>d[1]-u[1]).slice(0,ds.length),s=new Map,l=new Map;return o.forEach(([u],d)=>{if(s.set(u,ds[d]),r){let h=n.filterNodes((c,f)=>f.cluster===u).sort((c,f)=>n.getNodeAttribute(f,"deg")-n.getNodeAttribute(c,"deg"))[0];l.set(u,n.getNodeAttribute(h,"label"))}else l.set(u,u)}),n.forEachNode((u,d)=>{d.color=s.get(d.cluster)??Dr("--g-other")}),{g:n,colors:s,names:l}}async function Pf(a,n){let[e,t,r,i,o,s,l]=["--g-label","--g-dim","--g-edge","--g-edge-on","--g-back","--g-back-line","--g-shadow"].map(Dr);on(a,re("div",{class:"empty",style:"margin:24px"},me.graph.loading));let u=await Rr("graph",...n?[n]:[]),{g:d,colors:h,names:c}=xc(u),f=n??"all",g=bc(d,mc(f)),b=re("div",{class:"canvas"}),m=re("span",{class:"info"},me.graph.nodes(d.order,d.size)),v=re("aside",{}),p=new Set,y=!1,T=new Set(d.nodes().sort((V,te)=>d.getNodeAttribute(te,"deg")-d.getNodeAttribute(V,"deg")).slice(0,gc)),_=null,S=null,D=d.mapNodes((V,te)=>te.t).filter(V=>V>0),[A,E]=D.length?[Math.min(...D),Math.max(...D)]:[0,0],L=E,z=V=>!p.has(V.kind)&&(y||V.deg>0)&&(V.t===0||V.t<=L),C=()=>_??S,N=new Set,k=()=>{let V=C();N=V?new Set(d.neighbors(V)):new Set},F=null,M=0;g&&d.order<=Ar?(Ln.default.assign(d,{iterations:g===d.order?400:60,settings:{...(0,Ln.inferSettings)(d),scalingRatio:14,gravity:.6,slowDown:2}}),hs(f,d)):g&&(m.textContent=me.graph.layout,F=new cs.default(d,{settings:{...(0,Ln.inferSettings)(d),barnesHutOptimize:!0,slowDown:2}}),F.start(),M=window.setTimeout(()=>{F?.stop(),hs(f,d),m.textContent=me.graph.nodes(d.order,d.size)},g===d.order?Math.min(8e3,1500+d.order/3):hc));let I=V=>re("span",{class:"swatch",style:`background:${d.getNodeAttribute(V,"color")}`}),x=V=>re("a",{href:"#",onclick:te=>{te.preventDefault(),H(V)}},I(V),d.getNodeAttribute(V,"label")),P=V=>{let te=d.getNodeAttributes(V);te.kind==="note"?Ra(`#/n/${V.slice(1)}`):te.kind==="project"&&Ra(`#/p/${Lr(te.group)}`)},W=re("div",{class:"legend"},cc.map(V=>re("label",{},re("input",{type:"checkbox",checked:!0,onchange:te=>{te.target.checked?p.delete(V):p.add(V),Re()}}),me.graph.kinds[V])),re("label",{},re("input",{type:"checkbox",checked:y,onchange:V=>{y=V.target.checked,Re()}}),me.graph.orphans)),G=()=>{let V=d.filterNodes((ue,pe)=>pe.kind==="note").sort((ue,pe)=>d.getNodeAttribute(pe,"deg")-d.getNodeAttribute(ue,"deg")).slice(0,fc),te=[...h.keys()].map(ue=>{let pe=d.findNode((se,ee)=>ee.cluster===ue&&ee.label===c.get(ue))??d.findNode((se,ee)=>ee.cluster===ue),Le=d.filterNodes((se,ee)=>ee.cluster===ue).length;return re("a",{href:"#",onclick:se=>{se.preventDefault(),pe&&H(pe)}},re("span",{class:"swatch",style:`background:${h.get(ue)}`}),c.get(ue)??ue,re("span",{class:"faint"},` \xB7 ${Le}`))});on(v,re("h2",{style:"margin-top:0"},me.graph.show),W,te.length?re("h2",{},me.graph.groups):null,...te,re("h2",{},me.graph.hubs),...V.map(x))},R=V=>{let te=d.getNodeAttributes(V),ue=d.neighbors(V).sort((pe,Le)=>d.getNodeAttribute(Le,"deg")-d.getNodeAttribute(pe,"deg"));on(v,re("div",{class:"detail"},re("button",{class:"back",onclick:()=>H(null)},`\u2190 ${me.graph.back}`),re("h3",{},te.label),re("div",{class:"meta"},[me.graph.kinds[te.kind],te.t?me.ago(Cr(te.t)):"",h.has(te.cluster)?c.get(te.cluster):""].filter(Boolean).join(" \xB7 ")),re("div",{class:"actions"},te.kind==="session"?null:re("button",{class:"primary",onclick:()=>P(V)},me.graph.open),re("button",{onclick:()=>j(V)},me.graph.center)),re("h2",{},me.graph.connections,re("span",{class:"n"},ue.length)),ue.length?ue.map(x):re("div",{class:"faint"},me.graph.alone)))},j=V=>{let te=q.getNodeDisplayData(V);te&&q.getCamera().animate({x:te.x,y:te.y,ratio:.3},{duration:350})},H=V=>{_=V,k(),Re(),V?(R(V),j(V)):G()},Y=re("input",{type:"search",class:"find",placeholder:me.graph.find,onkeydown:V=>{if(V.key!=="Enter")return;let te=V.target.value.trim().toLowerCase();if(!te)return H(null);let ue=d.filterNodes((pe,Le)=>Le.label.toLowerCase().includes(te));ue.sort((pe,Le)=>+(d.getNodeAttribute(Le,"kind")==="project")-+(d.getNodeAttribute(pe,"kind")==="project")||d.getNodeAttribute(Le,"deg")-d.getNodeAttribute(pe,"deg")),ue.length&&H(ue[0])}}),Te=re("span",{},me.graph.all),de=re("input",{type:"range",min:0,max:1e3,value:1e3,"aria-label":me.graph.until,oninput:V=>{let te=Number(V.target.value)/1e3;L=A+(E-A)*te,Te.textContent=te>=1?me.graph.all:`${me.graph.until} ${new Date(L*1e3).toLocaleDateString("en-US",{year:"numeric",month:"short",day:"numeric"})}`,Re()}});on(a,re("div",{class:"graph"},re("div",{class:"stage"},b,re("div",{class:"tools"},Y,m),re("div",{class:"hint-box"},me.graph.focus),E>A?re("div",{class:"time"},de,Te):null),v)),G();let q=new us(d,b,{settings:{hideEdgesOnMove:d.order>Ar,hideLabelsOnMove:d.order>Ar,labelRenderedSizeThreshold:7,labelDensity:.35,labelGridCellSize:110,minEdgeThickness:.8,zoomToSizeRatioFunction:()=>1},nodeReducer:(V,te,ue)=>{let pe={labelColor:e,labelFont:dc,labelSize:13,backdropColor:o,backdropBorderColor:s,backdropBorderWidth:1,backdropCornerRadius:6,backdropShadowColor:l,cursor:"pointer"};return z(ue)?C()&&V!==C()&&!N.has(V)?(pe.color=t,pe.labelVisibility="hidden"):C()?(pe.labelVisibility="visible",pe.backdropVisibility="visible"):T.has(V)&&(pe.labelVisibility="visible"):pe.visibility="hidden",V===_&&(pe.backdropVisibility="visible"),pe},edgeReducer:(V,te,ue,pe,Le,se)=>{let[ee,ie]=se.extremities(V);if(!z(se.getNodeAttributes(ee))||!z(se.getNodeAttributes(ie)))return{visibility:"hidden"};let Fe=C();return Fe?ee===Fe||ie===Fe?{color:i,size:1.6}:{color:t}:{color:r}}}),Re=yc(()=>q.refresh({skipIndexation:!0}));return q.on("enterNode",({node:V})=>{S=V,k(),Re()}),q.on("leaveNode",()=>{S=null,k(),Re()}),q.on("clickNode",({node:V})=>H(V===_?null:V)),q.on("clickStage",()=>H(null)),q.on("doubleClickNode",({node:V})=>P(V)),()=>{clearTimeout(M),F?.kill(),q.kill()}}export{Pf as graph};
