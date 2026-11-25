import{a as ar,b as Je,d as rr,h as Q,k as Nt,l as ir,m as aa,n as or,p as se}from"./chunk-MIJXN42U.js";var ct=ar((wh,ra)=>{"use strict";var dt=typeof Reflect=="object"?Reflect:null,sr=dt&&typeof dt.apply=="function"?dt.apply:function(n,t,e){return Function.prototype.apply.call(n,t,e)},un;dt&&typeof dt.ownKeys=="function"?un=dt.ownKeys:Object.getOwnPropertySymbols?un=function(n){return Object.getOwnPropertyNames(n).concat(Object.getOwnPropertySymbols(n))}:un=function(n){return Object.getOwnPropertyNames(n)};function es(a){console&&console.warn&&console.warn(a)}var ur=Number.isNaN||function(n){return n!==n};function ie(){ie.init.call(this)}ra.exports=ie;ra.exports.once=rs;ie.EventEmitter=ie;ie.prototype._events=void 0;ie.prototype._eventsCount=0;ie.prototype._maxListeners=void 0;var lr=10;function dn(a){if(typeof a!="function")throw new TypeError('The "listener" argument must be of type Function. Received type '+typeof a)}Object.defineProperty(ie,"defaultMaxListeners",{enumerable:!0,get:function(){return lr},set:function(a){if(typeof a!="number"||a<0||ur(a))throw new RangeError('The value of "defaultMaxListeners" is out of range. It must be a non-negative number. Received '+a+".");lr=a}});ie.init=function(){(this._events===void 0||this._events===Object.getPrototypeOf(this)._events)&&(this._events=Object.create(null),this._eventsCount=0),this._maxListeners=this._maxListeners||void 0};ie.prototype.setMaxListeners=function(n){if(typeof n!="number"||n<0||ur(n))throw new RangeError('The value of "n" is out of range. It must be a non-negative number. Received '+n+".");return this._maxListeners=n,this};function dr(a){return a._maxListeners===void 0?ie.defaultMaxListeners:a._maxListeners}ie.prototype.getMaxListeners=function(){return dr(this)};ie.prototype.emit=function(n){for(var t=[],e=1;e<arguments.length;e++)t.push(arguments[e]);var r=n==="error",i=this._events;if(i!==void 0)r=r&&i.error===void 0;else if(!r)return!1;if(r){var o;if(t.length>0&&(o=t[0]),o instanceof Error)throw o;var s=new Error("Unhandled error."+(o?" ("+o.message+")":""));throw s.context=o,s}var l=i[n];if(l===void 0)return!1;if(typeof l=="function")sr(l,this,t);else for(var u=l.length,d=pr(l,u),e=0;e<u;++e)sr(d[e],this,t);return!0};function cr(a,n,t,e){var r,i,o;if(dn(t),i=a._events,i===void 0?(i=a._events=Object.create(null),a._eventsCount=0):(i.newListener!==void 0&&(a.emit("newListener",n,t.listener?t.listener:t),i=a._events),o=i[n]),o===void 0)o=i[n]=t,++a._eventsCount;else if(typeof o=="function"?o=i[n]=e?[t,o]:[o,t]:e?o.unshift(t):o.push(t),r=dr(a),r>0&&o.length>r&&!o.warned){o.warned=!0;var s=new Error("Possible EventEmitter memory leak detected. "+o.length+" "+String(n)+" listeners added. Use emitter.setMaxListeners() to increase limit");s.name="MaxListenersExceededWarning",s.emitter=a,s.type=n,s.count=o.length,es(s)}return a}ie.prototype.addListener=function(n,t){return cr(this,n,t,!1)};ie.prototype.on=ie.prototype.addListener;ie.prototype.prependListener=function(n,t){return cr(this,n,t,!0)};function ts(){if(!this.fired)return this.target.removeListener(this.type,this.wrapFn),this.fired=!0,arguments.length===0?this.listener.call(this.target):this.listener.apply(this.target,arguments)}function hr(a,n,t){var e={fired:!1,wrapFn:void 0,target:a,type:n,listener:t},r=ts.bind(e);return r.listener=t,e.wrapFn=r,r}ie.prototype.once=function(n,t){return dn(t),this.on(n,hr(this,n,t)),this};ie.prototype.prependOnceListener=function(n,t){return dn(t),this.prependListener(n,hr(this,n,t)),this};ie.prototype.removeListener=function(n,t){var e,r,i,o,s;if(dn(t),r=this._events,r===void 0)return this;if(e=r[n],e===void 0)return this;if(e===t||e.listener===t)--this._eventsCount===0?this._events=Object.create(null):(delete r[n],r.removeListener&&this.emit("removeListener",n,e.listener||t));else if(typeof e!="function"){for(i=-1,o=e.length-1;o>=0;o--)if(e[o]===t||e[o].listener===t){s=e[o].listener,i=o;break}if(i<0)return this;i===0?e.shift():ns(e,i),e.length===1&&(r[n]=e[0]),r.removeListener!==void 0&&this.emit("removeListener",n,s||t)}return this};ie.prototype.off=ie.prototype.removeListener;ie.prototype.removeAllListeners=function(n){var t,e,r;if(e=this._events,e===void 0)return this;if(e.removeListener===void 0)return arguments.length===0?(this._events=Object.create(null),this._eventsCount=0):e[n]!==void 0&&(--this._eventsCount===0?this._events=Object.create(null):delete e[n]),this;if(arguments.length===0){var i=Object.keys(e),o;for(r=0;r<i.length;++r)o=i[r],o!=="removeListener"&&this.removeAllListeners(o);return this.removeAllListeners("removeListener"),this._events=Object.create(null),this._eventsCount=0,this}if(t=e[n],typeof t=="function")this.removeListener(n,t);else if(t!==void 0)for(r=t.length-1;r>=0;r--)this.removeListener(n,t[r]);return this};function fr(a,n,t){var e=a._events;if(e===void 0)return[];var r=e[n];return r===void 0?[]:typeof r=="function"?t?[r.listener||r]:[r]:t?as(r):pr(r,r.length)}ie.prototype.listeners=function(n){return fr(this,n,!0)};ie.prototype.rawListeners=function(n){return fr(this,n,!1)};ie.listenerCount=function(a,n){return typeof a.listenerCount=="function"?a.listenerCount(n):gr.call(a,n)};ie.prototype.listenerCount=gr;function gr(a){var n=this._events;if(n!==void 0){var t=n[a];if(typeof t=="function")return 1;if(t!==void 0)return t.length}return 0}ie.prototype.eventNames=function(){return this._eventsCount>0?un(this._events):[]};function pr(a,n){for(var t=new Array(n),e=0;e<n;++e)t[e]=a[e];return t}function ns(a,n){for(;n+1<a.length;n++)a[n]=a[n+1];a.pop()}function as(a){for(var n=new Array(a.length),t=0;t<n.length;++t)n[t]=a[t].listener||a[t];return n}function rs(a,n){return new Promise(function(t,e){function r(o){a.removeListener(n,i),e(o)}function i(){typeof a.removeListener=="function"&&a.removeListener("error",r),t([].slice.call(arguments))}vr(a,n,i,{once:!0}),n!=="error"&&is(a,r,{once:!0})})}function is(a,n,t){typeof a.on=="function"&&vr(a,"error",n,t)}function vr(a,n,t,e){if(typeof a.on=="function")e.once?a.once(n,t):a.on(n,t);else if(typeof a.addEventListener=="function")a.addEventListener(n,function r(i){e.once&&a.removeEventListener(n,r),t(i)});else throw new TypeError('The "emitter" argument must be of type EventEmitter. Received type '+typeof a)}});var La=ar((qh,Ri)=>{Ri.exports=function(n){return n!==null&&typeof n=="object"&&typeof n.addUndirectedEdgeWithKey=="function"&&typeof n.dropNode=="function"&&typeof n.multi=="boolean"}});var xr=Je(ct(),1);function os(){let a=arguments[0];for(let n=1,t=arguments.length;n<t;n++)if(arguments[n])for(let e in arguments[n])a[e]=arguments[n][e];return a}var fe=os;typeof Object.assign=="function"&&(fe=Object.assign);function Ae(a,n,t,e){let r=a._nodes.get(n),i=null;return r&&(e==="mixed"?i=r.out&&r.out[t]||r.undirected&&r.undirected[t]:e==="directed"?i=r.out&&r.out[t]:i=r.undirected&&r.undirected[t]),i}function pe(a){return typeof a=="object"&&a!==null}function _r(a){let n;for(n in a)return!1;return!0}function Ee(a,n,t){Object.defineProperty(a,n,{enumerable:!1,configurable:!1,writable:!0,value:t})}function Ce(a,n,t){let e={enumerable:!0,configurable:!0};typeof t=="function"?e.get=t:(e.value=t,e.writable=!1),Object.defineProperty(a,n,e)}function mr(a){return!(!pe(a)||a.attributes&&!Array.isArray(a.attributes))}function ss(){let a=Math.floor(Math.random()*256)&255;return()=>a++}function Me(){let a=arguments,n=null,t=-1;return{[Symbol.iterator](){return this},next(){let e=null;do{if(n===null){if(t++,t>=a.length)return{done:!0};n=a[t][Symbol.iterator]()}if(e=n.next(),e.done){n=null;continue}break}while(!0);return e}}}function ft(){return{[Symbol.iterator](){return this},next(){return{done:!0}}}}var zt=class extends Error{constructor(n){super(),this.name="GraphError",this.message=n}},O=class a extends zt{constructor(n){super(n),this.name="InvalidArgumentsGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},W=class a extends zt{constructor(n){super(n),this.name="NotFoundGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}},X=class a extends zt{constructor(n){super(n),this.name="UsageGraphError",typeof Error.captureStackTrace=="function"&&Error.captureStackTrace(this,a.prototype.constructor)}};function Tr(a,n){this.key=a,this.attributes=n,this.clear()}Tr.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.undirectedDegree=0,this.undirectedLoops=0,this.directedLoops=0,this.in={},this.out={},this.undirected={}};function Sr(a,n){this.key=a,this.attributes=n,this.clear()}Sr.prototype.clear=function(){this.inDegree=0,this.outDegree=0,this.directedLoops=0,this.in={},this.out={}};function Er(a,n){this.key=a,this.attributes=n,this.clear()}Er.prototype.clear=function(){this.undirectedDegree=0,this.undirectedLoops=0,this.undirected={}};function gt(a,n,t,e,r){this.key=n,this.attributes=r,this.undirected=a,this.source=t,this.target=e}gt.prototype.attach=function(){let a="out",n="in";this.undirected&&(a=n="undirected");let t=this.source.key,e=this.target.key;this.source[a][e]=this,!(this.undirected&&t===e)&&(this.target[n][t]=this)};gt.prototype.attachMulti=function(){let a="out",n="in",t=this.source.key,e=this.target.key;this.undirected&&(a=n="undirected");let r=this.source[a],i=r[e];if(typeof i>"u"){r[e]=this,this.undirected&&t===e||(this.target[n][t]=this);return}i.previous=this,this.next=i,r[e]=this,this.target[n][t]=this};gt.prototype.detach=function(){let a=this.source.key,n=this.target.key,t="out",e="in";this.undirected&&(t=e="undirected"),delete this.source[t][n],delete this.target[e][a]};gt.prototype.detachMulti=function(){let a=this.source.key,n=this.target.key,t="out",e="in";this.undirected&&(t=e="undirected"),this.previous===void 0?this.next===void 0?(delete this.source[t][n],delete this.target[e][a]):(this.next.previous=void 0,this.source[t][n]=this.next,this.target[e][a]=this.next):(this.previous.next=this.next,this.next!==void 0&&(this.next.previous=this.previous))};var Ar=0,wr=1,ls=2,Dr=3;function ze(a,n,t,e,r,i,o){let s,l,u,d;if(e=""+e,t===Ar){if(s=a._nodes.get(e),!s)throw new W(`Graph.${n}: could not find the "${e}" node in the graph.`);u=r,d=i}else if(t===Dr){if(r=""+r,l=a._edges.get(r),!l)throw new W(`Graph.${n}: could not find the "${r}" edge in the graph.`);let c=l.source.key,h=l.target.key;if(e===c)s=l.target;else if(e===h)s=l.source;else throw new W(`Graph.${n}: the "${e}" node is not attached to the "${r}" edge (${c}, ${h}).`);u=i,d=o}else{if(l=a._edges.get(e),!l)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`);t===wr?s=l.source:s=l.target,u=r,d=i}return[s,u,d]}function us(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);return o.attributes[s]}}function ds(a,n,t){a.prototype[n]=function(e,r){let[i]=ze(this,n,t,e,r);return i.attributes}}function cs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);return o.attributes.hasOwnProperty(s)}}function hs(a,n,t){a.prototype[n]=function(e,r,i,o){let[s,l,u]=ze(this,n,t,e,r,i,o);return s.attributes[l]=u,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function fs(a,n,t){a.prototype[n]=function(e,r,i,o){let[s,l,u]=ze(this,n,t,e,r,i,o);if(typeof u!="function")throw new O(`Graph.${n}: updater should be a function.`);let d=s.attributes,c=u(d[l]);return d[l]=c,this.emit("nodeAttributesUpdated",{key:s.key,type:"set",attributes:s.attributes,name:l}),this}}function gs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);return delete o.attributes[s],this.emit("nodeAttributesUpdated",{key:o.key,type:"remove",attributes:o.attributes,name:s}),this}}function ps(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);if(!pe(s))throw new O(`Graph.${n}: provided attributes are not a plain object.`);return o.attributes=s,this.emit("nodeAttributesUpdated",{key:o.key,type:"replace",attributes:o.attributes}),this}}function vs(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);if(!pe(s))throw new O(`Graph.${n}: provided attributes are not a plain object.`);return fe(o.attributes,s),this.emit("nodeAttributesUpdated",{key:o.key,type:"merge",attributes:o.attributes,data:s}),this}}function ms(a,n,t){a.prototype[n]=function(e,r,i){let[o,s]=ze(this,n,t,e,r,i);if(typeof s!="function")throw new O(`Graph.${n}: provided updater is not a function.`);return o.attributes=s(o.attributes),this.emit("nodeAttributesUpdated",{key:o.key,type:"update",attributes:o.attributes}),this}}var bs=[{name:a=>`get${a}Attribute`,attacher:us},{name:a=>`get${a}Attributes`,attacher:ds},{name:a=>`has${a}Attribute`,attacher:cs},{name:a=>`set${a}Attribute`,attacher:hs},{name:a=>`update${a}Attribute`,attacher:fs},{name:a=>`remove${a}Attribute`,attacher:gs},{name:a=>`replace${a}Attributes`,attacher:ps},{name:a=>`merge${a}Attributes`,attacher:vs},{name:a=>`update${a}Attributes`,attacher:ms}];function ys(a){bs.forEach(function({name:n,attacher:t}){t(a,n("Node"),Ar),t(a,n("Source"),wr),t(a,n("Target"),ls),t(a,n("Opposite"),Dr)})}function xs(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return i.attributes[r]}}function _s(a,n,t){a.prototype[n]=function(e){let r;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>1){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let i=""+e,o=""+arguments[1];if(r=Ae(this,i,o,t),!r)throw new W(`Graph.${n}: could not find an edge for the given path ("${i}" - "${o}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,r=this._edges.get(e),!r)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return r.attributes}}function Ts(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return i.attributes.hasOwnProperty(r)}}function Ss(a,n,t){a.prototype[n]=function(e,r,i){let o;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+e,l=""+r;if(r=arguments[2],i=arguments[3],o=Ae(this,s,l,t),!o)throw new W(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,o=this._edges.get(e),!o)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return o.attributes[r]=i,this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function Es(a,n,t){a.prototype[n]=function(e,r,i){let o;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>3){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let s=""+e,l=""+r;if(r=arguments[2],i=arguments[3],o=Ae(this,s,l,t),!o)throw new W(`Graph.${n}: could not find an edge for the given path ("${s}" - "${l}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,o=this._edges.get(e),!o)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(typeof i!="function")throw new O(`Graph.${n}: updater should be a function.`);return o.attributes[r]=i(o.attributes[r]),this.emit("edgeAttributesUpdated",{key:o.key,type:"set",attributes:o.attributes,name:r}),this}}function As(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}return delete i.attributes[r],this.emit("edgeAttributesUpdated",{key:i.key,type:"remove",attributes:i.attributes,name:r}),this}}function ws(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(!pe(r))throw new O(`Graph.${n}: provided attributes are not a plain object.`);return i.attributes=r,this.emit("edgeAttributesUpdated",{key:i.key,type:"replace",attributes:i.attributes}),this}}function Ds(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(!pe(r))throw new O(`Graph.${n}: provided attributes are not a plain object.`);return fe(i.attributes,r),this.emit("edgeAttributesUpdated",{key:i.key,type:"merge",attributes:i.attributes,data:r}),this}}function Rs(a,n,t){a.prototype[n]=function(e,r){let i;if(this.type!=="mixed"&&t!=="mixed"&&t!==this.type)throw new X(`Graph.${n}: cannot find this type of edges in your ${this.type} graph.`);if(arguments.length>2){if(this.multi)throw new X(`Graph.${n}: cannot use a {source,target} combo when asking about an edge's attributes in a MultiGraph since we cannot infer the one you want information about.`);let o=""+e,s=""+r;if(r=arguments[2],i=Ae(this,o,s,t),!i)throw new W(`Graph.${n}: could not find an edge for the given path ("${o}" - "${s}").`)}else{if(t!=="mixed")throw new X(`Graph.${n}: calling this method with only a key (vs. a source and target) does not make sense since an edge with this key could have the other type.`);if(e=""+e,i=this._edges.get(e),!i)throw new W(`Graph.${n}: could not find the "${e}" edge in the graph.`)}if(typeof r!="function")throw new O(`Graph.${n}: provided updater is not a function.`);return i.attributes=r(i.attributes),this.emit("edgeAttributesUpdated",{key:i.key,type:"update",attributes:i.attributes}),this}}var Ls=[{name:a=>`get${a}Attribute`,attacher:xs},{name:a=>`get${a}Attributes`,attacher:_s},{name:a=>`has${a}Attribute`,attacher:Ts},{name:a=>`set${a}Attribute`,attacher:Ss},{name:a=>`update${a}Attribute`,attacher:Es},{name:a=>`remove${a}Attribute`,attacher:As},{name:a=>`replace${a}Attributes`,attacher:ws},{name:a=>`merge${a}Attributes`,attacher:Ds},{name:a=>`update${a}Attributes`,attacher:Rs}];function Cs(a){Ls.forEach(function({name:n,attacher:t}){t(a,n("Edge"),"mixed"),t(a,n("DirectedEdge"),"directed"),t(a,n("UndirectedEdge"),"undirected")})}var ks=[{name:"edges",type:"mixed"},{name:"inEdges",type:"directed",direction:"in"},{name:"outEdges",type:"directed",direction:"out"},{name:"inboundEdges",type:"mixed",direction:"in"},{name:"outboundEdges",type:"mixed",direction:"out"},{name:"directedEdges",type:"directed"},{name:"undirectedEdges",type:"undirected"}];function Ps(a,n,t,e){let r=!1;for(let i in n){if(i===e)continue;let o=n[i];if(r=t(o.key,o.attributes,o.source.key,o.target.key,o.source.attributes,o.target.attributes,o.undirected),a&&r)return o.key}}function Fs(a,n,t,e){let r,i,o,s=!1;for(let l in n)if(l!==e){r=n[l];do{if(i=r.source,o=r.target,s=t(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected),a&&s)return r.key;r=r.next}while(r!==void 0)}}function ia(a,n){let t=Object.keys(a),e=t.length,r,i=0;return{[Symbol.iterator](){return this},next(){do if(r)r=r.next;else{if(i>=e)return{done:!0};let o=t[i++];if(o===n){r=void 0;continue}r=a[o]}while(!r);return{done:!1,value:{edge:r.key,attributes:r.attributes,source:r.source.key,target:r.target.key,sourceAttributes:r.source.attributes,targetAttributes:r.target.attributes,undirected:r.undirected}}}}}function Is(a,n,t,e){let r=n[t];if(!r)return;let i=r.source,o=r.target;if(e(r.key,r.attributes,i.key,o.key,i.attributes,o.attributes,r.undirected)&&a)return r.key}function Ns(a,n,t,e){let r=n[t];if(!r)return;let i=!1;do{if(i=e(r.key,r.attributes,r.source.key,r.target.key,r.source.attributes,r.target.attributes,r.undirected),a&&i)return r.key;r=r.next}while(r!==void 0)}function oa(a,n){let t=a[n];if(t.next!==void 0)return{[Symbol.iterator](){return this},next(){if(!t)return{done:!0};let r={edge:t.key,attributes:t.attributes,source:t.source.key,target:t.target.key,sourceAttributes:t.source.attributes,targetAttributes:t.target.attributes,undirected:t.undirected};return t=t.next,{done:!1,value:r}}};let e=!1;return{[Symbol.iterator](){return this},next(){return e===!0?{done:!0}:(e=!0,{done:!1,value:{edge:t.key,attributes:t.attributes,source:t.source.key,target:t.target.key,sourceAttributes:t.source.attributes,targetAttributes:t.target.attributes,undirected:t.undirected}})}}}function Gs(a,n){if(a.size===0)return[];if(n==="mixed"||n===a.type)return Array.from(a._edges.keys());let t=n==="undirected"?a.undirectedSize:a.directedSize,e=new Array(t),r=n==="undirected",i=a._edges.values(),o=0,s,l;for(;s=i.next(),s.done!==!0;)l=s.value,l.undirected===r&&(e[o++]=l.key);return e}function Rr(a,n,t,e){if(n.size===0)return;let r=t!=="mixed"&&t!==n.type,i=t==="undirected",o,s,l=!1,u=n._edges.values();for(;o=u.next(),o.done!==!0;){if(s=o.value,r&&s.undirected!==i)continue;let{key:d,attributes:c,source:h,target:f}=s;if(l=e(d,c,h.key,f.key,h.attributes,f.attributes,s.undirected),a&&l)return d}}function Ms(a,n){if(a.size===0)return ft();let t=n!=="mixed"&&n!==a.type,e=n==="undirected",r=a._edges.values();return{[Symbol.iterator](){return this},next(){let i,o;for(;;){if(i=r.next(),i.done)return i;if(o=i.value,!(t&&o.undirected!==e))break}return{value:{edge:o.key,attributes:o.attributes,source:o.source.key,target:o.target.key,sourceAttributes:o.source.attributes,targetAttributes:o.target.attributes,undirected:o.undirected},done:!1}}}}function sa(a,n,t,e,r,i){let o=n?Fs:Ps,s;if(t!=="undirected"&&(e!=="out"&&(s=o(a,r.in,i),a&&s)||e!=="in"&&(s=o(a,r.out,i,e?void 0:r.key),a&&s))||t!=="directed"&&(s=o(a,r.undirected,i),a&&s))return s}function zs(a,n,t,e){let r=[];return sa(!1,a,n,t,e,function(i){r.push(i)}),r}function Bs(a,n,t){let e=ft();return a!=="undirected"&&(n!=="out"&&typeof t.in<"u"&&(e=Me(e,ia(t.in))),n!=="in"&&typeof t.out<"u"&&(e=Me(e,ia(t.out,n?void 0:t.key)))),a!=="directed"&&typeof t.undirected<"u"&&(e=Me(e,ia(t.undirected))),e}function la(a,n,t,e,r,i,o){let s=t?Ns:Is,l;if(n!=="undirected"&&(typeof r.in<"u"&&e!=="out"&&(l=s(a,r.in,i,o),a&&l)||typeof r.out<"u"&&e!=="in"&&(e||r.key!==i)&&(l=s(a,r.out,i,o),a&&l))||n!=="directed"&&typeof r.undirected<"u"&&(l=s(a,r.undirected,i,o),a&&l))return l}function Ws(a,n,t,e,r){let i=[];return la(!1,a,n,t,e,r,function(o){i.push(o)}),i}function Us(a,n,t,e){let r=ft();return a!=="undirected"&&(typeof t.in<"u"&&n!=="out"&&e in t.in&&(r=Me(r,oa(t.in,e))),typeof t.out<"u"&&n!=="in"&&e in t.out&&(n||t.key!==e)&&(r=Me(r,oa(t.out,e)))),a!=="directed"&&typeof t.undirected<"u"&&e in t.undirected&&(r=Me(r,oa(t.undirected,e))),r}function Os(a,n){let{name:t,type:e,direction:r}=n;a.prototype[t]=function(i,o){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return[];if(!arguments.length)return Gs(this,e);if(arguments.length===1){i=""+i;let s=this._nodes.get(i);if(typeof s>"u")throw new W(`Graph.${t}: could not find the "${i}" node in the graph.`);return zs(this.multi,e==="mixed"?this.type:e,r,s)}if(arguments.length===2){i=""+i,o=""+o;let s=this._nodes.get(i);if(!s)throw new W(`Graph.${t}:  could not find the "${i}" source node in the graph.`);if(!this._nodes.has(o))throw new W(`Graph.${t}:  could not find the "${o}" target node in the graph.`);return Ws(e,this.multi,r,s,o)}throw new O(`Graph.${t}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function Hs(a,n){let{name:t,type:e,direction:r}=n,i="forEach"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(u,d,c){if(!(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)){if(arguments.length===1)return c=u,Rr(!1,this,e,c);if(arguments.length===2){u=""+u,c=d;let h=this._nodes.get(u);if(typeof h>"u")throw new W(`Graph.${i}: could not find the "${u}" node in the graph.`);return sa(!1,this.multi,e==="mixed"?this.type:e,r,h,c)}if(arguments.length===3){u=""+u,d=""+d;let h=this._nodes.get(u);if(!h)throw new W(`Graph.${i}:  could not find the "${u}" source node in the graph.`);if(!this._nodes.has(d))throw new W(`Graph.${i}:  could not find the "${d}" target node in the graph.`);return la(!1,e,this.multi,r,h,d,c)}throw new O(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)}};let o="map"+t[0].toUpperCase()+t.slice(1);a.prototype[o]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),c;if(u.length===0){let h=0;e!=="directed"&&(h+=this.undirectedSize),e!=="undirected"&&(h+=this.directedSize),c=new Array(h);let f=0;u.push((g,m,v,p,b,y,T)=>{c[f++]=d(g,m,v,p,b,y,T)})}else c=[],u.push((h,f,g,m,v,p,b)=>{c.push(d(h,f,g,m,v,p,b))});return this[i].apply(this,u),c};let s="filter"+t[0].toUpperCase()+t.slice(1);a.prototype[s]=function(){let u=Array.prototype.slice.call(arguments),d=u.pop(),c=[];return u.push((h,f,g,m,v,p,b)=>{d(h,f,g,m,v,p,b)&&c.push(h)}),this[i].apply(this,u),c};let l="reduce"+t[0].toUpperCase()+t.slice(1);a.prototype[l]=function(){let u=Array.prototype.slice.call(arguments);if(u.length<2||u.length>4)throw new O(`Graph.${l}: invalid number of arguments (expecting 2, 3 or 4 and got ${u.length}).`);if(typeof u[u.length-1]=="function"&&typeof u[u.length-2]!="function")throw new O(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let d,c;u.length===2?(d=u[0],c=u[1],u=[]):u.length===3?(d=u[1],c=u[2],u=[u[0]]):u.length===4&&(d=u[2],c=u[3],u=[u[0],u[1]]);let h=c;return u.push((f,g,m,v,p,b,y)=>{h=d(h,f,g,m,v,p,b,y)}),this[i].apply(this,u),h}}function $s(a,n){let{name:t,type:e,direction:r}=n,i="find"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(l,u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return!1;if(arguments.length===1)return d=l,Rr(!0,this,e,d);if(arguments.length===2){l=""+l,d=u;let c=this._nodes.get(l);if(typeof c>"u")throw new W(`Graph.${i}: could not find the "${l}" node in the graph.`);return sa(!0,this.multi,e==="mixed"?this.type:e,r,c,d)}if(arguments.length===3){l=""+l,u=""+u;let c=this._nodes.get(l);if(!c)throw new W(`Graph.${i}:  could not find the "${l}" source node in the graph.`);if(!this._nodes.has(u))throw new W(`Graph.${i}:  could not find the "${u}" target node in the graph.`);return la(!0,e,this.multi,r,c,u,d)}throw new O(`Graph.${i}: too many arguments (expecting 1, 2 or 3 and got ${arguments.length}).`)};let o="some"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[o]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((c,h,f,g,m,v,p)=>u(c,h,f,g,m,v,p)),!!this[i].apply(this,l)};let s="every"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[s]=function(){let l=Array.prototype.slice.call(arguments),u=l.pop();return l.push((c,h,f,g,m,v,p)=>!u(c,h,f,g,m,v,p)),!this[i].apply(this,l)}}function Vs(a,n){let{name:t,type:e,direction:r}=n,i=t.slice(0,-1)+"Entries";a.prototype[i]=function(o,s){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return ft();if(!arguments.length)return Ms(this,e);if(arguments.length===1){o=""+o;let l=this._nodes.get(o);if(!l)throw new W(`Graph.${i}: could not find the "${o}" node in the graph.`);return Bs(e,r,l)}if(arguments.length===2){o=""+o,s=""+s;let l=this._nodes.get(o);if(!l)throw new W(`Graph.${i}:  could not find the "${o}" source node in the graph.`);if(!this._nodes.has(s))throw new W(`Graph.${i}:  could not find the "${s}" target node in the graph.`);return Us(e,r,l,s)}throw new O(`Graph.${i}: too many arguments (expecting 0, 1 or 2 and got ${arguments.length}).`)}}function js(a){ks.forEach(n=>{Os(a,n),Hs(a,n),$s(a,n),Vs(a,n)})}var Xs=[{name:"neighbors",type:"mixed"},{name:"inNeighbors",type:"directed",direction:"in"},{name:"outNeighbors",type:"directed",direction:"out"},{name:"inboundNeighbors",type:"mixed",direction:"in"},{name:"outboundNeighbors",type:"mixed",direction:"out"},{name:"directedNeighbors",type:"directed"},{name:"undirectedNeighbors",type:"undirected"}];function mn(){this.A=null,this.B=null}mn.prototype.wrap=function(a){this.A===null?this.A=a:this.B===null&&(this.B=a)};mn.prototype.has=function(a){return this.A!==null&&a in this.A||this.B!==null&&a in this.B};function Gt(a,n,t,e,r){for(let i in e){let o=e[i],s=o.source,l=o.target,u=s===t?l:s;if(n&&n.has(u.key))continue;let d=r(u.key,u.attributes);if(a&&d)return u.key}}function ua(a,n,t,e,r){if(n!=="mixed"){if(n==="undirected")return Gt(a,null,e,e.undirected,r);if(typeof t=="string")return Gt(a,null,e,e[t],r)}let i=new mn,o;if(n!=="undirected"){if(t!=="out"){if(o=Gt(a,null,e,e.in,r),a&&o)return o;i.wrap(e.in)}if(t!=="in"){if(o=Gt(a,i,e,e.out,r),a&&o)return o;i.wrap(e.out)}}if(n!=="directed"&&(o=Gt(a,i,e,e.undirected,r),a&&o))return o}function qs(a,n,t){if(a!=="mixed"){if(a==="undirected")return Object.keys(t.undirected);if(typeof n=="string")return Object.keys(t[n])}let e=[];return ua(!1,a,n,t,function(r){e.push(r)}),e}function Mt(a,n,t){let e=Object.keys(t),r=e.length,i=0;return{[Symbol.iterator](){return this},next(){let o=null;do{if(i>=r)return a&&a.wrap(t),{done:!0};let s=t[e[i++]],l=s.source,u=s.target;if(o=l===n?u:l,a&&a.has(o.key)){o=null;continue}}while(o===null);return{done:!1,value:{neighbor:o.key,attributes:o.attributes}}}}}function Ks(a,n,t){if(a!=="mixed"){if(a==="undirected")return Mt(null,t,t.undirected);if(typeof n=="string")return Mt(null,t,t[n])}let e=ft(),r=new mn;return a!=="undirected"&&(n!=="out"&&(e=Me(e,Mt(r,t,t.in))),n!=="in"&&(e=Me(e,Mt(r,t,t.out)))),a!=="directed"&&(e=Me(e,Mt(r,t,t.undirected))),e}function Ys(a,n){let{name:t,type:e,direction:r}=n;a.prototype[t]=function(i){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return[];i=""+i;let o=this._nodes.get(i);if(typeof o>"u")throw new W(`Graph.${t}: could not find the "${i}" node in the graph.`);return qs(e==="mixed"?this.type:e,r,o)}}function Zs(a,n){let{name:t,type:e,direction:r}=n,i="forEach"+t[0].toUpperCase()+t.slice(1,-1);a.prototype[i]=function(u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return;u=""+u;let c=this._nodes.get(u);if(typeof c>"u")throw new W(`Graph.${i}: could not find the "${u}" node in the graph.`);ua(!1,e==="mixed"?this.type:e,r,c,d)};let o="map"+t[0].toUpperCase()+t.slice(1);a.prototype[o]=function(u,d){let c=[];return this[i](u,(h,f)=>{c.push(d(h,f))}),c};let s="filter"+t[0].toUpperCase()+t.slice(1);a.prototype[s]=function(u,d){let c=[];return this[i](u,(h,f)=>{d(h,f)&&c.push(h)}),c};let l="reduce"+t[0].toUpperCase()+t.slice(1);a.prototype[l]=function(u,d,c){if(arguments.length<3)throw new O(`Graph.${l}: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.`);let h=c;return this[i](u,(f,g)=>{h=d(h,f,g)}),h}}function Qs(a,n){let{name:t,type:e,direction:r}=n,i=t[0].toUpperCase()+t.slice(1,-1),o="find"+i;a.prototype[o]=function(u,d){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return;u=""+u;let c=this._nodes.get(u);if(typeof c>"u")throw new W(`Graph.${o}: could not find the "${u}" node in the graph.`);return ua(!0,e==="mixed"?this.type:e,r,c,d)};let s="some"+i;a.prototype[s]=function(u,d){return!!this[o](u,d)};let l="every"+i;a.prototype[l]=function(u,d){return!this[o](u,(h,f)=>!d(h,f))}}function Js(a,n){let{name:t,type:e,direction:r}=n,i=t.slice(0,-1)+"Entries";a.prototype[i]=function(o){if(e!=="mixed"&&this.type!=="mixed"&&e!==this.type)return ft();o=""+o;let s=this._nodes.get(o);if(typeof s>"u")throw new W(`Graph.${i}: could not find the "${o}" node in the graph.`);return Ks(e==="mixed"?this.type:e,r,s)}}function el(a){Xs.forEach(n=>{Ys(a,n),Zs(a,n),Qs(a,n),Js(a,n)})}function cn(a,n,t,e,r){let i=e._nodes.values(),o=e.type,s,l,u,d,c,h,f;for(;s=i.next(),s.done!==!0;){let g=!1;if(l=s.value,o!=="undirected"){d=l.out;for(u in d){c=d[u];do{if(h=c.target,g=!0,f=r(l.key,h.key,l.attributes,h.attributes,c.key,c.attributes,c.undirected),a&&f)return c;c=c.next}while(c)}}if(o!=="directed"){d=l.undirected;for(u in d)if(!(n&&l.key>u)){c=d[u];do{if(h=c.target,h.key!==u&&(h=c.source),g=!0,f=r(l.key,h.key,l.attributes,h.attributes,c.key,c.attributes,c.undirected),a&&f)return c;c=c.next}while(c)}}if(t&&!g&&(f=r(l.key,null,l.attributes,null,null,null,null),a&&f))return null}}function tl(a,n){let t={key:a};return _r(n.attributes)||(t.attributes=fe({},n.attributes)),t}function nl(a,n,t){let e={key:n,source:t.source.key,target:t.target.key};return _r(t.attributes)||(e.attributes=fe({},t.attributes)),a==="mixed"&&t.undirected&&(e.undirected=!0),e}function al(a){if(!pe(a))throw new O('Graph.import: invalid serialized node. A serialized node should be a plain object with at least a "key" property.');if(!("key"in a))throw new O("Graph.import: serialized node is missing its key.");if("attributes"in a&&(!pe(a.attributes)||a.attributes===null))throw new O("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.")}function rl(a){if(!pe(a))throw new O('Graph.import: invalid serialized edge. A serialized edge should be a plain object with at least a "source" & "target" property.');if(!("source"in a))throw new O("Graph.import: serialized edge is missing its source.");if(!("target"in a))throw new O("Graph.import: serialized edge is missing its target.");if("attributes"in a&&(!pe(a.attributes)||a.attributes===null))throw new O("Graph.import: invalid attributes. Attributes should be a plain object, null or omitted.");if("undirected"in a&&typeof a.undirected!="boolean")throw new O("Graph.import: invalid undirectedness information. Undirected should be boolean or omitted.")}var il=ss(),ol=new Set(["directed","undirected","mixed"]),br=new Set(["domain","_events","_eventsCount","_maxListeners"]),sl=[{name:a=>`${a}Edge`,generateKey:!0},{name:a=>`${a}DirectedEdge`,generateKey:!0,type:"directed"},{name:a=>`${a}UndirectedEdge`,generateKey:!0,type:"undirected"},{name:a=>`${a}EdgeWithKey`},{name:a=>`${a}DirectedEdgeWithKey`,type:"directed"},{name:a=>`${a}UndirectedEdgeWithKey`,type:"undirected"}],ll={allowSelfLoops:!0,multi:!1,type:"mixed"};function ul(a,n,t){if(t&&!pe(t))throw new O(`Graph.addNode: invalid attributes. Expecting an object but got "${t}"`);if(n=""+n,t=t||{},a._nodes.has(n))throw new X(`Graph.addNode: the "${n}" node already exist in the graph.`);let e=new a.NodeDataClass(n,t);return a._nodes.set(n,e),a.emit("nodeAdded",{key:n,attributes:t}),e}function yr(a,n,t){let e=new a.NodeDataClass(n,t);return a._nodes.set(n,e),a.emit("nodeAdded",{key:n,attributes:t}),e}function Lr(a,n,t,e,r,i,o,s){if(!e&&a.type==="undirected")throw new X(`Graph.${n}: you cannot add a directed edge to an undirected graph. Use the #.addEdge or #.addUndirectedEdge instead.`);if(e&&a.type==="directed")throw new X(`Graph.${n}: you cannot add an undirected edge to a directed graph. Use the #.addEdge or #.addDirectedEdge instead.`);if(s&&!pe(s))throw new O(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`);if(i=""+i,o=""+o,s=s||{},!a.allowSelfLoops&&i===o)throw new X(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let l=a._nodes.get(i),u=a._nodes.get(o);if(!l)throw new W(`Graph.${n}: source node "${i}" not found.`);if(!u)throw new W(`Graph.${n}: target node "${o}" not found.`);let d={key:null,undirected:e,source:i,target:o,attributes:s};if(t)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new X(`Graph.${n}: the "${r}" edge already exists in the graph.`);if(!a.multi&&(e?typeof l.undirected[o]<"u":typeof l.out[o]<"u"))throw new X(`Graph.${n}: an edge linking "${i}" to "${o}" already exists. If you really want to add multiple edges linking those nodes, you should create a multi graph by using the 'multi' option.`);let c=new gt(e,r,l,u,s);a._edges.set(r,c);let h=i===o;return e?(l.undirectedDegree++,u.undirectedDegree++,h&&(l.undirectedLoops++,a._undirectedSelfLoopCount++)):(l.outDegree++,u.inDegree++,h&&(l.directedLoops++,a._directedSelfLoopCount++)),a.multi?c.attachMulti():c.attach(),e?a._undirectedSize++:a._directedSize++,d.key=r,a.emit("edgeAdded",d),r}function dl(a,n,t,e,r,i,o,s,l){if(!e&&a.type==="undirected")throw new X(`Graph.${n}: you cannot merge/update a directed edge to an undirected graph. Use the #.mergeEdge/#.updateEdge or #.addUndirectedEdge instead.`);if(e&&a.type==="directed")throw new X(`Graph.${n}: you cannot merge/update an undirected edge to a directed graph. Use the #.mergeEdge/#.updateEdge or #.addDirectedEdge instead.`);if(s){if(l){if(typeof s!="function")throw new O(`Graph.${n}: invalid updater function. Expecting a function but got "${s}"`)}else if(!pe(s))throw new O(`Graph.${n}: invalid attributes. Expecting an object but got "${s}"`)}i=""+i,o=""+o;let u;if(l&&(u=s,s=void 0),!a.allowSelfLoops&&i===o)throw new X(`Graph.${n}: source & target are the same ("${i}"), thus creating a loop explicitly forbidden by this graph 'allowSelfLoops' option set to false.`);let d=a._nodes.get(i),c=a._nodes.get(o),h,f;if(!t&&(h=a._edges.get(r),h)){if((h.source.key!==i||h.target.key!==o)&&(!e||h.source.key!==o||h.target.key!==i))throw new X(`Graph.${n}: inconsistency detected when attempting to merge the "${r}" edge with "${i}" source & "${o}" target vs. ("${h.source.key}", "${h.target.key}").`);f=h}if(!f&&!a.multi&&d&&(f=e?d.undirected[o]:d.out[o]),f){let b=[f.key,!1,!1,!1];if(l?!u:!s)return b;if(l){let y=f.attributes;f.attributes=u(y),a.emit("edgeAttributesUpdated",{type:"replace",key:f.key,attributes:f.attributes})}else fe(f.attributes,s),a.emit("edgeAttributesUpdated",{type:"merge",key:f.key,attributes:f.attributes,data:s});return b}s=s||{},l&&u&&(s=u(s));let g={key:null,undirected:e,source:i,target:o,attributes:s};if(t)r=a._edgeKeyGenerator();else if(r=""+r,a._edges.has(r))throw new X(`Graph.${n}: the "${r}" edge already exists in the graph.`);let m=!1,v=!1;d||(d=yr(a,i,{}),m=!0,i===o&&(c=d,v=!0)),c||(c=yr(a,o,{}),v=!0),h=new gt(e,r,d,c,s),a._edges.set(r,h);let p=i===o;return e?(d.undirectedDegree++,c.undirectedDegree++,p&&(d.undirectedLoops++,a._undirectedSelfLoopCount++)):(d.outDegree++,c.inDegree++,p&&(d.directedLoops++,a._directedSelfLoopCount++)),a.multi?h.attachMulti():h.attach(),e?a._undirectedSize++:a._directedSize++,g.key=r,a.emit("edgeAdded",g),[r,!0,m,v]}function ht(a,n){a._edges.delete(n.key);let{source:t,target:e,attributes:r}=n,i=n.undirected,o=t===e;i?(t.undirectedDegree--,e.undirectedDegree--,o&&(t.undirectedLoops--,a._undirectedSelfLoopCount--)):(t.outDegree--,e.inDegree--,o&&(t.directedLoops--,a._directedSelfLoopCount--)),a.multi?n.detachMulti():n.detach(),i?a._undirectedSize--:a._directedSize--,a.emit("edgeDropped",{key:n.key,attributes:r,source:t.key,target:e.key,undirected:i})}var re=class a extends xr.EventEmitter{constructor(n){if(super(),n=fe({},ll,n),typeof n.multi!="boolean")throw new O(`Graph.constructor: invalid 'multi' option. Expecting a boolean but got "${n.multi}".`);if(!ol.has(n.type))throw new O(`Graph.constructor: invalid 'type' option. Should be one of "mixed", "directed" or "undirected" but got "${n.type}".`);if(typeof n.allowSelfLoops!="boolean")throw new O(`Graph.constructor: invalid 'allowSelfLoops' option. Expecting a boolean but got "${n.allowSelfLoops}".`);let t=n.type==="mixed"?Tr:n.type==="directed"?Sr:Er;Ee(this,"NodeDataClass",t);let e="geid_"+il()+"_",r=0,i=()=>{let o;do o=e+r++;while(this._edges.has(o));return o};Ee(this,"_attributes",{}),Ee(this,"_nodes",new Map),Ee(this,"_edges",new Map),Ee(this,"_directedSize",0),Ee(this,"_undirectedSize",0),Ee(this,"_directedSelfLoopCount",0),Ee(this,"_undirectedSelfLoopCount",0),Ee(this,"_edgeKeyGenerator",i),Ee(this,"_options",n),br.forEach(o=>Ee(this,o,this[o])),Ce(this,"order",()=>this._nodes.size),Ce(this,"size",()=>this._edges.size),Ce(this,"directedSize",()=>this._directedSize),Ce(this,"undirectedSize",()=>this._undirectedSize),Ce(this,"selfLoopCount",()=>this._directedSelfLoopCount+this._undirectedSelfLoopCount),Ce(this,"directedSelfLoopCount",()=>this._directedSelfLoopCount),Ce(this,"undirectedSelfLoopCount",()=>this._undirectedSelfLoopCount),Ce(this,"multi",this._options.multi),Ce(this,"type",this._options.type),Ce(this,"allowSelfLoops",this._options.allowSelfLoops),Ce(this,"implementation",()=>"graphology")}_resetInstanceCounters(){this._directedSize=0,this._undirectedSize=0,this._directedSelfLoopCount=0,this._undirectedSelfLoopCount=0}hasNode(n){return this._nodes.has(""+n)}hasDirectedEdge(n,t){if(this.type==="undirected")return!1;if(arguments.length===1){let e=""+n,r=this._edges.get(e);return!!r&&!r.undirected}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?e.out.hasOwnProperty(t):!1}throw new O(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasUndirectedEdge(n,t){if(this.type==="directed")return!1;if(arguments.length===1){let e=""+n,r=this._edges.get(e);return!!r&&r.undirected}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?e.undirected.hasOwnProperty(t):!1}throw new O(`Graph.hasDirectedEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}hasEdge(n,t){if(arguments.length===1){let e=""+n;return this._edges.has(e)}else if(arguments.length===2){n=""+n,t=""+t;let e=this._nodes.get(n);return e?typeof e.out<"u"&&e.out.hasOwnProperty(t)||typeof e.undirected<"u"&&e.undirected.hasOwnProperty(t):!1}throw new O(`Graph.hasEdge: invalid arity (${arguments.length}, instead of 1 or 2). You can either ask for an edge id or for the existence of an edge between a source & a target.`)}directedEdge(n,t){if(this.type==="undirected")return;if(n=""+n,t=""+t,this.multi)throw new X("Graph.directedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.directedEdges instead.");let e=this._nodes.get(n);if(!e)throw new W(`Graph.directedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new W(`Graph.directedEdge: could not find the "${t}" target node in the graph.`);let r=e.out&&e.out[t]||void 0;if(r)return r.key}undirectedEdge(n,t){if(this.type==="directed")return;if(n=""+n,t=""+t,this.multi)throw new X("Graph.undirectedEdge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.undirectedEdges instead.");let e=this._nodes.get(n);if(!e)throw new W(`Graph.undirectedEdge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new W(`Graph.undirectedEdge: could not find the "${t}" target node in the graph.`);let r=e.undirected&&e.undirected[t]||void 0;if(r)return r.key}edge(n,t){if(this.multi)throw new X("Graph.edge: this method is irrelevant with multigraphs since there might be multiple edges between source & target. See #.edges instead.");n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.edge: could not find the "${n}" source node in the graph.`);if(!this._nodes.has(t))throw new W(`Graph.edge: could not find the "${t}" target node in the graph.`);let r=e.out&&e.out[t]||e.undirected&&e.undirected[t]||void 0;if(r)return r.key}areDirectedNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areDirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.in||t in e.out}areOutNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areOutNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.out}areInNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areInNeighbors: could not find the "${n}" node in the graph.`);return this.type==="undirected"?!1:t in e.in}areUndirectedNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areUndirectedNeighbors: could not find the "${n}" node in the graph.`);return this.type==="directed"?!1:t in e.undirected}areNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&(t in e.in||t in e.out)||this.type!=="directed"&&t in e.undirected}areInboundNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areInboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&t in e.in||this.type!=="directed"&&t in e.undirected}areOutboundNeighbors(n,t){n=""+n,t=""+t;let e=this._nodes.get(n);if(!e)throw new W(`Graph.areOutboundNeighbors: could not find the "${n}" node in the graph.`);return this.type!=="undirected"&&t in e.out||this.type!=="directed"&&t in e.undirected}inDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.inDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree}outDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.outDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.outDegree}directedDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.directedDegree: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree+t.outDegree}undirectedDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.undirectedDegree: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:t.undirectedDegree}inboundDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.inboundDegree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.inDegree),e}outboundDegree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.outboundDegree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.outDegree),e}degree(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.degree: could not find the "${n}" node in the graph.`);let e=0;return this.type!=="directed"&&(e+=t.undirectedDegree),this.type!=="undirected"&&(e+=t.inDegree+t.outDegree),e}inDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.inDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree-t.directedLoops}outDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.outDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.outDegree-t.directedLoops}directedDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.directedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="undirected"?0:t.inDegree+t.outDegree-t.directedLoops*2}undirectedDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.undirectedDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);return this.type==="directed"?0:t.undirectedDegree-t.undirectedLoops*2}inboundDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.inboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.inDegree,r+=t.directedLoops),e-r}outboundDegreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.outboundDegreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.outDegree,r+=t.directedLoops),e-r}degreeWithoutSelfLoops(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.degreeWithoutSelfLoops: could not find the "${n}" node in the graph.`);let e=0,r=0;return this.type!=="directed"&&(e+=t.undirectedDegree,r+=t.undirectedLoops*2),this.type!=="undirected"&&(e+=t.inDegree+t.outDegree,r+=t.directedLoops*2),e-r}source(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.source: could not find the "${n}" edge in the graph.`);return t.source.key}target(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.target: could not find the "${n}" edge in the graph.`);return t.target.key}extremities(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.extremities: could not find the "${n}" edge in the graph.`);return[t.source.key,t.target.key]}opposite(n,t){n=""+n,t=""+t;let e=this._edges.get(t);if(!e)throw new W(`Graph.opposite: could not find the "${t}" edge in the graph.`);let r=e.source.key,i=e.target.key;if(n===r)return i;if(n===i)return r;throw new W(`Graph.opposite: the "${n}" node is not attached to the "${t}" edge (${r}, ${i}).`)}hasExtremity(n,t){n=""+n,t=""+t;let e=this._edges.get(n);if(!e)throw new W(`Graph.hasExtremity: could not find the "${n}" edge in the graph.`);return e.source.key===t||e.target.key===t}isUndirected(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.isUndirected: could not find the "${n}" edge in the graph.`);return t.undirected}isDirected(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.isDirected: could not find the "${n}" edge in the graph.`);return!t.undirected}isSelfLoop(n){n=""+n;let t=this._edges.get(n);if(!t)throw new W(`Graph.isSelfLoop: could not find the "${n}" edge in the graph.`);return t.source===t.target}addNode(n,t){return ul(this,n,t).key}mergeNode(n,t){if(t&&!pe(t))throw new O(`Graph.mergeNode: invalid attributes. Expecting an object but got "${t}"`);n=""+n,t=t||{};let e=this._nodes.get(n);return e?(t&&(fe(e.attributes,t),this.emit("nodeAttributesUpdated",{type:"merge",key:n,attributes:e.attributes,data:t})),[n,!1]):(e=new this.NodeDataClass(n,t),this._nodes.set(n,e),this.emit("nodeAdded",{key:n,attributes:t}),[n,!0])}updateNode(n,t){if(t&&typeof t!="function")throw new O(`Graph.updateNode: invalid updater function. Expecting a function but got "${t}"`);n=""+n;let e=this._nodes.get(n);if(e){if(t){let i=e.attributes;e.attributes=t(i),this.emit("nodeAttributesUpdated",{type:"replace",key:n,attributes:e.attributes})}return[n,!1]}let r=t?t({}):{};return e=new this.NodeDataClass(n,r),this._nodes.set(n,e),this.emit("nodeAdded",{key:n,attributes:r}),[n,!0]}dropNode(n){n=""+n;let t=this._nodes.get(n);if(!t)throw new W(`Graph.dropNode: could not find the "${n}" node in the graph.`);let e;if(this.type!=="undirected"){for(let r in t.out){e=t.out[r];do ht(this,e),e=e.next;while(e)}for(let r in t.in){e=t.in[r];do ht(this,e),e=e.next;while(e)}}if(this.type!=="directed")for(let r in t.undirected){e=t.undirected[r];do ht(this,e),e=e.next;while(e)}this._nodes.delete(n),this.emit("nodeDropped",{key:n,attributes:t.attributes})}dropEdge(n){let t;if(arguments.length>1){let e=""+arguments[0],r=""+arguments[1];if(t=Ae(this,e,r,this.type),!t)throw new W(`Graph.dropEdge: could not find the "${e}" -> "${r}" edge in the graph.`)}else if(n=""+n,t=this._edges.get(n),!t)throw new W(`Graph.dropEdge: could not find the "${n}" edge in the graph.`);return ht(this,t),this}dropDirectedEdge(n,t){if(arguments.length<2)throw new X("Graph.dropDirectedEdge: it does not make sense to try and drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new X("Graph.dropDirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");n=""+n,t=""+t;let e=Ae(this,n,t,"directed");if(!e)throw new W(`Graph.dropDirectedEdge: could not find a "${n}" -> "${t}" edge in the graph.`);return ht(this,e),this}dropUndirectedEdge(n,t){if(arguments.length<2)throw new X("Graph.dropUndirectedEdge: it does not make sense to drop a directed edge by key. What if the edge with this key is undirected? Use #.dropEdge for this purpose instead.");if(this.multi)throw new X("Graph.dropUndirectedEdge: cannot use a {source,target} combo when dropping an edge in a MultiGraph since we cannot infer the one you want to delete as there could be multiple ones.");let e=Ae(this,n,t,"undirected");if(!e)throw new W(`Graph.dropUndirectedEdge: could not find a "${n}" -> "${t}" edge in the graph.`);return ht(this,e),this}clear(){this._edges.clear(),this._nodes.clear(),this._resetInstanceCounters(),this.emit("cleared")}clearEdges(){let n=this._nodes.values(),t;for(;t=n.next(),t.done!==!0;)t.value.clear();this._edges.clear(),this._resetInstanceCounters(),this.emit("edgesCleared")}getAttribute(n){return this._attributes[n]}getAttributes(){return this._attributes}hasAttribute(n){return this._attributes.hasOwnProperty(n)}setAttribute(n,t){return this._attributes[n]=t,this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}updateAttribute(n,t){if(typeof t!="function")throw new O("Graph.updateAttribute: updater should be a function.");let e=this._attributes[n];return this._attributes[n]=t(e),this.emit("attributesUpdated",{type:"set",attributes:this._attributes,name:n}),this}removeAttribute(n){return delete this._attributes[n],this.emit("attributesUpdated",{type:"remove",attributes:this._attributes,name:n}),this}replaceAttributes(n){if(!pe(n))throw new O("Graph.replaceAttributes: provided attributes are not a plain object.");return this._attributes=n,this.emit("attributesUpdated",{type:"replace",attributes:this._attributes}),this}mergeAttributes(n){if(!pe(n))throw new O("Graph.mergeAttributes: provided attributes are not a plain object.");return fe(this._attributes,n),this.emit("attributesUpdated",{type:"merge",attributes:this._attributes,data:n}),this}updateAttributes(n){if(typeof n!="function")throw new O("Graph.updateAttributes: provided updater is not a function.");return this._attributes=n(this._attributes),this.emit("attributesUpdated",{type:"update",attributes:this._attributes}),this}updateEachNodeAttributes(n,t){if(typeof n!="function")throw new O("Graph.updateEachNodeAttributes: expecting an updater function.");if(t&&!mr(t))throw new O("Graph.updateEachNodeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let e=this._nodes.values(),r,i;for(;r=e.next(),r.done!==!0;)i=r.value,i.attributes=n(i.key,i.attributes);this.emit("eachNodeAttributesUpdated",{hints:t||null})}updateEachEdgeAttributes(n,t){if(typeof n!="function")throw new O("Graph.updateEachEdgeAttributes: expecting an updater function.");if(t&&!mr(t))throw new O("Graph.updateEachEdgeAttributes: invalid hints. Expecting an object having the following shape: {attributes?: [string]}");let e=this._edges.values(),r,i,o,s;for(;r=e.next(),r.done!==!0;)i=r.value,o=i.source,s=i.target,i.attributes=n(i.key,i.attributes,o.key,s.key,o.attributes,s.attributes,i.undirected);this.emit("eachEdgeAttributesUpdated",{hints:t||null})}forEachAdjacencyEntry(n){if(typeof n!="function")throw new O("Graph.forEachAdjacencyEntry: expecting a callback.");cn(!1,!1,!1,this,n)}forEachAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new O("Graph.forEachAdjacencyEntryWithOrphans: expecting a callback.");cn(!1,!1,!0,this,n)}forEachAssymetricAdjacencyEntry(n){if(typeof n!="function")throw new O("Graph.forEachAssymetricAdjacencyEntry: expecting a callback.");cn(!1,!0,!1,this,n)}forEachAssymetricAdjacencyEntryWithOrphans(n){if(typeof n!="function")throw new O("Graph.forEachAssymetricAdjacencyEntryWithOrphans: expecting a callback.");cn(!1,!0,!0,this,n)}nodes(){return Array.from(this._nodes.keys())}forEachNode(n){if(typeof n!="function")throw new O("Graph.forEachNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)r=e.value,n(r.key,r.attributes)}findNode(n){if(typeof n!="function")throw new O("Graph.findNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,n(r.key,r.attributes))return r.key}mapNodes(n){if(typeof n!="function")throw new O("Graph.mapNode: expecting a callback.");let t=this._nodes.values(),e,r,i=new Array(this.order),o=0;for(;e=t.next(),e.done!==!0;)r=e.value,i[o++]=n(r.key,r.attributes);return i}someNode(n){if(typeof n!="function")throw new O("Graph.someNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,n(r.key,r.attributes))return!0;return!1}everyNode(n){if(typeof n!="function")throw new O("Graph.everyNode: expecting a callback.");let t=this._nodes.values(),e,r;for(;e=t.next(),e.done!==!0;)if(r=e.value,!n(r.key,r.attributes))return!1;return!0}filterNodes(n){if(typeof n!="function")throw new O("Graph.filterNodes: expecting a callback.");let t=this._nodes.values(),e,r,i=[];for(;e=t.next(),e.done!==!0;)r=e.value,n(r.key,r.attributes)&&i.push(r.key);return i}reduceNodes(n,t){if(typeof n!="function")throw new O("Graph.reduceNodes: expecting a callback.");if(arguments.length<2)throw new O("Graph.reduceNodes: missing initial value. You must provide it because the callback takes more than one argument and we cannot infer the initial value from the first iteration, as you could with a simple array.");let e=t,r=this._nodes.values(),i,o;for(;i=r.next(),i.done!==!0;)o=i.value,e=n(e,o.key,o.attributes);return e}nodeEntries(){let n=this._nodes.values();return{[Symbol.iterator](){return this},next(){let t=n.next();if(t.done)return t;let e=t.value;return{value:{node:e.key,attributes:e.attributes},done:!1}}}}export(){let n=new Array(this._nodes.size),t=0;this._nodes.forEach((r,i)=>{n[t++]=tl(i,r)});let e=new Array(this._edges.size);return t=0,this._edges.forEach((r,i)=>{e[t++]=nl(this.type,i,r)}),{options:{type:this.type,multi:this.multi,allowSelfLoops:this.allowSelfLoops},attributes:this.getAttributes(),nodes:n,edges:e}}import(n,t=!1){if(n instanceof a)return n.forEachNode((l,u)=>{t?this.mergeNode(l,u):this.addNode(l,u)}),n.forEachEdge((l,u,d,c,h,f,g)=>{t?g?this.mergeUndirectedEdgeWithKey(l,d,c,u):this.mergeDirectedEdgeWithKey(l,d,c,u):g?this.addUndirectedEdgeWithKey(l,d,c,u):this.addDirectedEdgeWithKey(l,d,c,u)}),this;if(!pe(n))throw new O("Graph.import: invalid argument. Expecting a serialized graph or, alternatively, a Graph instance.");if(n.attributes){if(!pe(n.attributes))throw new O("Graph.import: invalid attributes. Expecting a plain object.");t?this.mergeAttributes(n.attributes):this.replaceAttributes(n.attributes)}let e,r,i,o,s;if(n.nodes){if(i=n.nodes,!Array.isArray(i))throw new O("Graph.import: invalid nodes. Expecting an array.");for(e=0,r=i.length;e<r;e++){o=i[e],al(o);let{key:l,attributes:u}=o;t?this.mergeNode(l,u):this.addNode(l,u)}}if(n.edges){let l=!1;if(this.type==="undirected"&&(l=!0),i=n.edges,!Array.isArray(i))throw new O("Graph.import: invalid edges. Expecting an array.");for(e=0,r=i.length;e<r;e++){s=i[e],rl(s);let{source:u,target:d,attributes:c,undirected:h=l}=s,f;"key"in s?(f=t?h?this.mergeUndirectedEdgeWithKey:this.mergeDirectedEdgeWithKey:h?this.addUndirectedEdgeWithKey:this.addDirectedEdgeWithKey,f.call(this,s.key,u,d,c)):(f=t?h?this.mergeUndirectedEdge:this.mergeDirectedEdge:h?this.addUndirectedEdge:this.addDirectedEdge,f.call(this,u,d,c))}}return this}nullCopy(n){let t=new a(fe({},this._options,n));return t.replaceAttributes(fe({},this.getAttributes())),t}emptyCopy(n){let t=this.nullCopy(n);return this._nodes.forEach((e,r)=>{let i=fe({},e.attributes);e=new t.NodeDataClass(r,i),t._nodes.set(r,e)}),t}copy(n){if(n=n||{},typeof n.type=="string"&&n.type!==this.type&&n.type!=="mixed")throw new X(`Graph.copy: cannot create an incompatible copy from "${this.type}" type to "${n.type}" because this would mean losing information about the current graph.`);if(typeof n.multi=="boolean"&&n.multi!==this.multi&&n.multi!==!0)throw new X("Graph.copy: cannot create an incompatible copy by downgrading a multi graph to a simple one because this would mean losing information about the current graph.");if(typeof n.allowSelfLoops=="boolean"&&n.allowSelfLoops!==this.allowSelfLoops&&n.allowSelfLoops!==!0)throw new X("Graph.copy: cannot create an incompatible copy from a graph allowing self loops to one that does not because this would mean losing information about the current graph.");let t=this.emptyCopy(n),e=this._edges.values(),r,i;for(;r=e.next(),r.done!==!0;)i=r.value,Lr(t,"copy",!1,i.undirected,i.key,i.source.key,i.target.key,fe({},i.attributes));return t}toJSON(){return this.export()}toString(){return"[object Graph]"}inspect(){let n={};this._nodes.forEach((i,o)=>{n[o]=i.attributes});let t={},e={};this._edges.forEach((i,o)=>{let s=i.undirected?"--":"->",l="",u=i.source.key,d=i.target.key,c;i.undirected&&u>d&&(c=u,u=d,d=c);let h=`(${u})${s}(${d})`;o.startsWith("geid_")?this.multi&&(typeof e[h]>"u"?e[h]=0:e[h]++,l+=`${e[h]}. `):l+=`[${o}]: `,l+=h,t[l]=i.attributes});let r={};for(let i in this)this.hasOwnProperty(i)&&!br.has(i)&&typeof this[i]!="function"&&typeof i!="symbol"&&(r[i]=this[i]);return r.attributes=this._attributes,r.nodes=n,r.edges=t,Ee(r,"constructor",this.constructor),r}};typeof Symbol<"u"&&(re.prototype[Symbol.for("nodejs.util.inspect.custom")]=re.prototype.inspect);sl.forEach(a=>{["add","merge","update"].forEach(n=>{let t=a.name(n),e=n==="add"?Lr:dl;a.generateKey?re.prototype[t]=function(r,i,o){return e(this,t,!0,(a.type||this.type)==="undirected",null,r,i,o,n==="update")}:re.prototype[t]=function(r,i,o,s){return e(this,t,!1,(a.type||this.type)==="undirected",r,i,o,s,n==="update")}})});ys(re);Cs(re);js(re);el(re);var hn=class extends re{constructor(n){let t=fe({type:"directed"},n);if("multi"in t&&t.multi!==!1)throw new O("DirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(t.type!=="directed")throw new O('DirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},fn=class extends re{constructor(n){let t=fe({type:"undirected"},n);if("multi"in t&&t.multi!==!1)throw new O("UndirectedGraph.from: inconsistent indication that the graph should be multi in given options!");if(t.type!=="undirected")throw new O('UndirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},gn=class extends re{constructor(n){let t=fe({multi:!0},n);if("multi"in t&&t.multi!==!0)throw new O("MultiGraph.from: inconsistent indication that the graph should be simple in given options!");super(t)}},pn=class extends re{constructor(n){let t=fe({type:"directed",multi:!0},n);if("multi"in t&&t.multi!==!0)throw new O("MultiDirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(t.type!=="directed")throw new O('MultiDirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}},vn=class extends re{constructor(n){let t=fe({type:"undirected",multi:!0},n);if("multi"in t&&t.multi!==!0)throw new O("MultiUndirectedGraph.from: inconsistent indication that the graph should be simple in given options!");if(t.type!=="undirected")throw new O('MultiUndirectedGraph.from: inconsistent "'+t.type+'" type in given options!');super(t)}};function pt(a){a.from=function(n,t){let e=fe({},n.options,t),r=new a(e);return r.import(n),r}}pt(re);pt(hn);pt(fn);pt(gn);pt(pn);pt(vn);re.Graph=re;re.DirectedGraph=hn;re.UndirectedGraph=fn;re.MultiGraph=gn;re.MultiDirectedGraph=pn;re.MultiUndirectedGraph=vn;re.InvalidArgumentsGraphError=O;re.NotFoundGraphError=W;re.UsageGraphError=X;var Cr=["#4f7cff","#ff3d8b","#12c99b","#ff9f1a","#a259ff","#ff5c39","#1bb8f0","#8bc34a","#e040fb","#00bfa5"],kr=["note","project","session"],cl="wisply.graph.",hl=3,fl=15;function Pr(){try{for(let a of Object.keys(localStorage))a.startsWith(cl)&&localStorage.removeItem(a)}catch{return}}function Bt(a){return Math.round(Math.min(1,Math.max(0,a))*255).toString(16).padStart(2,"0")}function Fr(a,n,t,e,r){a.forEachNode((s,l)=>{let u=t.get(n.get(s)??"");u!==void 0&&!t.has(l.cluster)&&(l.color=u)});let i=Bt(e[3]),o=`#${Bt(e[0])}${Bt(e[1])}${Bt(e[2])}${Bt(e[3]*r)}`;a.forEachEdge((s,l,u,d)=>{let c=n.get(u),h=t.get(c??"");c===void 0||c!==n.get(d)?l.color=o:h!==void 0&&(l.color=h+i)})}function gl(a){let n=new Map(a.nodes().map(t=>[t,t]));for(let t=0;t<fl;t++){let e=!1;if(a.forEachNode(r=>{let i=new Map;a.forEachNeighbor(r,l=>{let u=n.get(l);i.set(u,(i.get(u)??0)+1)});let o=n.get(r),s=0;for(let[l,u]of i)(u>s||u===s&&l<o)&&(o=l,s=u);s>0&&o!==n.get(r)&&(n.set(r,o),e=!0)}),!e)break}return n}function Ir(a,n){let t=new re({type:"undirected",multi:!1,allowSelfLoops:!1}),e=new Array(a.nodes.length).fill(0);for(let[c,h]of a.edges)e[c]++,e[h]++;a.nodes.forEach(([c,h,f,g,m],v)=>{let p=f;t.mergeNode(c,{label:h||c,tag:"",kind:p,group:g,cluster:"",t:m,x:0,y:0,size:p==="project"?3+Math.sqrt(e[v])*.5:1.2+Math.sqrt(e[v])*.5,color:n,deg:e[v]})});for(let[c,h]of a.edges){let f=a.nodes[c][0],g=a.nodes[h][0];f!==g&&!t.hasEdge(f,g)&&t.addEdge(f,g)}let i=new Set(a.nodes.map(c=>c[3]).filter(Boolean)).size>=2?null:gl(t);t.forEachNode((c,h)=>{h.cluster=i?i.get(c):h.group});let o=new Map;t.forEachNode((c,h)=>o.set(h.cluster,(o.get(h.cluster)??0)+1));let s=[...o.entries()].filter(([c,h])=>c&&h>=hl).sort((c,h)=>h[1]-c[1]||(c[0]<h[0]?-1:1)),l=new Map,u=new Map,d=new Map;return t.forEachNode((c,h)=>{let f=d.get(h.cluster);(f===void 0||h.deg>t.getNodeAttribute(f,"deg"))&&d.set(h.cluster,c)}),s.forEach(([c],h)=>{l.set(c,Cr[h%Cr.length]),u.set(c,i?t.getNodeAttribute(d.get(c),"label"):c)}),t.forEachNode((c,h)=>{h.color=l.get(h.cluster)??n}),{g:t,colors:l,names:u}}function Nr(a,n,t){let e=new Map;for(let[r,i]of n)e.set(r,{key:r,name:t.get(r)??r,color:i,count:0,hub:"",lead:!1});return a.forEachNode((r,i)=>{let o=e.get(i.cluster);if(!o)return;o.count++;let s=i.label===o.name;o.hub&&(o.lead&&!s||o.lead===s&&a.getNodeAttribute(o.hub,"deg")>=i.deg)||(o.hub=r,o.lead=s)}),[...e.values()]}function Wt(a){let n=a.trim();return n.length>30?`${n.slice(0,29).trimEnd()}\u2026`:n}function Gr(a){let n=a.map(e=>e.x),t=a.map(e=>e.y);return{left:Math.min(...n),right:Math.max(...n),low:Math.min(...t),high:Math.max(...t)}}function Mr(a,n,t,e){let r=[],i=[];for(let o of a){let s=Math.max(o.r,n*1.3/2)+2.5,l={left:o.x-o.r-2.5,right:o.x+o.r+4+o.text.length*n*.56+2.5,low:o.y-s,high:o.y+s};l.left<0||l.right>t||l.low<0||l.high>e||r.some(u=>l.left<u.right&&u.left<l.right&&l.low<u.high&&u.low<l.high)||(r.push(l),i.push(o.n))}return i}function zr(a,n,t,e,r){let i=[];return a.forEachNode((o,s)=>{s.tag||s.size<t||s.x<n.left||s.x>n.right||s.y<n.low||s.y>n.high||e.has(o)||i.push(o)}),i.length<=r?i:i.sort((o,s)=>a.getNodeAttribute(s,"size")-a.getNodeAttribute(o,"size")||(o<s?-1:1)).slice(0,r)}var pl=240,vl=.9,ml=2.4,bl=.05,yl=1,Br={off:!1,named:!1,mute:!1},Wr={off:!1},Ur={hush:!1};function Or(a,n){let t=new re({type:a.type,multi:a.multi,allowSelfLoops:a.allowSelfLoops}),e=[];return{top:t,spot:(i,o)=>{t.order&&t.clear(),e=[];let s=(l,u)=>{if(n.has(l)||t.hasNode(l))return;let d=a.getNodeAttributes(l);t.addNode(l,{...d,tag:Wt(d.label),pin:u,plate:u}),u&&e.unshift(l)};if(o!==null&&s(o,!0),i!==null){s(i,!0);let l=a.filterNeighbors(i,u=>!n.has(u));l.sort((u,d)=>a.getNodeAttribute(d,"deg")-a.getNodeAttribute(u,"deg")||(u<d?-1:1));for(let u of l)s(u,!1);e.push(...l.filter(u=>!e.includes(u)).slice(0,pl)),a.forEachEdge(i,(u,d,c,h)=>{t.hasNode(c)&&t.hasNode(h)&&t.addEdgeWithKey(u,c,h)})}return t.order},ranked:()=>e}}function Hr(a){let n=[...a].sort((e,r)=>e-r),t=n[Math.floor(n.length*vl)]??n[n.length-1]??0;return Math.min(yl,Math.max(bl,t*ml))}function $r(a,n){let t=new Set,e=new Set;return{gone:t,cut:e,sift:i=>{let o=[],s=[];a.forEachNode((h,f)=>{let g=!i(f);g!==t.has(h)&&(g?(t.add(h),o.push(h)):(t.delete(h),s.push(h)))});let l=o.concat(s);if(!l.length)return l;n.setNodesState(o,{off:!0}),n.setNodesState(s,{off:!1});let u=new Set,d=[],c=[];for(let h of l)a.forEachEdge(h,(f,g,m,v)=>{u.has(f)||(u.add(f),(t.has(m)||t.has(v)?d:c).push(f))});for(let h of d)e.add(h);for(let h of c)e.delete(h);return n.setEdgesState(d,{off:!0}),n.setEdgesState(c,{off:!1}),l}}}function Vr(a){let n=!1;return()=>{n||(n=!0,requestAnimationFrame(()=>{n=!1,a()}))}}function jr(a,n){if(typeof requestIdleCallback=="function"){let e=requestIdleCallback(a,{timeout:n});return()=>cancelIdleCallback(e)}let t=setTimeout(a,n);return()=>clearTimeout(t)}var Ut=Math.PI*(3-Math.sqrt(5)),vt=6,xl=4,_l=12,Tl=3,Sl=.5,El=18,Al=4,Xr=.2,wl=3,Dl=8;function da(a,n){return 2*Math.asin(Math.min(1,a/n))}function ca(a,n=0){let t=[...a].sort((o,s)=>s.r-o.r),e=n,r=0;!n&&t.length&&(t[0].x=0,t[0].y=0,e=t[0].r,r=1);let i=0;for(;r<t.length;){let o=e+t[r].r,s=[],l=0;for(;r<t.length&&(!s.length||l+da(t[r].r,o)<=2*Math.PI);)l+=da(t[r].r,o),s.push(t[r++]);let u=Math.max(0,2*Math.PI-l)/s.length,d=i;for(let c of s){let h=da(c.r,o)+u;c.x=o*Math.cos(d+h/2),c.y=o*Math.sin(d+h/2),d+=h}e=o+s[0].r,i+=Ut}return e}function Rl(a,n,t){let e=new Map;a.forEachNeighbor(n,o=>{let s=t.get(o);s!==void 0&&e.set(s,(e.get(s)??0)+1)});let r=null,i=0;for(let[o,s]of e)(s>i||s===i&&r!==null&&o<r)&&(r=o,i=s);return r}function Ll(a,n,t){for(let e of n){if(t.has(e))continue;let r=`\0${e}`,i=[e];t.set(e,r);for(let o=0;o<i.length;o++)a.forEachNeighbor(i[o],s=>{t.has(s)||(t.set(s,r),i.push(s))})}}function Cl(a,n){let t=new Map,e=(r,i)=>{let o=t.get(r)??new Map;o.set(i,(o.get(i)??0)+1),t.set(r,o)};return a.forEachEdge((r,i,o,s)=>{let l=n.get(o),u=n.get(s);l===void 0||u===void 0||l===u||(e(l,u),e(u,l))}),t}function kl(a){let n=0,t=0,e="";for(let[r,i]of a)n+=i,(i>t||i===t&&r<e)&&(e=r,t=i);return{best:e,most:t,all:n}}function Pl(a,n,t){let e=new Map,r=l=>{let u=l;for(let d=e.get(u);d!==void 0;d=e.get(u))u=d;return u},i=(l,u)=>{let d=r(l),c=r(u);d!==c&&e.set(c,d)},o=l=>(t.get(l)??[]).length;for(let[l,u]of Cl(a,n)){for(let[f,g]of u)l<f&&g>=wl&&g>=Xr*Math.min(o(l),o(f))&&i(l,f);let{best:d,most:c,all:h}=kl(u);c*2>h&&c>=Xr*o(l)&&i(d,l)}let s=new Map;for(let l of t.keys()){let u=r(l),d=s.get(u);d?d.push(l):s.set(u,[l])}return[...s.values()]}function Fl(a){let n=new Map,t=[],e=[];a.forEachNode((i,o)=>{a.degree(i)?o.cluster?n.set(i,o.cluster):e.push(i):t.push(i)});for(let i=0;i<Al&&e.length;i++){let o=e.map(s=>[s,Rl(a,s,n)]);e=[];for(let[s,l]of o)l===null?e.push(s):n.set(s,l)}Ll(a,e,n);let r=new Map;for(let[i,o]of n){let s=r.get(o);s?s.push(i):r.set(o,[i])}for(let i of r.values())i.sort((o,s)=>a.degree(s)-a.degree(o)||(o<s?-1:1));return{groups:r,loose:t,home:n}}function Il(a,n,t){let e=new Map;for(let[r,i]of t){let o=i[0];for(let s of i){if(s===o)continue;let l=a.degree(s),u=o,d=1/0;a.forEachNeighbor(s,h=>{if(h===o||n.get(h)!==r)return;let f=a.degree(h);f<l||f===l&&h>s||(f<d||f===d&&h<u)&&(u=h,d=f)});let c=e.get(u);c?c.push(s):e.set(u,[s])}}return e}function qr(a,n,t,e){let r=xl*Math.sqrt(1+Math.sqrt(a.degree(t))),i=[],o=[];for(let c of n.get(t)??[])n.has(c)?o.push(qr(a,n,c,Tl)):i.push(c);let s=Math.sqrt(r*r+i.length*vt*vt),l=s,u=0,d=0;if(o.length){ca(o,s);let c=[-s,s,-s,s];for(let h of o)c[0]=Math.min(c[0],h.x-h.r),c[1]=Math.max(c[1],h.x+h.r),c[2]=Math.min(c[2],h.y-h.r),c[3]=Math.max(c[3],h.y+h.r);u=(c[0]+c[1])/2,d=(c[2]+c[3])/2,l=Math.hypot(u,d)+s;for(let h of o)l=Math.max(l,Math.hypot(h.x-u,h.y-d)+h.r)}return{node:t,hole:r,leaves:i,heavy:o,r:l+Math.min(e,l*Sl),x:0,y:0,ox:-u,oy:-d}}function Kr(a,n,t,e){let r=a.getNodeAttributes(n.node);r.x=t+n.ox,r.y=e+n.oy,n.leaves.forEach((i,o)=>{let s=Math.sqrt(n.hole*n.hole+(o+1)*vt*vt),l=a.getNodeAttributes(i);l.x=r.x+s*Math.cos((o+1)*Ut),l.y=r.y+s*Math.sin((o+1)*Ut)});for(let i of n.heavy)Kr(a,i,r.x+i.x,r.y+i.y)}function Yr(a,n,t,e,r){let i=a.getNodeAttributes(t);i.x+=e,i.y+=r;for(let o of n.get(t)??[])Yr(a,n,o,e,r)}function Nl(a,n,t,e){a.forEachNode((r,i)=>{if(a.degree(r)<Dl||e.has(r))return;let o=n.get(r),s=0,l=0,u=0,d=0;a.forEachNeighbor(r,(c,h)=>{s++,n.get(c)!==o&&l++,u+=h.x,d+=h.y}),l*2>s&&Yr(a,t,r,u/s-i.x,d/s-i.y)})}function Gl(a,n){let t=0;a.forEachNode((r,i)=>{a.degree(r)&&(t=Math.max(t,i.x*i.x+i.y*i.y))});let e=(Math.sqrt(t)+El)**2;n.forEach((r,i)=>{let o=Math.sqrt(e+i*vt*vt),s=a.getNodeAttributes(r);s.x=o*Math.cos(i*Ut),s.y=o*Math.sin(i*Ut)})}function Zr(a){let{groups:n,loose:t,home:e}=Fl(a),r=Il(a,e,n),i=new Map;for(let[s,l]of n)i.set(s,qr(a,r,l[0],_l));let o=Pl(a,e,n).map(s=>{let l=s.flatMap(u=>i.get(u)??[]);return{list:l,r:l.length>1?ca(l):l[0].r,x:0,y:0}});ca(o);for(let s of o)for(let l of s.list)Kr(a,l,s.x+l.x,s.y+l.y);return Nl(a,e,r,new Set([...n.values()].map(s=>s[0]))),Gl(a,t),e}function xn(a,n){(n==null||n>a.length)&&(n=a.length);for(var t=0,e=Array(n);t<n;t++)e[t]=a[t];return e}function _n(a,n){if(a){if(typeof a=="string")return xn(a,n);var t={}.toString.call(a).slice(8,-1);return t==="Object"&&a.constructor&&(t=a.constructor.name),t==="Map"||t==="Set"?Array.from(a):t==="Arguments"||/^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(t)?xn(a,n):void 0}}function Ml(a){if(Array.isArray(a))return a}function zl(a,n){var t=a==null?null:typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(t!=null){var e,r,i,o,s=[],l=!0,u=!1;try{if(i=(t=t.call(a)).next,n===0){if(Object(t)!==t)return;l=!1}else for(;!(l=(e=i.call(t)).done)&&(s.push(e.value),s.length!==n);l=!0);}catch(d){u=!0,r=d}finally{try{if(!l&&t.return!=null&&(o=t.return(),Object(o)!==o))return}finally{if(u)throw r}}return s}}function Bl(){throw new TypeError(`Invalid attempt to destructure non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function ae(a,n){return Ml(a)||zl(a,n)||_n(a,n)||Bl()}function z(a,n){var t=typeof Symbol<"u"&&a[Symbol.iterator]||a["@@iterator"];if(!t){if(Array.isArray(a)||(t=_n(a))||n&&a&&typeof a.length=="number"){t&&(a=t);var e=0,r=function(){};return{s:r,n:function(){return e>=a.length?{done:!0}:{done:!1,value:a[e++]}},e:function(l){throw l},f:r}}throw new TypeError(`Invalid attempt to iterate non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}var i,o=!0,s=!1;return{s:function(){t=t.call(a)},n:function(){var l=t.next();return o=l.done,l},e:function(l){s=!0,i=l},f:function(){try{o||t.return==null||t.return()}finally{if(s)throw i}}}}function we(){return Float32Array.of(1,0,0,0,1,0,0,0,1)}function bn(a,n,t){return a[0]=n,a[4]=typeof t=="number"?t:n,a}function Qr(a,n){var t=Math.sin(n),e=Math.cos(n);return a[0]=e,a[1]=t,a[3]=-t,a[4]=e,a}function Jr(a,n,t){return a[6]=n,a[7]=t,a}function $e(a,n){var t=a[0],e=a[1],r=a[2],i=a[3],o=a[4],s=a[5],l=a[6],u=a[7],d=a[8],c=n[0],h=n[1],f=n[2],g=n[3],m=n[4],v=n[5],p=n[6],b=n[7],y=n[8];return a[0]=c*t+h*i+f*l,a[1]=c*e+h*o+f*u,a[2]=c*r+h*s+f*d,a[3]=g*t+m*i+v*l,a[4]=g*e+m*o+v*u,a[5]=g*r+m*s+v*d,a[6]=p*t+b*i+y*l,a[7]=p*e+b*o+y*u,a[8]=p*r+b*s+y*d,a}function ha(a,n){var t=Math.cos(n),e=Math.sin(n);return{x:t*a.x+e*a.y,y:-e*a.x+t*a.y}}function bt(a,n){var t=arguments.length>2&&arguments[2]!==void 0?arguments[2]:1,e=a[0],r=a[1],i=a[3],o=a[4],s=a[6],l=a[7],u=n.x,d=n.y;return{x:u*e+d*i+s*t,y:u*r+d*o+l*t}}function Wl(a,n){var t=a.height/a.width,e=n.height/n.width;return t<1&&e>1||t>1&&e<1?1:Math.min(Math.max(e,1/e),Math.max(1/t,t))}function Ve(a,n,t,e,r){var i=a.angle,o=a.ratio,s=a.x,l=a.y,u=n.width,d=n.height,c=we(),h=Math.min(u,d)-2*e,f=Wl(n,t);return r?($e(c,Jr(we(),s,l)),$e(c,bn(we(),o)),$e(c,Qr(we(),i)),$e(c,bn(we(),u/h/2/f,d/h/2/f))):($e(c,bn(we(),2*(h/u)*f,2*(h/d)*f)),$e(c,Qr(we(),-i)),$e(c,bn(we(),1/o)),$e(c,Jr(we(),-s,-l))),c}function ei(a,n,t){var e=bt(a,{x:Math.cos(n.angle),y:Math.sin(n.angle)},0),r=e.x,i=e.y;return 1/Math.sqrt(Math.pow(r,2)+Math.pow(i,2))/t.width}function fa(a,n,t,e,r){var i=Math.floor(n/r*e),o=Math.floor(a.drawingBufferHeight/r-t/r*e);return[i,o]}var Ht={black:"#000000",silver:"#C0C0C0",gray:"#808080",grey:"#808080",white:"#FFFFFF",maroon:"#800000",red:"#FF0000",purple:"#800080",fuchsia:"#FF00FF",green:"#008000",lime:"#00FF00",olive:"#808000",yellow:"#FFFF00",navy:"#000080",blue:"#0000FF",teal:"#008080",aqua:"#00FFFF",darkblue:"#00008B",mediumblue:"#0000CD",darkgreen:"#006400",darkcyan:"#008B8B",deepskyblue:"#00BFFF",darkturquoise:"#00CED1",mediumspringgreen:"#00FA9A",springgreen:"#00FF7F",cyan:"#00FFFF",midnightblue:"#191970",dodgerblue:"#1E90FF",lightseagreen:"#20B2AA",forestgreen:"#228B22",seagreen:"#2E8B57",darkslategray:"#2F4F4F",darkslategrey:"#2F4F4F",limegreen:"#32CD32",mediumseagreen:"#3CB371",turquoise:"#40E0D0",royalblue:"#4169E1",steelblue:"#4682B4",darkslateblue:"#483D8B",mediumturquoise:"#48D1CC",indigo:"#4B0082",darkolivegreen:"#556B2F",cadetblue:"#5F9EA0",cornflowerblue:"#6495ED",rebeccapurple:"#663399",mediumaquamarine:"#66CDAA",dimgray:"#696969",dimgrey:"#696969",slateblue:"#6A5ACD",olivedrab:"#6B8E23",slategray:"#708090",slategrey:"#708090",lightslategray:"#778899",lightslategrey:"#778899",mediumslateblue:"#7B68EE",lawngreen:"#7CFC00",chartreuse:"#7FFF00",aquamarine:"#7FFFD4",skyblue:"#87CEEB",lightskyblue:"#87CEFA",blueviolet:"#8A2BE2",darkred:"#8B0000",darkmagenta:"#8B008B",saddlebrown:"#8B4513",darkseagreen:"#8FBC8F",lightgreen:"#90EE90",mediumpurple:"#9370DB",darkviolet:"#9400D3",palegreen:"#98FB98",darkorchid:"#9932CC",yellowgreen:"#9ACD32",sienna:"#A0522D",brown:"#A52A2A",darkgray:"#A9A9A9",darkgrey:"#A9A9A9",lightblue:"#ADD8E6",greenyellow:"#ADFF2F",paleturquoise:"#AFEEEE",lightsteelblue:"#B0C4DE",powderblue:"#B0E0E6",firebrick:"#B22222",darkgoldenrod:"#B8860B",mediumorchid:"#BA55D3",rosybrown:"#BC8F8F",darkkhaki:"#BDB76B",mediumvioletred:"#C71585",indianred:"#CD5C5C",peru:"#CD853F",chocolate:"#D2691E",tan:"#D2B48C",lightgray:"#D3D3D3",lightgrey:"#D3D3D3",thistle:"#D8BFD8",orchid:"#DA70D6",goldenrod:"#DAA520",palevioletred:"#DB7093",crimson:"#DC143C",gainsboro:"#DCDCDC",plum:"#DDA0DD",burlywood:"#DEB887",lightcyan:"#E0FFFF",lavender:"#E6E6FA",darksalmon:"#E9967A",violet:"#EE82EE",palegoldenrod:"#EEE8AA",lightcoral:"#F08080",khaki:"#F0E68C",aliceblue:"#F0F8FF",honeydew:"#F0FFF0",azure:"#F0FFFF",sandybrown:"#F4A460",wheat:"#F5DEB3",beige:"#F5F5DC",whitesmoke:"#F5F5F5",mintcream:"#F5FFFA",ghostwhite:"#F8F8FF",salmon:"#FA8072",antiquewhite:"#FAEBD7",linen:"#FAF0E6",lightgoldenrodyellow:"#FAFAD2",oldlace:"#FDF5E6",magenta:"#FF00FF",deeppink:"#FF1493",orangered:"#FF4500",tomato:"#FF6347",hotpink:"#FF69B4",coral:"#FF7F50",darkorange:"#FF8C00",lightsalmon:"#FFA07A",orange:"#FFA500",lightpink:"#FFB6C1",pink:"#FFC0CB",gold:"#FFD700",peachpuff:"#FFDAB9",navajowhite:"#FFDEAD",moccasin:"#FFE4B5",bisque:"#FFE4C4",mistyrose:"#FFE4E1",blanchedalmond:"#FFEBCD",papayawhip:"#FFEFD5",lavenderblush:"#FFF0F5",seashell:"#FFF5EE",cornsilk:"#FFF8DC",lemonchiffon:"#FFFACD",floralwhite:"#FFFAF0",snow:"#FFFAFA",lightyellow:"#FFFFE0",ivory:"#FFFFF0"},ti=new Int8Array(4),yn=new Int32Array(ti.buffer,0,1),ni=new Float32Array(ti.buffer,0,1),Ul=/^\s*rgba?\s*\(/,Ol=/^\s*rgba?\s*\(\s*([0-9]*)\s*,\s*([0-9]*)\s*,\s*([0-9]*)(?:\s*,\s*(.*)?)?\)\s*$/;function Tn(a){var n=0,t=0,e=0,r=1,i=a.toLowerCase();if(i==="transparent")return{r:0,g:0,b:0,a:0};if(i in Ht)return Tn(Ht[i]);if(a[0]==="#")a.length===4?(n=parseInt(a.charAt(1)+a.charAt(1),16),t=parseInt(a.charAt(2)+a.charAt(2),16),e=parseInt(a.charAt(3)+a.charAt(3),16)):(n=parseInt(a.charAt(1)+a.charAt(2),16),t=parseInt(a.charAt(3)+a.charAt(4),16),e=parseInt(a.charAt(5)+a.charAt(6),16)),a.length===9&&(r=parseInt(a.charAt(7)+a.charAt(8),16)/255);else if(Ul.test(a)){var o=a.match(Ol);o&&(n=+o[1],t=+o[2],e=+o[3],o[4]&&(r=+o[4]))}return{r:n,g:t,b:e,a:r}}function ai(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,t=Tn(a),e=t.r,r=t.g,i=t.b,o=t.a;return n?[e/255*o,r/255*o,i/255*o,o]:[e/255,r/255,i/255,o]}var mt={};for(Ot in Ht)mt[Ot]=De(Ht[Ot]),mt[Ht[Ot]]=mt[Ot];var Ot;function Hl(a,n,t,e,r){return yn[0]=e<<24|t<<16|n<<8|a,r&&(yn[0]=yn[0]&4278190079),ni[0]}function De(a){if(a=a.toLowerCase(),typeof mt[a]<"u")return mt[a];var n=Tn(a),t=n.r,e=n.g,r=n.b,i=n.a;i=i*255|0;var o=Hl(t,e,r,i,!0);return mt[a]=o,o}function je(a,n){ni[0]=De(a);var t=yn[0];n&&(t=t|16777216);var e=t&255,r=t>>8&255,i=t>>16&255,o=t>>24&255;return[e,r,i,o]}function yt(a){var n=a>>>8&255,t=a>>>16&255,e=a>>>24&255,r=a&255;return(r<<24|e<<16|t<<8|n)>>>0}function ga(a,n,t,e){return(t<<24|n<<16|a<<8|e)>>>0}function ri(a,n,t,e,r,i){var o=fa(a,t,e,r,i),s=ae(o,2),l=s[0],u=s[1],d=new Uint8Array(4);a.bindFramebuffer(a.FRAMEBUFFER,n),a.readPixels(l,u,1,1,a.RGBA,a.UNSIGNED_BYTE,d);var c=ae(d,4),h=c[0],f=c[1],g=c[2],m=c[3];return[h,f,g,m]}function ii(a){var n=Tn(a),t=n.r,e=n.g,r=n.b,i=n.a,o=(t/255).toFixed(6),s=(e/255).toFixed(6),l=(r/255).toFixed(6),u=i.toFixed(6);return"vec4(".concat(o,", ").concat(s,", ").concat(l,", ").concat(u,")")}function $l(a,n){if(typeof a!="object"||!a)return a;var t;if(typeof Symbol<"u"&&(t=a[Symbol.toPrimitive])!==void 0){var e=t.call(a,n||"default");if(typeof e!="object")return e;throw new TypeError("@@toPrimitive must return a primitive value.")}return(n==="string"?String:Number)(a)}function li(a){var n=$l(a,"string");return typeof n=="symbol"?n:n+""}function S(a,n,t){return(n=li(n))in a?Object.defineProperty(a,n,{value:t,enumerable:!0,configurable:!0,writable:!0}):a[n]=t,a}function oi(a,n){var t=Object.keys(a);if(Object.getOwnPropertySymbols){var e=Object.getOwnPropertySymbols(a);n&&(e=e.filter(function(r){return Object.getOwnPropertyDescriptor(a,r).enumerable})),t.push.apply(t,e)}return t}function B(a){for(var n=1;n<arguments.length;n++){var t=arguments[n]!=null?arguments[n]:{};n%2?oi(Object(t),!0).forEach(function(e){S(a,e,t[e])}):Object.getOwnPropertyDescriptors?Object.defineProperties(a,Object.getOwnPropertyDescriptors(t)):oi(Object(t)).forEach(function(e){Object.defineProperty(a,e,Object.getOwnPropertyDescriptor(t,e))})}return a}function J(a,n){if(!(a instanceof n))throw new TypeError("Cannot call a class as a function")}function si(a,n){for(var t=0;t<n.length;t++){var e=n[t];e.enumerable=e.enumerable||!1,e.configurable=!0,"value"in e&&(e.writable=!0),Object.defineProperty(a,li(e.key),e)}}function ee(a,n,t){return n&&si(a.prototype,n),t&&si(a,t),Object.defineProperty(a,"prototype",{writable:!1}),a}function et(a){return et=Object.setPrototypeOf?Object.getPrototypeOf.bind():function(n){return n.__proto__||Object.getPrototypeOf(n)},et(a)}function ui(){try{var a=!Boolean.prototype.valueOf.call(Reflect.construct(Boolean,[],function(){}))}catch{}return(ui=function(){return!!a})()}function Vl(a){if(a===void 0)throw new ReferenceError("this hasn't been initialised - super() hasn't been called");return a}function jl(a,n){if(n&&(typeof n=="object"||typeof n=="function"))return n;if(n!==void 0)throw new TypeError("Derived constructors may only return object or undefined");return Vl(a)}function le(a,n,t){return n=et(n),jl(a,ui()?Reflect.construct(n,t||[],et(a).constructor):n.apply(a,t))}function pa(a,n){return pa=Object.setPrototypeOf?Object.setPrototypeOf.bind():function(t,e){return t.__proto__=e,t},pa(a,n)}function ue(a,n){if(typeof n!="function"&&n!==null)throw new TypeError("Super expression must either be null or a function");a.prototype=Object.create(n&&n.prototype,{constructor:{value:a,writable:!0,configurable:!0}}),Object.defineProperty(a,"prototype",{writable:!1}),n&&pa(a,n)}function Xl(a){if(Array.isArray(a))return xn(a)}function ql(a){if(typeof Symbol<"u"&&a[Symbol.iterator]!=null||a["@@iterator"]!=null)return Array.from(a)}function Kl(){throw new TypeError(`Invalid attempt to spread non-iterable instance.
In order to be iterable, non-array objects must have a [Symbol.iterator]() method.`)}function V(a){return Xl(a)||ql(a)||_n(a)||Kl()}function te(a){"@babel/helpers - typeof";return te=typeof Symbol=="function"&&typeof Symbol.iterator=="symbol"?function(n){return typeof n}:function(n){return n&&typeof Symbol=="function"&&n.constructor===Symbol&&n!==Symbol.prototype?"symbol":typeof n},te(a)}function Be(a){return te(a)==="object"&&a!==null&&"attribute"in a}function di(){var a=`
float sdf_circle(vec2 uv, float size) {
  return length(uv) - size;
}
`;return{name:"circle",glsl:a,uniforms:[]}}function ci(a){var n,t=WebGL2RenderingContext,e=t.UNSIGNED_BYTE,r=(n=a?.color)!==null&&n!==void 0?n:{attribute:"color"};if(!Be(r)){var i=r,o=`
vec4 layer_fill() {
  return `.concat(ii(i),`;
}
`);return{name:"fill",uniforms:[],attributes:[],glsl:o}}var s=r.attribute,l=`
vec4 layer_fill(vec4 v_fillColor) {
  return v_fillColor;
}
`;return{name:"fill",uniforms:[],attributes:[{name:"fillColor",size:4,type:e,normalized:!0,source:s}],glsl:l}}function Yl(a,n){if(typeof a=="string"){var t=ai(a).map(function(u){return u.toFixed(6)}),e=ae(t,4),r=e[0],i=e[1],o=e[2],s=e[3];return{glsl:"vec4(".concat(r,", ").concat(i,", ").concat(o,", ").concat(s,")"),attributes:[],needsNodeColors:!1}}if("node"in a&&a.node)return{glsl:a.node==="source"?"v_sourceColor":"v_targetColor",attributes:[],needsNodeColors:!0};var l=a;return{glsl:"v_".concat(n),attributes:[{name:"a_".concat(n),size:4,type:WebGL2RenderingContext.UNSIGNED_BYTE,normalized:!0,source:l.attribute,defaultValue:l.default}],needsNodeColors:!1}}function tt(a){var n=a?.color;if(n===void 0){var t=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return v_color;
}
`;return{name:"plain",glsl:t,uniforms:[],attributes:[]}}var e=Yl(n,"plainColor"),r=`
// Plain solid color layer
vec4 layer_plain(EdgeContext ctx) {
  return `.concat(e.glsl,`;
}
`);return{name:"plain",glsl:r,uniforms:[],attributes:e.attributes,needsNodeColors:e.needsNodeColors}}function hi(){var a=`
// Position at parameter t \u2208 [0, 1]
vec2 path_straight_position(float t, vec2 source, vec2 target) {
  return mix(source, target, t);
}

// Total length of the path (analytical - more efficient than sampling)
float path_straight_length(vec2 source, vec2 target) {
  return length(target - source);
}
`;return{name:"straight",segments:1,minBodyLengthRatio:0,linearParameterization:!0,glsl:a,uniforms:[],attributes:[]}}function fi(a){var n=a??{},t=n.segments,e=t===void 0?32:t,r=`
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
`;return{name:"loop",segments:e,glsl:r,needsNodeSize:!0,uniforms:[],attributes:[{name:"loopRadius",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopAngle",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopSpread",size:1,type:WebGL2RenderingContext.FLOAT},{name:"loopFixedOrientation",size:1,type:WebGL2RenderingContext.FLOAT}],variables:{loopRadius:{type:"number",default:4},loopAngle:{type:"number",default:Math.PI/4},loopSpread:{type:"number",default:80*Math.PI/180},loopFixedOrientation:{type:"number",default:0}},spread:{variable:"loopRadius",compute:function(o){return o+4}}}}var yi=Je(ct());function gi(a){return te(a)==="object"&&"glsl"in a&&!("attributes"in a)}function pi(a){return te(a)==="object"&&"glsl"in a&&!("attributes"in a)}var $t={shapes:[di()],variables:{},layers:[ci()],label:{},backdrop:{},labelAttachments:{}},xt={paths:[hi(),fi()],extremities:[],variables:{},layers:[tt()],defaultHead:"none",defaultTail:"none",label:{}},Sn=["nodes","topNodes"],En=["edges","topEdges"],An=[].concat(En,Sn),vi={nodes:$t,edges:xt,depthLayers:V(An)};var Zl=function(n){return n},Ql=function(n){return n*n},Jl=function(n){return n*(2-n)},eu=function(n){return(n*=2)<1?.5*n*n:-.5*(--n*(n-2)-1)},tu=function(n){return n*n*n},nu=function(n){return--n*n*n+1},au=function(n){return(n*=2)<1?.5*n*n*n:.5*((n-=2)*n*n+2)},ru=function(n){return n===0?0:Math.pow(2,10*(n-1))},iu=function(n){return n===1?1:1-Math.pow(2,-10*n)},ou=function(n){return n===0?0:n===1?1:n<.5?Math.pow(2,10*(2*n-1))/2:(2-Math.pow(2,-10*(2*n-1)))/2},va={linear:Zl,quadraticIn:Ql,quadraticOut:Jl,quadraticInOut:eu,cubicIn:tu,cubicOut:nu,cubicInOut:au,exponentialIn:ru,exponentialOut:iu,exponentialInOut:ou};function wn(a){return a?typeof a=="function"?a:va[a]:va.linear}function ba(a){return te(a)==="object"&&a!==null&&"attribute"in a}function xi(a){return typeof a=="function"}function ya(a){return te(a)==="object"&&a!==null&&"when"in a&&typeof a.when=="function"&&"then"in a}function xa(a){return te(a)==="object"&&a!==null&&"whenState"in a&&"then"in a}function _a(a){return te(a)==="object"&&a!==null&&"whenData"in a&&"then"in a}var su={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,isDragged:!1},lu={isHovered:!1,isLabelHovered:!1,isHidden:!1,isHighlighted:!1,parallelIndex:0,parallelCount:1},uu={isIdle:!0,isPanning:!1,isZooming:!1,isDragging:!1,hasHovered:!1,hasHighlighted:!1};function _i(a){return B(B({},su),a)}function Ti(a){return B(B({},lu),a)}function Ta(a){return B(B({},uu),a)}var Si={x:{attribute:"x"},y:{attribute:"y"},size:{whenState:"isHovered",then:{attribute:"size",defaultValue:12},else:{attribute:"size",defaultValue:10}},color:{attribute:"color",defaultValue:"#666"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"},labelVisibility:{whenState:"isHovered",then:"visible",else:"auto"},backdropVisibility:{whenState:"isHovered",then:"visible",else:"hidden"},backdropColor:"#ffffff",backdropShadowColor:"rgba(0, 0, 0, 0.5)",backdropShadowBlur:12,backdropPadding:6},Ei={size:{attribute:"size",defaultValue:1},color:{attribute:"color",defaultValue:"#ccc"},label:{attribute:"label"},visibility:{whenState:"isHidden",then:"hidden",else:"visible"}},du={nodes:Si,edges:Ei},Dn={nodes:B(B({},Si),{},{depth:{whenState:"isHovered",then:"topNodes",else:"nodes"}}),edges:B(B({},Ei),{},{depth:{whenState:["isHighlighted","isHovered"],then:"topEdges",else:"edges"}})};function cu(a){return"min"in a||"max"in a||"minValue"in a||"maxValue"in a||"easing"in a}function hu(a){return"dict"in a}function jt(a,n){return typeof a=="string"?n[a]===!0:Array.isArray(a)?a.every(function(t){return n[t]===!0}):te(a)==="object"&&a!==null?Object.entries(a).every(function(t){var e=ae(t,2),r=e[0],i=e[1];return n[r]===i}):!1}function fu(a,n){var t=n[a.attribute];return t===void 0?a.defaultValue:t}function gu(a,n,t){var e,r,i,o,s=n[a.attribute];if(s===void 0)return a.defaultValue;var l=Number(s);if(isNaN(l))return a.defaultValue;if(a.min===void 0&&a.max===void 0)return l;var u=(e=a.minValue)!==null&&e!==void 0?e:l,d=(r=a.maxValue)!==null&&r!==void 0?r:l;if(d===u){var c;return(c=a.min)!==null&&c!==void 0?c:l}var h=(l-u)/(d-u);h=Math.max(0,Math.min(1,h));var f=wn(a.easing);h=f(h);var g=(i=a.min)!==null&&i!==void 0?i:0,m=(o=a.max)!==null&&o!==void 0?o:1;return g+h*(m-g)}function pu(a,n){var t=n[a.attribute];if(t===void 0)return a.defaultValue;var e=String(t);return e in a.dict?a.dict[e]:a.defaultValue}function vu(a,n,t){return hu(a)?pu(a,n):cu(a)?gu(a,n):fu(a,n)}function Sa(a,n){return typeof a=="string"?!!n[a]:Array.isArray(a)?a.every(function(t){return!!n[t]}):te(a)==="object"&&a!==null?Object.entries(a).every(function(t){var e=ae(t,2),r=e[0],i=e[1];return n[r]===i}):!1}function Vt(a,n,t,e,r,i){if(a==null)return i;if(te(a)!=="object"&&typeof a!="function")return a;if(ya(a)){var o=a.when(n,t,e,r)?a.then:a.else;return o===void 0?i:Vt(o,n,t,e,r,i)}if(xa(a)){var s=jt(a.whenState,t)?a.then:a.else;return s===void 0?i:Vt(s,n,t,e,r,i)}if(_a(a)){var l=Sa(a.whenData,n)?a.then:a.else;return l===void 0?i:Vt(l,n,t,e,r,i)}if(xi(a)){var u=a(n,t,e,r);return u??i}if(ba(a)){var d=vu(a,n);return d??i}return a}function ke(a){if(a==null)return"static";if(ya(a))return"graph-state";if(xa(a)){var n=ke(a.then),t=a.else!==void 0?ke(a.else):"static";return Re("item-state",Re(n,t))}if(_a(a)){var e=ke(a.then),r=a.else!==void 0?ke(a.else):"static";return Re(e,r)}return xi(a)?"graph-state":(ba(a),"static")}function Re(a,n){return a==="graph-state"||n==="graph-state"?"graph-state":a==="item-state"||n==="item-state"?"item-state":"static"}function Rn(a){if(!a)return{dependency:"static",xAttribute:null,yAttribute:null};var n=Array.isArray(a)?a:[a],t="static",e=null,r=null,i=z(n),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if("matchData"in s&&"cases"in s)for(var l=s.cases,u=0,d=Object.values(l);u<d.length;u++)for(var c=d[u],h=0,f=Object.values(c);h<f.length;h++){var g=f[h];t=Re(t,ke(g))}else if("matchState"in s&&"cases"in s){t=Re(t,"item-state");for(var m=s.cases,v=0,p=Object.values(m);v<p.length;v++)for(var b=p[v],y=0,T=Object.values(b);y<T.length;y++){var x=T[y];t=Re(t,ke(x))}}else if("when"in s&&"then"in s)t="graph-state";else if("whenState"in s){t=Re(t,"item-state");var _=s.then;if(_&&te(_)==="object")for(var A=0,E=Object.values(_);A<E.length;A++){var L=E[A];t=Re(t,ke(L))}var w=s.else;if(w&&te(w)==="object")for(var F=0,D=Object.values(w);F<D.length;F++){var I=D[F];t=Re(t,ke(I))}}else if("whenData"in s){var R=s.then;if(R&&te(R)==="object")for(var P=0,G=Object.values(R);P<G.length;P++){var N=G[P];t=Re(t,ke(N))}var k=s.else;if(k&&te(k)==="object")for(var C=0,M=Object.values(k);C<M.length;C++){var $=M[C];t=Re(t,ke($))}}else for(var H=0,Y=Object.entries(s);H<Y.length;H++){var j=ae(Y[H],2),q=j[0],be=j[1];if(!Aa.has(q)&&(t=Re(t,ke(be)),(q==="x"||q==="y")&&ba(be))){var ve=be.attribute;q==="x"&&!e&&(e=ve),q==="y"&&!r&&(r=ve)}}}}catch(ye){i.e(ye)}finally{i.f()}return{dependency:t,xAttribute:e,yAttribute:r}}var We={size:10,color:"#666",opacity:1,shape:"circle",rotationAlignment:"viewport",labelRotationAlignment:"viewport",visibility:"visible",depth:"nodes",zIndex:0,label:"",labelColor:"#000",labelSize:12,labelFont:"sans-serif",labelVisibility:"auto",labelPosition:"right",labelAngle:0,labelDepth:"nodes",backdropVisibility:"hidden",backdropColor:"transparent",backdropShadowColor:"transparent",backdropShadowBlur:0,backdropPadding:0,backdropBorderColor:"transparent",backdropBorderWidth:0,backdropCornerRadius:0,backdropLabelPadding:-1,backdropArea:"both",labelAttachment:null,labelAttachmentPlacement:"below"},Ea={size:1,color:"#ccc",opacity:1,path:"straight",selfLoopPath:"loop",parallelSpread:.25,tail:"none",head:"none",visibility:"visible",depth:"edges",zIndex:0,label:"",labelColor:"#666",labelVisibility:"auto",labelDepth:"edges"},Aa=new Set(["when","whenState","whenData","then","else"]),mi=Object.keys(We),mu=Object.values(We),bi=Object.keys(Ea),bu=Object.values(Ea);function _t(a,n,t,e,r,i,o){for(var s in n){var l;if(!Aa.has(s)){var u=(l=a[s])!==null&&l!==void 0?l:o[s];a[s]=Vt(n[s],t,e,r,i,u)}}"depth"in n&&!("labelDepth"in n)&&(a.labelDepth=a.depth)}function Ai(a,n,t,e,r,i,o){var s=z(n),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if("matchData"in u&&"cases"in u){var d,c=String((d=t[u.matchData])!==null&&d!==void 0?d:""),h=u.cases;c in h&&_t(a,h[c],t,e,r,i,o)}else if("matchState"in u&&"cases"in u){var f,g=String((f=e[u.matchState])!==null&&f!==void 0?f:""),m=u.cases;g in m&&_t(a,m[g],t,e,r,i,o)}else if("when"in u&&"then"in u){var v=u.when;if(!v(t,e,r,i))continue;_t(a,u.then,t,e,r,i,o)}else if("whenState"in u&&"then"in u){if(!jt(u.whenState,e))continue;_t(a,u.then,t,e,r,i,o)}else if("whenData"in u&&"then"in u){if(!Sa(u.whenData,t))continue;_t(a,u.then,t,e,r,i,o)}else _t(a,u,t,e,r,i,o)}}catch(p){s.e(p)}finally{s.f()}}function Ln(a,n,t,e,r,i){for(var o=i||{},s=o,l=0,u=mi.length;l<u;l++)s[mi[l]]=mu[l];if(s.x=void 0,s.y=void 0,s.labelBackgroundColor=void 0,s.labelBackgroundPadding=void 0,s.labelCursor=void 0,!a){var d,c,h,f,g;return s.x=(d=n.x)!==null&&d!==void 0?d:0,s.y=(c=n.y)!==null&&c!==void 0?c:0,o.size=(h=n.size)!==null&&h!==void 0?h:10,o.color=(f=n.color)!==null&&f!==void 0?f:"#666",o.label=(g=n.label)!==null&&g!==void 0?g:"",o}var m=Array.isArray(a)?a:[a];return Ai(s,m,n,t,e,r,We),o}function Cn(a,n,t,e,r,i){for(var o=i||{},s=o,l=0,u=bi.length;l<u;l++)s[bi[l]]=bu[l];if(!a){var d,c,h;return o.size=(d=n.size)!==null&&d!==void 0?d:1,o.color=(c=n.color)!==null&&c!==void 0?c:"#ccc",o.label=(h=n.label)!==null&&h!==void 0?h:"",o}var f=Array.isArray(a)?a:[a];return Ai(s,f,n,t,e,r,Ea),typeof s.labelPosition=="number"&&(s.labelPosition=void 0),o}function yu(a,n,t){var e,r;if(a==null)return t;if(typeof a=="function")return(e=a(n))!==null&&e!==void 0?e:t;if(te(a)==="object"&&"when"in a){var i,o=a,s=o.when(n),l=s?o.then:o.else;return l===void 0?t:typeof l=="function"?(i=l(n))!==null&&i!==void 0?i:t:l??t}if(te(a)==="object"&&"whenState"in a){var u,d=a,c=jt(d.whenState,n),h=c?d.then:d.else;return h===void 0?t:typeof h=="function"?(u=h(n))!==null&&u!==void 0?u:t:h??t}return(r=a)!==null&&r!==void 0?r:t}function wa(a,n){var t={};if(!a)return t;var e=Array.isArray(a)?a:[a],r=z(e),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;if("when"in o&&"then"in o){var s=o.when(n),l=s?o.then:o.else;l&&te(l)==="object"&&ma(t,l,n)}else if("whenState"in o&&"then"in o){var u=jt(o.whenState,n),d=u?o.then:o.else;d&&te(d)==="object"&&ma(t,d,n)}else ma(t,o,n)}}catch(c){r.e(c)}finally{r.f()}return t}function ma(a,n,t){for(var e=0,r=Object.entries(n);e<r.length;e++){var i=ae(r[e],2),o=i[0],s=i[1];Aa.has(o)||(a[o]=yu(s,t))}}var Xt=(function(a){function n(){var t;return J(this,n),t=le(this,n),t.rawEmitter=t,t}return ue(n,a),ee(n)})(yi.EventEmitter);var Tt=du,Da=vi;function Ra(a,n){var t=n.size;if(t!==0){var e=a.length;a.length+=t;var r=0;n.forEach(function(i){a[e+r]=i,r++})}}function wi(a){a=a||{};for(var n=0,t=arguments.length<=1?0:arguments.length-1;n<t;n++){var e=n+1<1||arguments.length<=n+1?void 0:arguments[n+1];e&&Object.assign(a,e)}return a}function St(a,n){for(var t in n)if(Object.prototype.hasOwnProperty.call(n,t)&&n[t]!==a[t])return!0;return!1}function Di(a,n){var t=a,e=n;for(var r in t)if(Object.prototype.hasOwnProperty.call(t,r)&&t[r]!==e[r])return!1;for(var i in e)if(Object.prototype.hasOwnProperty.call(e,i)&&e[i]!==t[i])return!1;return!0}function nt(a,n,t){t?a.add(n):a.delete(n)}var Li=Je(La());var Ci={easing:"quadraticInOut",duration:150};function Xe(a){return a.labelVisibility==="visible"&&a.visibility!=="hidden"}function Ca(a){return a.backdropVisibility==="visible"&&a.visibility!=="hidden"}function kn(a){return[a.rotationAlignment==="graph"?1:0,a.labelRotationAlignment==="graph"?1:0]}function ka(a,n,t){var e=a[n];if(e)for(var r=0;r<e.length;r++){var i=e[r];if(!(t<i.offset||t>=i.offset+i.count)){if(i.count===1)e.splice(r,1);else if(t===i.offset)i.offset++,i.count--;else if(t===i.offset+i.count-1)i.count--;else{var o=t+1,s=i.offset+i.count-o;i.count=t-i.offset,e.splice(r+1,0,{offset:o,count:s})}return}}}function Pa(a,n,t){if(!a[n]){a[n]=[{offset:t,count:1}];return}for(var e=a[n],r=e.length,i=0;i<e.length;i++)if(t<e[i].offset){r=i;break}var o=r>0?e[r-1]:null,s=r<e.length?e[r]:null,l=o&&o.offset+o.count===t,u=s&&t+1===s.offset;l&&u?(o.count+=1+s.count,e.splice(r,1)):l?o.count++:u?(s.offset--,s.count++):e.splice(r,0,{offset:t,count:1})}function ki(a){if(!(0,Li.default)(a))throw new Error("Sigma: invalid graph instance.")}function Pn(a){var n=ae(a.x,2),t=n[0],e=n[1],r=ae(a.y,2),i=r[0],o=r[1],s=Math.max(e-t,o-i),l=(e+t)/2,u=(o+i)/2;(s===0||Math.abs(s)===1/0||isNaN(s))&&(s=1),isNaN(l)&&(l=0),isNaN(u)&&(u=0);var d=function(h){return{x:.5+(h.x-l)/s,y:.5+(h.y-u)/s}};return d.applyTo=function(c){c.x=.5+(c.x-l)/s,c.y=.5+(c.y-u)/s},d.inverse=function(c){return{x:l+s*(c.x-.5),y:u+s*(c.y-.5)}},d.ratio=s,d}var Fn={hideEdgesOnMove:!1,hideLabelsOnMove:!1,renderLabels:!0,renderEdgeLabels:!1,edgeLabelAnchors:"nodeLabels",enableEdgeEvents:!1,nodeLabelEvents:!1,edgeLabelEvents:!1,pickingDownSizingRatio:2,nodePickingPadding:0,edgePickingPadding:4,labelPickingPadding:10,stagePadding:30,minEdgeThickness:1.7,antiAliasingFeather:1,antialiasEdges:!0,antialiasNodes:!0,dragTimeout:100,draggedEventsTolerance:3,inertiaDuration:200,inertiaRatio:3,zoomDuration:250,zoomingRatio:1.7,doubleClickTimeout:300,doubleClickZoomingRatio:2.2,doubleClickZoomingDuration:200,tapMoveTolerance:10,zoomToSizeRatioFunction:Math.sqrt,itemSizesReference:"positions",autoRescale:!0,autoRescaleContent:"positions",enableNodeDrag:!1,getDraggedNodes:function(n){return[n]},dragPositionToAttributes:null,labelDensity:1,labelGridCellSize:100,labelRenderedSizeThreshold:6,labelPixelSnapping:!0,minCameraRatio:null,maxCameraRatio:null,enableCameraZooming:!0,enableCameraPanning:!0,enableCameraRotation:!0,enableCameraMouseRotation:!0,cameraPanBoundaries:null,gestureTarget:"graph",sharedGestureWheelMessage:"Use Ctrl + scroll to zoom the graph",sharedGestureAppleWheelMessage:"Use \u2318 + scroll to zoom the graph",sharedGestureTouchMessage:"Use two fingers to move the graph",allowInvalidContainer:!1,DEBUG_displayPickingLayer:!1,DEBUG_logShaders:!1,DEBUG_logRenderStats:!1,DEBUG_gpuTimerQueries:!1};function In(a){if(typeof a.labelDensity!="number"||a.labelDensity<0)throw new Error("Settings: invalid `labelDensity`. Expecting a positive number.");var n=a.minCameraRatio,t=a.maxCameraRatio;if(typeof n=="number"&&typeof t=="number"&&t<n)throw new Error("Settings: invalid camera ratio boundaries. Expecting `maxCameraRatio` to be greater than `minCameraRatio`.")}function Pi(a){return wi({},Fn,a)}var xu=new Set(["italic","oblique"]),_u=new Set(["bold","bolder","lighter"]);function Nn(a){for(var n="normal",t="normal",e=a.trim(),r=e.split(/\s+/),i=0,o=0;o<r.length;o++){var s=r[o].toLowerCase();if(xu.has(s))t=s,i=o+1;else if(_u.has(s)||/^\d{3}$/.test(s))n=s,i=o+1;else if(s==="normal")i=o+1;else break}var l=r.slice(i).join(" ");return{family:l||e,weight:n,style:t}}function Et(a,n,t){var e=document.createElement(a);if(n)for(var r in n)e.style[r]=n[r];if(t)for(var i in t)e.setAttribute(i,t[i]);return e}function qt(){return typeof window.devicePixelRatio<"u"?window.devicePixelRatio:1}var Oi=Je(ct());function Tu(a){var n=a.type===WebGL2RenderingContext.FLOAT&&!a.normalized,t=a.type===WebGL2RenderingContext.UNSIGNED_BYTE&&a.size===4&&a.normalized;if(!n&&!t)throw new Error('Attribute "'.concat(a.name,'" is invalid: only non-normalized FLOAT and normalized UNSIGNED_BYTE with size 4 are supported.'));return a.normalized?1:a.size}function Fa(a){var n=0;return a.forEach(function(t){return n+=Tu(t)}),n}function Hi(a,n,t){var e=a==="VERTEX"?n.VERTEX_SHADER:n.FRAGMENT_SHADER,r=n.createShader(e);if(r===null)throw new Error("loadShader: error while creating the shader");n.shaderSource(r,t),n.compileShader(r);var i=n.getShaderParameter(r,n.COMPILE_STATUS);if(!i){var o=n.getShaderInfoLog(r);throw n.deleteShader(r),new Error(`loadShader: error while compiling the shader:
`.concat(o,`
`).concat(t))}return r}function Na(a,n){return Hi("VERTEX",a,n)}function Ga(a,n){return Hi("FRAGMENT",a,n)}function Ma(a,n){var t=a.createProgram();if(t===null)throw new Error("loadProgram: error while creating the program.");var e,r;for(e=0,r=n.length;e<r;e++)a.attachShader(t,n[e]);a.linkProgram(t);var i=a.getProgramParameter(t,a.LINK_STATUS);if(!i){var o=a.getProgramInfoLog(t);throw a.deleteProgram(t),new Error("loadProgram: error while linking the program: ".concat(o))}return t}function Fi(a){var n=a.gl,t=a.buffer,e=a.program,r=a.vertexShader,i=a.fragmentShader;n.deleteShader(r),n.deleteShader(i),n.deleteProgram(e),n.deleteBuffer(t)}function he(a){return a%1===0?a.toFixed(1):a.toString()}var Un=new Map,$i=new Map,Su=0;function Vi(a,n){var t,e=(t=a.variables)===null||t===void 0||(t=t[n.source||n.name.replace(/^a_/,"")])===null||t===void 0?void 0:t.default,r=typeof e=="number"?e:typeof n.defaultValue=="number"?n.defaultValue:0;return he(r)}function On(a,n,t){var e,r=arguments.length>3&&arguments[3]!==void 0?arguments[3]:function(o){return"v_".concat(o.name.replace(/^a_/,""))},i=[n,t].concat(V(a.uniforms.filter(function(o){return o.type==="float"}).map(function(o){var s;return he((s=o.value)!==null&&s!==void 0?s:0)})),V(((e=a.attributes)!==null&&e!==void 0?e:[]).map(r)));return"sdf_".concat(a.name,"(").concat(i.join(", "),")")}function Eu(a){var n,t=a.name,e=a.uniforms.filter(function(o){return o.type==="float"&&o.value!==void 0&&o.value!==0}).map(function(o){return"".concat(o.name.replace("u_",""),"=").concat(o.value)}).sort(),r=((n=a.attributes)!==null&&n!==void 0?n:[]).map(function(o){var s;return"".concat(o.name,"@").concat((s=o.source)!==null&&s!==void 0?s:"","=").concat(Vi(a,o))}),i=[].concat(V(e),V(r));return i.length>0&&(t+="#"+i.join("#")),t}function Au(a){var n=Eu(a);if(!Un.has(n)){var t={},e=z(a.uniforms),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;i.type==="float"&&i.value!==void 0&&(t[i.name]=i.value)}}catch(o){e.e(o)}finally{e.f()}Un.set(n,{shape:a,uniformValues:t,slug:n}),$i.set(n,Su++)}return n}function Jt(a){var n;return(n=$i.get(a))!==null&&n!==void 0?n:-1}function ji(a){var n=[],t=new Set,e=new Set,r=/mat2 rotate2D\(float angle\)\s*\{[^}]+\}/,i=z(a),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;if(!e.has(s.name)){e.add(s.name);var l=s.glsl;r.test(l)&&(t.has("rotate2D")?l=l.replace(r,""):t.add("rotate2D")),n.push(l)}}}catch(u){i.e(u)}finally{i.f()}return n.join(`
`)}function wu(){return ji(Array.from(Un.values()).map(function(a){return a.shape}))}function Du(a){var n=Array.from(Un.entries());if(n.length===0)return`
float querySDF(int shapeId, vec2 uv, float size) {
  return length(uv) - size;
}
`;var t=n.map(function(e,r){var i=ae(e,2),o=i[0],s=i[1].shape,l=On(s,"uv","size",function(u){var d=u.name.replace(/^a_/,"");return a!=null&&a.has(d)?"g_".concat(d):Vi(s,u)});return"    case ".concat(r,": return ").concat(l,"; // ").concat(o)}).join(`
`);return`
float querySDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(t,`
    default: return length(uv) - size;
  }
}
`)}function za(a){return ji(a)}function en(a){return V(new Map(a.flatMap(function(n){return n.uniforms}).map(function(n){return[n.name,n]})).values())}function Ru(a,n){if(a.length===0)return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = length(uv) - size;
  context.inradiusFactor = 1.0;
}
`;var t=function(c){return On(c,"uv","size")},e=function(c){var h,f;return(h=c.inradiusFactorGLSL)!==null&&h!==void 0?h:he((f=c.inradiusFactor)!==null&&f!==void 0?f:1)};if(a.length===1){var r=a[0];return`
void queryNodeSDF(int shapeId, vec2 uv, float size) {
  context.sdf = `.concat(t(r),`;
  context.inradiusFactor = `).concat(e(r),`;
}
`)}var i=a.map(function(d,c){return"    case ".concat(c,": // ").concat(d.name,`
      context.sdf = `).concat(t(d),`;
      context.inradiusFactor = `).concat(e(d),`;
      break;`)}).join(`
`),o=a[0],s="",l="shapeId";if(n&&n.length>1){var u=n.map(function(d,c){return"    case ".concat(d,": return ").concat(c,"; // ").concat(a[c].name)}).join(`
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
`)}var wt={right:0,left:1,above:2,below:3,over:4},Dt=5,Ba=3,Xi=`
float matrixScaleX = length(vec2(u_matrix[0][0], u_matrix[1][0]));
float nodeRadiusGraphSpace = nodeSize * u_correctionRatio / u_sizeRatio * 2.0;
float nodeRadiusNDC = nodeRadiusGraphSpace * matrixScaleX;
float nodeRadiusPixels = nodeRadiusNDC * u_resolution.x / 2.0;
`,Lu=`
mat2 rotate2D(float angle) {
  float c = cos(angle);
  float s = sin(angle);
  return mat2(c, -s, s, c);
}
`,qi=`
vec2 getLabelDirection(float positionMode) {
  if (positionMode < 0.5) return vec2(1.0, 0.0);   // Right
  if (positionMode < 1.5) return vec2(-1.0, 0.0);  // Left
  if (positionMode < 2.5) return vec2(0.0, -1.0);  // Above
  if (positionMode < 3.5) return vec2(0.0, 1.0);   // Below
  return vec2(0.0);                                 // Over (centered)
}
`,Cu=`
float sdfBox(vec2 p, vec2 halfSize) {
  vec2 d = abs(p) - halfSize;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}
`,ku=`
float sdfRotatedBox(vec2 p, vec2 halfSize, float angle) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfBox(rotatedP, halfSize);
}
`,Pu=`
float sdfRoundedBox(vec2 p, vec2 halfSize, float radius) {
  vec2 d = abs(p) - halfSize + radius;
  return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - radius;
}
`,Fu=`
float sdfRoundedRotatedBox(vec2 p, vec2 halfSize, float angle, float radius) {
  float c = cos(-angle);
  float s = sin(-angle);
  vec2 rotatedP = mat2(c, -s, s, c) * p;
  return sdfRoundedBox(rotatedP, halfSize, radius);
}
`,Ki=`
vec2 labelBoxCenter(float positionMode, float labelStart, vec2 halfSize, float textHalfY) {
  if (positionMode < 0.5) return vec2(labelStart + halfSize.x, 0.0);    // right
  if (positionMode < 1.5) return vec2(-(labelStart + halfSize.x), 0.0); // left
  if (positionMode < 2.5) return vec2(0.0, -(labelStart + textHalfY));  // above
  if (positionMode < 3.5) return vec2(0.0, labelStart + textHalfY);     // below
  return vec2(0.0);                                                     // over
}
`,Wa=2,Ue=`
vec4 readNodeData(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Wa,`;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),rt=`
vec4 readNodeFlags(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Wa,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  return texelFetch(nodeDataTexture, coord, 0);
}
`),Iu=`
vec4 readNodeColor(sampler2D nodeDataTexture, int nodeDataTextureWidth, int nodeIndex) {
  int t = nodeIndex * `.concat(Wa,` + 1;
  ivec2 coord = ivec2(t % nodeDataTextureWidth, t / nodeDataTextureWidth);
  vec4 texel = texelFetch(nodeDataTexture, coord, 0);
  float r = floor(texel.b / 65536.0);
  float g = floor(mod(texel.b, 65536.0) / 256.0);
  float b = mod(texel.b, 256.0);
  return vec4(r / 255.0, g / 255.0, b / 255.0, texel.a);
}
`),Rt=`
vec4 readFrameTexel(sampler2D frameTexture, int frameTextureWidth, int index) {
  ivec2 coord = ivec2(index % frameTextureWidth, index / frameTextureWidth);
  return texelFetch(frameTexture, coord, 0);
}
`;function Nu(a,n){var t=function(o){return On(o,"uv","size")};if(a.length===1)return{code:Ii(t(a[0])),multiShape:!1};var e=a.map(function(i,o){return"    case ".concat(n?n[o]:o,": return ").concat(t(i),";")}).join(`
`),r=`
float queryShapeSDF(int shapeId, vec2 uv, float size) {
  switch (shapeId) {
`.concat(e,`
    default: return `).concat(t(a[0]),`;
  }
}
int g_shapeId;
`).concat(Ii("queryShapeSDF(g_shapeId, uv, size)"),`
`);return{code:r,multiShape:!0}}function Ii(a){return`
float findEdgeDistance(vec2 direction, float size) {
  float lo = 0.0, hi = 2.0;
  for (int i = 0; i < 8; i++) {
    float mid = (lo + hi) * 0.5;
    vec2 uv = direction * mid;
    if (`.concat(a,` < 0.0) lo = mid; else hi = mid;
  }
  return (lo + hi) * 0.5;
}
`)}function Gu(a,n){for(;!{}.hasOwnProperty.call(a,n)&&(a=et(a))!==null;);return a}function Ia(){return Ia=typeof Reflect<"u"&&Reflect.get?Reflect.get.bind():function(a,n,t){var e=Gu(a,n);if(e){var r=Object.getOwnPropertyDescriptor(e,n);return r.get?r.get.call(arguments.length<3?a:t):r.value}},Ia.apply(null,arguments)}function ge(a,n,t,e){var r=Ia(et(1&e?a.prototype:a),n,t);return 2&e&&typeof r=="function"?function(i){return r.apply(t,i)}:r}var Mu=1024,zu=1.5,Yi=-1,Hn=(function(){function a(n,t){var e=arguments.length>2&&arguments[2]!==void 0?arguments[2]:Mu;J(this,a),S(this,"texture",null),S(this,"dirty",!1),S(this,"dirtyRangeStart",1/0),S(this,"dirtyRangeEnd",-1),S(this,"indexMap",new Map),S(this,"freeIndices",[]),S(this,"nextIndex",0),this.gl=n,this.maxTextureSize=n.getParameter(n.MAX_TEXTURE_SIZE),this.TEXELS_PER_ITEM=t,this.capacity=this.roundUpToPowerOfTwo(e);var r=this.computeTextureDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.data=new Float32Array(this.textureWidth*this.textureHeight*4),this.createTexture()}return ee(a,[{key:"computeTextureDimensions",value:function(t){var e=t*this.TEXELS_PER_ITEM,r=Math.min(e,this.maxTextureSize),i=Math.ceil(e/r);return{width:r,height:i}}},{key:"roundUpToPowerOfTwo",value:function(t){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,t))))}},{key:"createTexture",value:function(){var t=this.gl;this.texture=t.createTexture(),t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture),t.texImage2D(t.TEXTURE_2D,0,t.RGBA32F,this.textureWidth,this.textureHeight,0,t.RGBA,t.FLOAT,this.data),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_S,t.CLAMP_TO_EDGE),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_T,t.CLAMP_TO_EDGE),t.bindTexture(t.TEXTURE_2D,null)}},{key:"resize",value:function(t){if(!(t<=this.capacity)){var e=this.roundUpToPowerOfTwo(Math.ceil(t*zu)),r=this.gl,i=this.computeTextureDimensions(e),o=new Float32Array(i.width*i.height*4);o.set(this.data),this.texture&&r.deleteTexture(this.texture),this.data=o,this.capacity=e,this.textureWidth=i.width,this.textureHeight=i.height,this.createTexture(),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.nextIndex}}},{key:"allocate",value:function(t){var e=this.indexMap.get(t);if(e!==void 0)return e;var r;return this.freeIndices.length>0?r=this.freeIndices.pop():(r=this.nextIndex++,r>=this.capacity&&this.resize(r+1)),this.indexMap.set(t,r),r}},{key:"free",value:function(t){var e=this.indexMap.get(t);if(e!==void 0){this.indexMap.delete(t),this.freeIndices.push(e);for(var r=e*this.TEXELS_PER_ITEM*4,i=0;i<this.TEXELS_PER_ITEM*4;i++)this.data[r+i]=0;this.markDirty(e)}}},{key:"getIndex",value:function(t){var e;return(e=this.indexMap.get(t))!==null&&e!==void 0?e:-1}},{key:"has",value:function(t){return this.indexMap.has(t)}},{key:"markDirty",value:function(t){this.dirty=!0,this.dirtyRangeStart=Math.min(this.dirtyRangeStart,t),this.dirtyRangeEnd=Math.max(this.dirtyRangeEnd,t+1)}},{key:"upload",value:function(){if(!(!this.dirty||!this.texture)){var t=this.gl,e=this.textureWidth;t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture);var r=this.dirtyRangeStart*this.TEXELS_PER_ITEM,i=Math.min(this.dirtyRangeEnd*this.TEXELS_PER_ITEM,this.capacity*this.TEXELS_PER_ITEM);if(r<i)for(var o=Math.floor(r/e),s=Math.floor((i-1)/e),l=o;l<=s;l++){var u=l*e,d=Math.min(u+e,this.capacity*this.TEXELS_PER_ITEM),c=Math.max(r,u),h=Math.min(i,d);if(c<h){var f=c-u,g=h-c,m=this.data.subarray(c*4,h*4);t.texSubImage2D(t.TEXTURE_2D,0,f,l,g,1,t.RGBA,t.FLOAT,m)}}t.bindTexture(t.TEXTURE_2D,null),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}}},{key:"bind",value:function(t){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getCapacity",value:function(){return this.capacity}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"getTexelsPerItem",value:function(){return this.TEXELS_PER_ITEM}},{key:"getCount",value:function(){return this.indexMap.size}},{key:"getHighWaterMark",value:function(){return this.nextIndex}},{key:"isDirty",value:function(){return this.dirty}},{key:"clear",value:function(){this.indexMap.clear(),this.freeIndices=[],this.nextIndex=0,this.data.fill(0),this.dirty=!0,this.dirtyRangeStart=0,this.dirtyRangeEnd=this.capacity}},{key:"restore",value:function(){this.createTexture(),this.dirty=!1,this.dirtyRangeStart=1/0,this.dirtyRangeEnd=-1}},{key:"kill",value:function(){this.texture&&(this.gl.deleteTexture(this.texture),this.texture=null),this.indexMap.clear(),this.freeIndices=[]}}])})();function Le(a){var n={},t={},e=0,r=z(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=z(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");d in n||(n[d]=e,t[d]=u,e+=u.size)}}catch(c){s.e(c)}finally{s.f()}}}catch(c){r.e(c)}finally{r.f()}return{floatsPerItem:e,texelsPerItem:Math.max(1,Math.ceil(e/4)),offsets:n,specs:t}}var $n=(function(a){function n(t,e,r){var i;return J(this,n),i=le(this,n,[t,e.texelsPerItem,r]),i.floatsPerItem=e.floatsPerItem,i}return ue(n,a),ee(n,[{key:"updateAllAttributes",value:function(e,r){var i=this.indexMap.get(e);i===void 0&&(i=this.allocate(e));for(var o=i*this.TEXELS_PER_ITEM*4,s=Math.min(r.length,this.floatsPerItem),l=0;l<s;l++)this.data[o+l]=r[l];this.markDirty(i)}},{key:"updateAllAttributesAtRow",value:function(e,r){e>=this.capacity&&this.resize(e+1);for(var i=e*this.TEXELS_PER_ITEM*4,o=Math.min(r.length,this.floatsPerItem),s=0;s<o;s++)this.data[i+s]=r[s];this.markDirty(e)}}])})(Hn);function Vn(a,n,t){for(var e=[],r=new Set,i=0;i<a.length;i++){var o=a[i],s=z(o.attributes),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.name.replace(/^a_/,"");if(!r.has(d)){r.add(d);var c=n.offsets[d];if(c!==void 0){var h=t?.get(i);e.push({sourceKey:u.source||d,packedOffset:c,size:u.size,isColor:u.size===4&&!!u.normalized,defaultNum:typeof u.defaultValue=="number"?u.defaultValue:u.defaultValue===!0?1:0,defaultColor:typeof u.defaultValue=="string"?u.defaultValue:"",sourceIndex:i,hasLifecycleHook:!!(h!=null&&h.getAttributeData)})}}}}catch(f){s.e(f)}finally{s.f()}}return e}function jn(a,n,t,e,r,i){t.fill(0);for(var o=0,s=a.length;o<s;o++){var l=a[o],u=l.packedOffset,d=void 0;if(l.hasLifecycleHook){var c=r.get(l.sourceIndex-i);d=c.getAttributeData(n,l.sourceKey),d===null&&(d=n[l.sourceKey])}else d=n[l.sourceKey];if(l.isColor){var h=typeof d=="string"?d:l.defaultColor||e,f=je(h),g=ae(f,4),m=g[0],v=g[1],p=g[2],b=g[3];t[u]=m/255,t[u+1]=v/255,t[u+2]=p/255,t[u+3]=b/255}else if(l.size===1)t[u]=typeof d=="number"?d:typeof d=="boolean"?d?1:0:l.defaultNum;else{var y=Array.isArray(d)?d:null;if(y)for(var T=0;T<l.size;T++){var x;t[u+T]=(x=y[T])!==null&&x!==void 0?x:0}}}}var Gn=["r","g","b","a"];function At(a,n){var t=a.offsets,e=a.specs,r=a.texelsPerItem,i=a.floatsPerItem,o=Object.keys(t);if(o.length===0||i===0)return{fetchCode:"",varyingAssignments:""};var s=n.varPrefix,l=n.baseTexelExpr,u=n.textureWidthUniform,d=n.textureSamplerUniform,c=n.outputPrefix,h=c===void 0?"v_":c,f=[];f.push("  int ".concat(s,"BaseTexel = ").concat(l,";")),f.push("");for(var g=0;g<r;g++)f.push("  ivec2 ".concat(s,"Coord").concat(g," = ivec2((").concat(s,"BaseTexel + ").concat(g,") % ").concat(u,", (").concat(s,"BaseTexel + ").concat(g,") / ").concat(u,");")),f.push("  vec4 ".concat(s,"Texel").concat(g," = texelFetch(").concat(d,", ").concat(s,"Coord").concat(g,", 0);"));f.push("");for(var m=[],v=function(){var T=b[p],x=e[T],_=t[T],A=Math.floor(_/4),E=_%4,L="".concat(s,"Texel").concat(A),w="".concat(s,"Texel").concat(A+1),F="".concat(s,"Fetched_").concat(T);if(x.size===1)f.push("  float ".concat(F," = ").concat(L,".").concat(Gn[E],";"));else if(x.size===4&&E===0)f.push("  vec4 ".concat(F," = ").concat(L,";"));else{var D=E+x.size;if(D<=4){var I="vec".concat(x.size),R=Gn.slice(E,D).join("");f.push("  ".concat(I," ").concat(F," = ").concat(L,".").concat(R,";"))}else{var P=x.size===4?"vec4":"vec".concat(x.size),G=Gn.slice(E).map(function(C){return"".concat(L,".").concat(C)}),N=Gn.slice(0,D-4).map(function(C){return"".concat(w,".").concat(C)}),k=[].concat(V(G),V(N)).join(", ");f.push("  ".concat(P," ").concat(F," = ").concat(P,"(").concat(k,");"))}}m.push("  ".concat(h).concat(T," = ").concat(F,";"))},p=0,b=o;p<b.length;p++)v();return{fetchCode:f.join(`
`),varyingAssignments:m.join(`
`)}}function Ua(a,n,t){if(!(!n||t.type==="sampler2D"))switch(t.type){case"float":a.uniform1f(n,t.value);break;case"int":case"bool":a.uniform1i(n,t.value);break;case"vec2":a.uniform2fv(n,t.value);break;case"vec3":a.uniform3fv(n,t.value);break;case"vec4":a.uniform4fv(n,t.value);break;case"mat3":a.uniformMatrix3fv(n,!1,t.value);break;case"mat4":a.uniformMatrix4fv(n,!1,t.value);break}}var Bu=S(S(S(S(S(S(S(S({},WebGL2RenderingContext.BOOL,1),WebGL2RenderingContext.BYTE,1),WebGL2RenderingContext.UNSIGNED_BYTE,1),WebGL2RenderingContext.SHORT,2),WebGL2RenderingContext.UNSIGNED_SHORT,2),WebGL2RenderingContext.INT,4),WebGL2RenderingContext.UNSIGNED_INT,4),WebGL2RenderingContext.FLOAT,4);function Mn(a){var n=a.match(/^(#version[^\n]*\n)/);return n?n[1]+`#define PICKING_MODE
`+a.slice(n[1].length):`#define PICKING_MODE
`+a}var it=(function(){function a(n,t,e){J(this,a),S(this,"floats",new Float32Array),S(this,"ints",new Uint32Array),S(this,"constantArray",new Float32Array),S(this,"capacity",0),S(this,"verticesCount",0),S(this,"bufferGeneration",0),S(this,"uploadedGeneration",new Map),S(this,"constantBufferGeneration",0),S(this,"uploadedConstantGeneration",new Map),S(this,"renderOffset",0),S(this,"renderCount",-1),S(this,"debugStats",{drawCalls:0,verticesDrawn:0,bufferUploadBytes:0}),S(this,"shadersLogged",!1),S(this,"pickProgram",null);var r=this.getDefinition();if(this.VERTICES=r.VERTICES,this.VERTEX_SHADER_SOURCE=r.VERTEX_SHADER_SOURCE,this.FRAGMENT_SHADER_SOURCE=r.FRAGMENT_SHADER_SOURCE,this.UNIFORMS=r.UNIFORMS,this.ATTRIBUTES=r.ATTRIBUTES,this.METHOD=r.METHOD,this.CONSTANT_ATTRIBUTES="CONSTANT_ATTRIBUTES"in r?r.CONSTANT_ATTRIBUTES:[],this.CONSTANT_DATA="CONSTANT_DATA"in r?r.CONSTANT_DATA:[],this.isInstanced="CONSTANT_ATTRIBUTES"in r,this.ATTRIBUTES_ITEMS_COUNT=Fa(this.ATTRIBUTES),this.STRIDE=this.VERTICES*this.ATTRIBUTES_ITEMS_COUNT,this.renderer=e,this.normalProgram=this.getProgramInfo("normal",n,r.VERTEX_SHADER_SOURCE,r.FRAGMENT_SHADER_SOURCE,null),this.pickProgram=this.getProgramInfo("pick",n,Mn(r.VERTEX_SHADER_SOURCE),Mn(r.FRAGMENT_SHADER_SOURCE),null),this.isInstanced){var i=Fa(this.CONSTANT_ATTRIBUTES);if(this.CONSTANT_DATA.length!==this.VERTICES)throw new Error("Program: error while getting constant data (expected ".concat(this.VERTICES," items, received ").concat(this.CONSTANT_DATA.length," instead)"));this.constantArray=new Float32Array(this.CONSTANT_DATA.length*i);for(var o=0;o<this.CONSTANT_DATA.length;o++){var s=this.CONSTANT_DATA[o];if(s.length!==i)throw new Error("Program: error while getting constant data (one vector has ".concat(s.length," items instead of ").concat(i,")"));for(var l=0;l<s.length;l++)this.constantArray[o*i+l]=s[l]}this.STRIDE=this.ATTRIBUTES_ITEMS_COUNT}}return ee(a,[{key:"kill",value:function(){Fi(this.normalProgram),this.pickProgram&&Fi(this.pickProgram)}},{key:"resetDebugStats",value:function(){this.debugStats.drawCalls=0,this.debugStats.verticesDrawn=0,this.debugStats.bufferUploadBytes=0}},{key:"getProgramInfo",value:function(t,e,r,i,o){var s=e.createBuffer();if(s===null)throw new Error("Program: error while creating the WebGL buffer.");var l=Na(e,r),u=Ga(e,i),d=Ma(e,[l,u]),c={};this.UNIFORMS.forEach(function(g){var m=e.getUniformLocation(d,g);m&&(c[g]=m)});var h={};this.ATTRIBUTES.forEach(function(g){h[g.name]=e.getAttribLocation(d,g.name)});var f;if(this.isInstanced&&(this.CONSTANT_ATTRIBUTES.forEach(function(g){h[g.name]=e.getAttribLocation(d,g.name)}),f=e.createBuffer(),f===null))throw new Error("Program: error while creating the WebGL constant buffer.");return{name:t,program:d,gl:e,frameBuffer:o,buffer:s,constantBuffer:f||{},uniformLocations:c,attributeLocations:h,isPicking:t==="pick",vertexShader:l,fragmentShader:u}}},{key:"bindProgram",value:function(t){var e=this,r=0,i=t.gl,o=t.buffer;if(this.isInstanced){if(i.bindBuffer(i.ARRAY_BUFFER,t.constantBuffer),r=0,this.CONSTANT_ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r,!1)}),this.uploadedConstantGeneration.get(t.constantBuffer)!==this.constantBufferGeneration){var l;i.bufferData(i.ARRAY_BUFFER,this.constantArray,i.STATIC_DRAW),this.uploadedConstantGeneration.set(t.constantBuffer,this.constantBufferGeneration),(l=this.renderer)!==null&&l!==void 0&&l.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.constantArray.byteLength)}if(i.bindBuffer(i.ARRAY_BUFFER,t.buffer),r=this.renderOffset*this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT,this.ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r,!0)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var u;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(u=this.renderer)!==null&&u!==void 0&&u.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}}else if(i.bindBuffer(i.ARRAY_BUFFER,o),r=0,this.ATTRIBUTES.forEach(function(d){return r+=e.bindAttribute(d,t,r)}),this.uploadedGeneration.get(o)!==this.bufferGeneration){var s;i.bufferData(i.ARRAY_BUFFER,this.floats,i.DYNAMIC_DRAW),this.uploadedGeneration.set(o,this.bufferGeneration),(s=this.renderer)!==null&&s!==void 0&&s.getSetting("DEBUG_logRenderStats")&&(this.debugStats.bufferUploadBytes+=this.floats.byteLength)}i.bindBuffer(i.ARRAY_BUFFER,null)}},{key:"unbindProgram",value:function(t){var e=this;this.isInstanced?(this.CONSTANT_ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t,!1)}),this.ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t,!0)})):this.ATTRIBUTES.forEach(function(r){return e.unbindAttribute(r,t)})}},{key:"bindAttribute",value:function(t,e,r,i){var o=Bu[t.type];if(typeof o!="number")throw new Error('Program.bind: yet unsupported attribute type "'.concat(t.type,'"'));var s=e.attributeLocations[t.name],l=e.gl;if(s!==-1){l.enableVertexAttribArray(s);var u=this.isInstanced?(i?this.ATTRIBUTES_ITEMS_COUNT:Fa(this.CONSTANT_ATTRIBUTES))*Float32Array.BYTES_PER_ELEMENT:this.ATTRIBUTES_ITEMS_COUNT*Float32Array.BYTES_PER_ELEMENT;l.vertexAttribPointer(s,t.size,t.type,t.normalized||!1,u,r),this.isInstanced&&i&&l.vertexAttribDivisor(s,1)}return t.size*o}},{key:"unbindAttribute",value:function(t,e,r){var i=e.attributeLocations[t.name],o=e.gl;i!==-1&&(o.disableVertexAttribArray(i),this.isInstanced&&r&&o.vertexAttribDivisor(i,0))}},{key:"reallocate",value:function(t){t!==this.capacity&&(this.capacity=t,this.verticesCount=this.VERTICES*t,this.floats=new Float32Array(this.isInstanced?this.capacity*this.ATTRIBUTES_ITEMS_COUNT:this.verticesCount*this.ATTRIBUTES_ITEMS_COUNT),this.ints=new Uint32Array(this.floats.buffer),this.invalidateBuffers())}},{key:"invalidateBuffers",value:function(){this.bufferGeneration++,this.constantBufferGeneration++}},{key:"hasNothingToRender",value:function(){return this.verticesCount===0}},{key:"setTypedUniform",value:function(t,e){var r;Ua(e.gl,(r=e.uniformLocations[t.name])!==null&&r!==void 0?r:null,t)}},{key:"renderProgram",value:function(t,e){var r=e.gl,i=e.program,o=e.isPicking;o?r.disable(r.BLEND):r.enable(r.BLEND),r.useProgram(i),this.setUniforms(t,e),this.drawWebGL(this.METHOD,e)}},{key:"render",value:function(t,e,r){var i;if(!this.shadersLogged&&(i=this.renderer)!==null&&i!==void 0&&i.getSetting("DEBUG_logShaders")&&(this.shadersLogged=!0,console.log("[sigma] DEBUG_logShaders: ".concat(this.constructor.name),{normal:{vertexShaderSource:this.VERTEX_SHADER_SOURCE,fragmentShaderSource:this.FRAGMENT_SHADER_SOURCE},pick:{vertexShaderSource:Mn(this.VERTEX_SHADER_SOURCE),fragmentShaderSource:Mn(this.FRAGMENT_SHADER_SOURCE)}})),!this.hasNothingToRender()){this.renderOffset=e??0,this.renderCount=r??-1;var o=this.normalProgram.gl;if(o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,t.width*t.pixelRatio,t.height*t.pixelRatio),this.bindProgram(this.normalProgram),this.renderProgram(t,this.normalProgram),this.unbindProgram(this.normalProgram),this.pickProgram&&t.pickingFrameBuffer){var s=Math.ceil(t.width*t.pixelRatio/t.downSizingRatio),l=Math.ceil(t.height*t.pixelRatio/t.downSizingRatio);o.bindFramebuffer(o.FRAMEBUFFER,t.pickingFrameBuffer),o.viewport(0,0,s,l),this.bindProgram(this.pickProgram),this.renderProgram(t,this.pickProgram),this.unbindProgram(this.pickProgram),o.bindFramebuffer(o.FRAMEBUFFER,null),o.viewport(0,0,t.width*t.pixelRatio,t.height*t.pixelRatio)}}}},{key:"drawWebGL",value:function(t,e){var r,i=e.gl,o=this.renderCount>=0?this.renderCount:this.capacity;(r=this.renderer)!==null&&r!==void 0&&r.getSetting("DEBUG_logRenderStats")&&(this.debugStats.drawCalls++,this.debugStats.verticesDrawn+=o*this.VERTICES),this.isInstanced?i.drawArraysInstanced(t,0,this.VERTICES,o):i.drawArrays(t,this.renderOffset*this.VERTICES,o*this.VERTICES)}}])})(),Ye=2,tn=7,Zi={below:0,above:1,left:2,right:3};function Wu(){var a=`#version 300 es
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

`.concat(Ue,`
`).concat(rt,`
`).concat(Rt,`
`).concat(Ki,`
`).concat(Lu,`

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
  float gap = `).concat(Ye.toFixed(1),` * zoomScale * u_pixelRatio;
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
`);return a}var Uu=`#version 300 es
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
`;function Ou(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Dt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=Wu(),c=Uu,h=(function(f){function g(){var m;J(this,g);for(var v=arguments.length,p=new Array(v),b=0;b<v;b++)p[b]=arguments[b];return m=le(this,g,[].concat(p)),S(m,"totalCount",0),S(m,"bufferCapacity",0),m}return ue(g,f),ee(g,[{key:"getDefinition",value:function(){var v=WebGL2RenderingContext,p=v.FLOAT,b=v.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:c,METHOD:b,UNIFORMS:["u_matrix","u_resolution","u_labelPixelSnapping","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_atlasTexture"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:p},{name:"a_atlasRect",size:4,type:p},{name:"a_attachmentSize",size:2,type:p},{name:"a_positionMode",size:1,type:p},{name:"a_attachmentPlacement",size:1,type:p},{name:"a_labelWidth",size:1,type:p},{name:"a_labelHeight",size:1,type:p},{name:"a_textHeight",size:1,type:p},{name:"a_labelAngle",size:1,type:p}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:p}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processAttachment",value:function(v,p){var b=this.floats,y=v*this.STRIDE;b[y++]=p.nodeIndex,b[y++]=p.atlasX,b[y++]=p.atlasY,b[y++]=p.atlasW,b[y++]=p.atlasH,b[y++]=p.attachWidth,b[y++]=p.attachHeight,b[y++]=p.positionMode,b[y++]=p.attachmentPlacement,b[y++]=p.labelWidth,b[y++]=p.labelHeight,b[y++]=p.textHeight,b[y++]=p.labelAngle}},{key:"setUniforms",value:function(v,p){var b=p.gl,y=p.uniformLocations;b.uniformMatrix3fv(y.u_matrix,!1,v.matrix),b.uniform2f(y.u_resolution,v.width*v.pixelRatio,v.height*v.pixelRatio),b.uniform1f(y.u_labelPixelSnapping,v.labelPixelSnapping),b.uniform1f(y.u_sizeRatio,v.sizeRatio),b.uniform1f(y.u_correctionRatio,v.correctionRatio),b.uniform1f(y.u_cameraAngle,v.cameraAngle),b.uniform1f(y.u_pixelRatio,v.pixelRatio),b.uniform1f(y.u_labelMargin,g.labelMargin),b.uniform1f(y.u_zoomLabelSizeRatio,1/u(v.zoomRatio)),b.uniform1i(y.u_nodeDataTexture,v.nodeDataTextureUnit),b.uniform1i(y.u_nodeDataTextureWidth,v.nodeDataTextureWidth),b.uniform1i(y.u_nodeFrameTexture,v.nodeFrameTextureUnit),b.uniform1i(y.u_nodeFrameTextureWidth,v.nodeFrameTextureWidth),b.uniform1i(y.u_atlasTexture,tn)}},{key:"reallocateAttachments",value:function(v){this.totalCount=v,v>this.bufferCapacity&&(this.bufferCapacity=Math.max(v,Math.ceil(this.bufferCapacity*1.5)||10),ge(g,"reallocate",this,3)([this.bufferCapacity]))}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(v,p){var b=p.gl;this.totalCount!==0&&b.drawArraysInstanced(v,0,this.VERTICES,this.totalCount)}}])})(it);return S(h,"labelMargin",l),new h(a,n,t)}var qe=5;function Xn(a,n){return[].concat(V(n),V(a.map(function(t){var e;return{attributes:(e=t.attributes)!==null&&e!==void 0?e:[]}})))}function Lt(a,n){var t=Le(Xn(a,n)),e=new Set(a.flatMap(function(u){var d;return(d=u.attributes)!==null&&d!==void 0?d:[]}).map(function(u){return u.name.replace(/^a_/,"")})),r={},i={},o=z(e),s;try{for(o.s();!(s=o.n()).done;){var l=s.value;r[l]=t.offsets[l],i[l]=t.specs[l]}}catch(u){o.e(u)}finally{o.f()}return B(B({},t),{},{offsets:r,specs:i})}function Hu(a,n){var t=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_pickingPadding","u_nodeDataTexture","u_layerAttributeTexture"]),e=new Set,r=a.flatMap(function(f){return f.uniforms}).filter(function(f){return t.has(f.name)||e.has(f.name)?!1:(e.add(f.name),!0)}).map(function(f){return"uniform ".concat(f.type," ").concat(f.name,";")}).join(`
`),i=n.flatMap(function(f){return f.uniforms}).filter(function(f){return t.has(f.name)||e.has(f.name)?!1:(e.add(f.name),!0)}).map(function(f){return"uniform ".concat(f.type," ").concat(f.name,";")}).join(`
`),o=Xn(a,n),s=new Set,l=o.flatMap(function(f){return f.attributes}).filter(function(f){var g=f.name.replace(/^a_/,"");return s.has(g)?!1:(s.add(g),!0)}).map(function(f){var g=f.name.replace(/^a_/,""),m=f.size===1?"float":"vec".concat(f.size);return"out ".concat(m," v_").concat(g,";")}).join(`
`),u=At(Le(o),{varPrefix:"layer",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),d=u.fetchCode,c=u.varyingAssignments,h=`#version 300 es

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
`).concat(Ue,`
`).concat(rt,`

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
`).concat(c,`
}
`);return h}function $u(a,n,t){var e=arguments.length>3&&arguments[3]!==void 0?arguments[3]:!0,r=n.map(function(g,m){var v="layer_".concat(g.name),p=[].concat(V(g.attributes.map(function(b){return"v_".concat(b.name.replace(/^a_/,""))})),V(g.uniforms.map(function(b){return b.name}))).join(", ");return"  // Layer ".concat(m+1,": ").concat(g.name,`
  color = blendOver(color, `).concat(v,"(").concat(p,"));")}).join(`

`),i=new Set(["u_correctionRatio"]),o=new Set,s=a.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),l=n.flatMap(function(g){return g.uniforms}).filter(function(g){return i.has(g.name)||o.has(g.name)?!1:(o.add(g.name),!0)}).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),u=new Set,d=Xn(a,n).flatMap(function(g){return g.attributes}).filter(function(g){var m=g.name.replace(/^a_/,"");return u.has(m)?!1:(u.add(m),!0)}).map(function(g){var m=g.name.replace(/^a_/,""),v=g.size===1?"float":"vec".concat(g.size);return"in ".concat(v," v_").concat(m,";")}).join(`
`),c=za(a),h=Ru(a,t),f=`#version 300 es
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
`).concat(c,`

// Shape selector function (sets context.sdf and context.inradiusFactor)
`).concat(h,`

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
`);return f}function Vu(a,n){var t=new Set;return t.add("u_matrix"),t.add("u_sizeRatio"),t.add("u_correctionRatio"),t.add("u_cameraAngle"),t.add("u_pickingPadding"),t.add("u_nodeDataTexture"),t.add("u_nodeDataTextureWidth"),t.add("u_layerAttributeTexture"),t.add("u_layerAttributeTextureWidth"),t.add("u_layerAttributeTexelsPerNode"),a.forEach(function(e){e.uniforms.forEach(function(r){return t.add(r.name)})}),n.forEach(function(e){e.uniforms.forEach(function(r){return t.add(r.name)})}),Array.from(t)}function ju(a){var n=WebGL2RenderingContext,t=n.UNSIGNED_BYTE,e=n.FLOAT;return[{name:"a_nodeIndex",size:1,type:e},{name:"a_id",size:4,type:t,normalized:!0},{name:"a_opacity",size:1,type:e}]}function Ni(a){var n=a.shapes,t=a.layers,e=a.shapeGlobalIds,r=a.antialias,i=r===void 0?!0:r;return{vertexShader:Hu(n,t),fragmentShader:$u(n,t,e,i),uniforms:Vu(n,t),attributes:ju()}}function Qi(a,n,t){var e=Lt(a,n),r=e.specs;return Object.keys(r).map(function(i){return"flat ".concat(t," ").concat(r[i].size===1?"float":"vec".concat(r[i].size)," v_").concat(i,";")}).join(`
`)}function Xu(a){var n,t,e=a.shapes,r=a.layers,i=a.shapeGlobalIds,o=At(Lt(e,r),{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),s=e.length===1?"float inradiusFactor = ".concat(he((n=e[0].inradiusFactor)!==null&&n!==void 0?n:1),";"):"float inradiusFactor = ".concat(he((t=e[0].inradiusFactor)!==null&&t!==void 0?t:1),`;
  switch (int(shapeId)) {
`).concat(e.map(function(u,d){var c,h=i?i[d]:d;return"    case ".concat(h,": inradiusFactor = ").concat(he((c=u.inradiusFactor)!==null&&c!==void 0?c:1),"; break;")}).join(`
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
`.concat(Qi(e,r,"out"),`

`).concat(Ue,`
`).concat(rt,`
`).concat(Rt,`

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

  `).concat(Xi,`
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
`);return l}function qu(a){var n=a.shapes,t=a.layers,e=a.shapeGlobalIds,r=za(n),i=en(n).map(function(h){return"uniform ".concat(h.type," ").concat(h.name,";")}).join(`
`),o=function(f){return On(f,"nodeUV","1.0")},s;if(n.length===1)s="float nodeSdfNormalized = ".concat(o(n[0]),";");else{var l=n.map(function(h,f){var g=e?e[f]:f;return"    case ".concat(g,": nodeSdfNormalized = ").concat(o(h),"; break;")}).join(`
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
  vec2 nodeUV = vec2(rotatedScreenUV.x, -rotatedScreenUV.y) / effectiveRadius;`,c=`#version 300 es
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
`.concat(Qi(n,t,"in"),`

uniform float u_cameraAngle;
`).concat(i,`

layout(location = 0) out vec4 fragColor;
layout(location = 1) out vec4 fragPicking;

`).concat(r,`
`).concat(Cu,`
`).concat(ku,`
`).concat(Pu,`
`).concat(Fu,`

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
`);return c}function Ku(a){var n=["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"],t=z(en(a)),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.includes(r.name)||n.push(r.name)}}catch(i){t.e(i)}finally{t.f()}return n}function Yu(a){return{vertexShader:Xu(a),fragmentShader:qu(a),uniforms:Ku(a.shapes)}}function Zu(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=e.shapes,u=e.layers,d=e.getAttributeTexture,c=e.shapeGlobalIds;if(l.length===0)throw new Error("createBackdropProgram: at least one shape must be provided in 'shapes'");var h=(r=s.margin)!==null&&r!==void 0?r:5,f=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},g={shapes:l,layers:u,shapeGlobalIds:c},m=Yu(g),v=(function(p){function b(){var y;J(this,b);for(var T=arguments.length,x=new Array(T),_=0;_<T;_++)x[_]=arguments[_];return y=le(this,b,[].concat(x)),S(y,"totalBackdropCount",0),S(y,"bufferCapacity",0),y}return ue(b,p),ee(b,[{key:"getDefinition",value:function(){var T=WebGL2RenderingContext,x=T.FLOAT,_=T.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:m.vertexShader,FRAGMENT_SHADER_SOURCE:m.fragmentShader,METHOD:_,UNIFORMS:m.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:x},{name:"a_labelWidth",size:1,type:x},{name:"a_labelHeight",size:1,type:x},{name:"a_textHeight",size:1,type:x},{name:"a_positionMode",size:1,type:x},{name:"a_labelAngle",size:1,type:x},{name:"a_backdropColor",size:4,type:x},{name:"a_backdropShadowColor",size:4,type:x},{name:"a_backdropShadowBlur",size:1,type:x},{name:"a_backdropPadding",size:1,type:x},{name:"a_backdropBorderColor",size:4,type:x},{name:"a_backdropExtra",size:4,type:x},{name:"a_labelBoxOffset",size:2,type:x}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:x}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processBackdrop",value:function(T,x){var _=this.floats,A=this.STRIDE,E=T*A;_[E++]=x.nodeIndex,_[E++]=x.labelWidth,_[E++]=x.labelHeight,_[E++]=x.textHeight,_[E++]=wt[x.position],_[E++]=x.labelAngle,_[E++]=x.backdropColor[0],_[E++]=x.backdropColor[1],_[E++]=x.backdropColor[2],_[E++]=x.backdropColor[3],_[E++]=x.backdropShadowColor[0],_[E++]=x.backdropShadowColor[1],_[E++]=x.backdropShadowColor[2],_[E++]=x.backdropShadowColor[3],_[E++]=x.backdropShadowBlur,_[E++]=x.backdropPadding,_[E++]=x.backdropBorderColor[0],_[E++]=x.backdropBorderColor[1],_[E++]=x.backdropBorderColor[2],_[E++]=x.backdropBorderColor[3],_[E++]=x.backdropBorderWidth,_[E++]=x.backdropCornerRadius,_[E++]=x.backdropLabelPadding,_[E++]=x.backdropArea,_[E++]=x.labelBoxOffset[0],_[E++]=x.labelBoxOffset[1]}},{key:"setUniforms",value:function(T,x){var _=x.gl,A=x.uniformLocations;_.uniformMatrix3fv(A.u_matrix,!1,T.matrix),_.uniform1f(A.u_sizeRatio,T.sizeRatio),_.uniform1f(A.u_correctionRatio,T.correctionRatio),_.uniform1f(A.u_cameraAngle,T.cameraAngle),_.uniform2f(A.u_resolution,T.width*T.pixelRatio,T.height*T.pixelRatio),_.uniform1f(A.u_pixelRatio,T.pixelRatio),_.uniform1f(A.u_labelMargin,b.labelMargin),_.uniform1f(A.u_zoomLabelSizeRatio,1/f(T.zoomRatio)),_.uniform1f(A.u_labelPixelSnapping,T.labelPixelSnapping),_.uniform1i(A.u_nodeDataTexture,T.nodeDataTextureUnit),_.uniform1i(A.u_nodeDataTextureWidth,T.nodeDataTextureWidth),_.uniform1i(A.u_nodeFrameTexture,T.nodeFrameTextureUnit),_.uniform1i(A.u_nodeFrameTextureWidth,T.nodeFrameTextureWidth);var E=d();A.u_layerAttributeTexture&&E&&(E.bind(qe),_.uniform1i(A.u_layerAttributeTexture,qe),_.uniform1i(A.u_layerAttributeTextureWidth,E.getTextureWidth()),_.uniform1i(A.u_layerAttributeTexelsPerNode,E.getTexelsPerItem()));var L=z(en(l)),w;try{for(L.s();!(w=L.n()).done;){var F=w.value;this.setTypedUniform(F,x)}}catch(D){L.e(D)}finally{L.f()}}},{key:"hasNothingToRender",value:function(){return this.totalBackdropCount===0}},{key:"drawWebGL",value:function(T,x){var _=x.gl;this.totalBackdropCount!==0&&(this.isInstanced?_.drawArraysInstanced(T,0,this.VERTICES,this.totalBackdropCount):_.drawArrays(T,0,this.totalBackdropCount*this.VERTICES))}},{key:"reallocate",value:function(T){this.totalBackdropCount=T,T>this.bufferCapacity&&(this.bufferCapacity=Math.max(T,Math.ceil(this.bufferCapacity*1.5)||10),ge(b,"reallocate",this,3)([this.bufferCapacity]))}}])})(it);return S(v,"labelMargin",h),new v(a,n,t)}var Ji=(function(a){function n(){var t;J(this,n);for(var e=arguments.length,r=new Array(e),i=0;i<e;i++)r[i]=arguments[i];return t=le(this,n,[].concat(r)),S(t,"totalCharacterCount",0),S(t,"bufferCapacity",0),t}return ue(n,a),ee(n,[{key:"processLabel",value:function(e,r,i){if(i.hidden||!i.text)return 0;for(var o=i.text,s=o.length,l=0;l<s;l++){var u=o[l];this.processCharacter(r+l,i,u,l)}return s}},{key:"hasNothingToRender",value:function(){return this.totalCharacterCount===0}},{key:"drawWebGL",value:function(e,r){var i=r.gl;this.totalCharacterCount!==0&&(this.isInstanced?i.drawArraysInstanced(e,0,this.VERTICES,this.totalCharacterCount):i.drawArrays(e,0,this.totalCharacterCount*this.VERTICES))}},{key:"reallocate",value:function(e){this.totalCharacterCount=e,e>this.bufferCapacity&&(this.bufferCapacity=Math.max(e,Math.ceil(this.bufferCapacity*1.5)||1e3),ge(n,"reallocate",this,3)([this.bufferCapacity]))}}])})(it);function Qu(){var a=`#version 300 es

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

`.concat(Ue,`
`).concat(rt,`
`).concat(Rt,`
`).concat(Ki,`

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

  `).concat(Xi,`

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
`);return a}var Ju=`#version 300 es
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
`;function ed(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Dt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=Qu(),c=(function(h){function f(){var g;J(this,f);for(var m=arguments.length,v=new Array(m),p=0;p<m;p++)v[p]=arguments[p];return g=le(this,f,[].concat(v)),S(g,"totalCount",0),S(g,"bufferCapacity",0),g}return ue(f,h),ee(f,[{key:"getDefinition",value:function(){var m=WebGL2RenderingContext,v=m.FLOAT,p=m.UNSIGNED_BYTE,b=m.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:Ju,METHOD:b,UNIFORMS:["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_pixelRatio","u_labelMargin","u_zoomLabelSizeRatio","u_labelPixelSnapping","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth"],ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:v},{name:"a_id",size:4,type:p,normalized:!0},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_labelWidth",size:1,type:v},{name:"a_labelHeight",size:1,type:v},{name:"a_textHeight",size:1,type:v},{name:"a_positionMode",size:1,type:v},{name:"a_labelAngle",size:1,type:v},{name:"a_padding",size:1,type:v}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:v}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"processLabelBackground",value:function(m,v){var p=this.floats,b=this.ints,y=m*this.STRIDE;p[y++]=v.nodeIndex,b[y++]=v.id,p[y++]=v.color,p[y++]=v.labelWidth,p[y++]=v.labelHeight,p[y++]=v.textHeight,p[y++]=v.positionMode,p[y++]=v.labelAngle,p[y++]=v.padding}},{key:"setUniforms",value:function(m,v){var p=v.gl,b=v.uniformLocations;p.uniformMatrix3fv(b.u_matrix,!1,m.matrix),p.uniform1f(b.u_sizeRatio,m.sizeRatio),p.uniform1f(b.u_correctionRatio,m.correctionRatio),p.uniform1f(b.u_cameraAngle,m.cameraAngle),p.uniform2f(b.u_resolution,m.width*m.pixelRatio,m.height*m.pixelRatio),p.uniform1f(b.u_pixelRatio,m.pixelRatio),p.uniform1f(b.u_labelMargin,f.labelMargin),p.uniform1f(b.u_zoomLabelSizeRatio,1/u(m.zoomRatio)),p.uniform1f(b.u_labelPixelSnapping,m.labelPixelSnapping),p.uniform1f(b.u_pickingPadding,m.labelPickingPadding),p.uniform1i(b.u_nodeDataTexture,m.nodeDataTextureUnit),p.uniform1i(b.u_nodeDataTextureWidth,m.nodeDataTextureWidth),p.uniform1i(b.u_nodeFrameTexture,m.nodeFrameTextureUnit),p.uniform1i(b.u_nodeFrameTextureWidth,m.nodeFrameTextureWidth)}},{key:"hasNothingToRender",value:function(){return this.totalCount===0}},{key:"drawWebGL",value:function(m,v){var p=v.gl;this.totalCount!==0&&p.drawArraysInstanced(p.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(m){this.totalCount=m,m>this.bufferCapacity&&(this.bufferCapacity=Math.max(m,Math.ceil(this.bufferCapacity*1.5)||10),ge(f,"reallocate",this,3)([this.bufferCapacity]))}}])})(it);return S(c,"labelMargin",l),new c(a,n,t)}var Fe={fontSize:64,buffer:8,radius:24,cutoff:.25,maxTextureSize:2048,debounceTimeout:100},Kt=2,Zt=1e20;function td(a,n,t,e,r){var i=a+r*4,o=document.createElement("canvas");o.width=i,o.height=i;var s=o.getContext("2d",{willReadFrequently:!0});return s.font="".concat(e," ").concat(t," ").concat(a,"px ").concat(n),s.textBaseline="alphabetic",s.textAlign="left",s.fillStyle="black",{ctx:s,canvasSize:i,gridOuter:new Float64Array(i*i),gridInner:new Float64Array(i*i),f:new Float64Array(i),z:new Float64Array(i+1),v:new Uint16Array(i)}}function nd(a,n,t,e,r){var i=a.ctx,o=a.canvasSize,s=i.measureText(n),l=s.width,u=s.actualBoundingBoxAscent,d=s.actualBoundingBoxDescent,c=s.actualBoundingBoxLeft,h=s.actualBoundingBoxRight,f=Math.ceil(u),g=Math.ceil(c),m=Math.max(0,Math.min(o-t,Math.ceil(c)+Math.ceil(h))),v=Math.min(o-t,f+Math.ceil(d)),p=m+2*t,b=v+2*t,y=Math.max(p*b,0),T=new Uint8ClampedArray(y),x={data:T,width:p,height:b,glyphWidth:m,glyphHeight:v,glyphTop:f,glyphLeft:g,glyphAdvance:l};if(m===0||v===0)return x;var _=a.gridInner,A=a.gridOuter;i.clearRect(t,t,m,v),i.fillText(n,t+g,t+f);var E=i.getImageData(t,t,m,v);A.fill(Zt,0,y),_.fill(0,0,y);for(var L=0;L<v;L++)for(var w=0;w<m;w++){var F=E.data[4*(L*m+w)+3]/255;if(F!==0){var D=(L+t)*p+w+t;if(F===1)A[D]=0,_[D]=Zt;else{var I=.5-F;A[D]=I>0?I*I:0,_[D]=I<0?I*I:0}}}Gi(A,0,0,p,b,p,a.f,a.v,a.z),Gi(_,t,t,m,v,p,a.f,a.v,a.z);for(var R=0;R<y;R++){var P=Math.sqrt(A[R])-Math.sqrt(_[R]);T[R]=Math.round(255-255*(P/e+r))}return x}function Gi(a,n,t,e,r,i,o,s,l){for(var u=n;u<n+e;u++)Mi(a,t*i+u,i,r,o,s,l);for(var d=t;d<t+r;d++)Mi(a,d*i+n,1,e,o,s,l)}function Mi(a,n,t,e,r,i,o){i[0]=0,o[0]=-Zt,o[1]=Zt,r[0]=a[n];for(var s=1,l=0,u=0;s<e;s++){r[s]=a[n+s*t];var d=s*s;do{var c=i[l];u=(r[s]-r[c]+d-c*c)/(s-c)/2}while(u<=o[l]&&--l>-1);l++,i[l]=s,o[l]=u,o[l+1]=Zt}for(var h=0,f=0;h<e;h++){for(;o[f+1]<h;)f++;var g=i[f],m=h-g;a[n+h*t]=r[g]+m*m}}var at=(function(a){function n(){var t,e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{};return J(this,n),t=le(this,n),S(t,"fonts",new Map),S(t,"textures",[]),S(t,"cursor",{x:0,y:0,rowHeight:0,atlasIndex:0}),S(t,"pendingGlyphs",[]),S(t,"debounceTimer",null),t.options=B(B({},Fe),e),t.canvas=document.createElement("canvas"),t.canvas.width=t.options.maxTextureSize,t.canvas.height=t.options.maxTextureSize,t.ctx=t.canvas.getContext("2d",{willReadFrequently:!0}),t.measureCanvas=document.createElement("canvas"),t.measureCtx=t.measureCanvas.getContext("2d"),t.textures.push(t.ctx.getImageData(0,0,1,1)),t}return ue(n,a),ee(n,[{key:"getFontKey",value:function(e){return"".concat(e.family,"-").concat(e.weight,"-").concat(e.style)}},{key:"registerFont",value:function(e){var r=this.getFontKey(e);if(this.fonts.has(r))return r;var i=td(this.options.fontSize,e.family,e.weight,e.style,this.options.buffer);return this.fonts.set(r,{descriptor:e,generator:i,glyphs:new Map}),r}},{key:"ensureGlyphs",value:function(e,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered. Call registerFont() first.'));var o=!1,s=z(e),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=u.codePointAt(0);d!==void 0&&(i.glyphs.has(d)||(this.pendingGlyphs.push({fontKey:r,charCode:d}),o=!0))}}catch(c){s.e(c)}finally{s.f()}o&&this.scheduleTextureGeneration()}},{key:"measureText",value:function(e,r){var i=this.fonts.get(r);if(!i)throw new Error('Font "'.concat(r,'" is not registered.'));var o=i.descriptor,s=o.family,l=o.weight,u=o.style;return this.measureCtx.font="".concat(u," ").concat(l," ").concat(this.options.fontSize,"px ").concat(s),this.measureCtx.measureText(e).width}},{key:"getGlyph",value:function(e,r){var i=this.fonts.get(r);if(i)return i.glyphs.get(e)}},{key:"getTextures",value:function(){return this.textures}},{key:"getFontCount",value:function(){return this.fonts.size}},{key:"getGlyphCount",value:function(){var e=0,r=z(this.fonts.values()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;e+=o.glyphs.size}}catch(s){r.e(s)}finally{r.f()}return e}},{key:"hasPendingGlyphs",value:function(){return this.pendingGlyphs.length>0}},{key:"flush",value:function(){this.debounceTimer&&(clearTimeout(this.debounceTimer),this.debounceTimer=null),this.generateTextures()}},{key:"destroy",value:function(){this.debounceTimer&&clearTimeout(this.debounceTimer),this.fonts.clear(),this.textures=[],this.pendingGlyphs=[],this.removeAllListeners()}},{key:"scheduleTextureGeneration",value:function(){var e=this;this.debounceTimer===null&&(this.options.debounceTimeout===null?this.generateTextures():this.debounceTimer=setTimeout(function(){e.debounceTimer=null,e.generateTextures()},this.options.debounceTimeout))}},{key:"generateTextures",value:function(){if(this.pendingGlyphs.length!==0){var e=this.options,r=e.maxTextureSize,i=e.buffer,o=e.radius,s=e.cutoff,l=z(this.pendingGlyphs),u;try{for(l.s();!(u=l.n()).done;){var d=u.value,c=d.fontKey,h=d.charCode,f=this.fonts.get(c);if(!(!f||f.glyphs.has(h))){var g=String.fromCodePoint(h),m=nd(f.generator,g,i,o,s),v=m.width,p=m.height;this.cursor.x+v+Kt>r&&(this.cursor.x=0,this.cursor.y+=this.cursor.rowHeight+Kt,this.cursor.rowHeight=0),this.cursor.y+p+Kt>r&&(this.finalizeCurrentTexture(),this.cursor={x:0,y:0,rowHeight:0,atlasIndex:this.cursor.atlasIndex+1},this.ctx.clearRect(0,0,r,r));for(var b=m.data,y=new Uint8ClampedArray(v*p*4),T=0;T<b.length;T++){var x=T*4;y[x]=255,y[x+1]=255,y[x+2]=255,y[x+3]=b[T]}var _=new ImageData(y,v,p);this.ctx.putImageData(_,this.cursor.x,this.cursor.y);var A={charCode:h,width:m.glyphWidth,height:m.glyphHeight,bearingX:-m.glyphLeft-i,bearingY:m.glyphTop+i,advance:m.glyphAdvance,atlasX:this.cursor.x,atlasY:this.cursor.y,atlasWidth:v,atlasHeight:p,atlasIndex:this.cursor.atlasIndex};f.glyphs.set(h,A),this.cursor.x+=v+Kt,this.cursor.rowHeight=Math.max(this.cursor.rowHeight,p)}}}catch(E){l.e(E)}finally{l.f()}this.finalizeCurrentTexture(),this.pendingGlyphs=[],this.emit(n.ATLAS_UPDATED_EVENT,{textures:this.textures,glyphCount:this.getGlyphCount()})}}},{key:"finalizeCurrentTexture",value:function(){var e=this.options.maxTextureSize,r=this.cursor.y>0||this.cursor.rowHeight>0,i=Math.min(e,Math.max(this.cursor.x,r?e:1)),o=Math.min(e,this.cursor.y+this.cursor.rowHeight+Kt),s=this.ctx.getImageData(0,0,i,o);this.cursor.atlasIndex>=this.textures.length?this.textures.push(s):this.textures[this.cursor.atlasIndex]=s}}])})(Oi.EventEmitter);S(at,"ATLAS_UPDATED_EVENT","atlasUpdated");var ad=Fe.fontSize;function rd(){var a=`  // -------------------------------------------------------------------------
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
const float ATLAS_FONT_SIZE = `.concat(he(ad),`;

// ============================================================================
// Helper Functions
// ============================================================================

`).concat(Ue,`
`).concat(rt,`
`).concat(Rt,`
`).concat(qi,`

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
`);return n}function id(){var a=`#version 300 es
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
`;return a}function od(){return["u_matrix","u_sizeRatio","u_correctionRatio","u_cameraAngle","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_pixelRatio","u_nodeDataTexture","u_nodeDataTextureWidth","u_nodeFrameTexture","u_nodeFrameTextureWidth","u_zoomLabelSizeRatio","u_labelPixelSnapping"]}function sd(){return{vertexShader:rd(),fragmentShader:id(),uniforms:od()}}function ld(a,n,t,e){var r,i,o=e.label,s=o===void 0?{}:o,l=(r=s.margin)!==null&&r!==void 0?r:Dt,u=(i=s.zoomToLabelSizeRatioFunction)!==null&&i!==void 0?i:function(){return 1},d=sd(),c=(function(h){function f(g,m,v){var p,b,y,T;if(J(this,f),T=le(this,f,[g,m,v]),S(T,"atlasTexture",null),S(T,"atlasNeedsUpdate",!1),S(T,"labelGlyphCache",new Map),T.atlasFontSize=Fe.fontSize*qt(),T.atlasManager=new at({fontSize:T.atlasFontSize}),T.gamma=.025,T.sdfBuffer=Fe.cutoff,T.atlasTexture=g.createTexture(),!T.atlasTexture)throw new Error("NodeLabelProgram: failed to create atlas texture");g.bindTexture(g.TEXTURE_2D,T.atlasTexture),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_S,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_WRAP_T,g.CLAMP_TO_EDGE),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MIN_FILTER,g.LINEAR),g.texParameteri(g.TEXTURE_2D,g.TEXTURE_MAG_FILTER,g.LINEAR),g.bindTexture(g.TEXTURE_2D,null),T.atlasManager.on(at.ATLAS_UPDATED_EVENT,function(){T.atlasNeedsUpdate=!0});var x={family:((p=s.font)===null||p===void 0?void 0:p.family)||"sans-serif",weight:((b=s.font)===null||b===void 0?void 0:b.weight)||"normal",style:((y=s.font)===null||y===void 0?void 0:y.style)||"normal"};return T.defaultFontKey=T.atlasManager.registerFont(x),T}return ue(f,h),ee(f,[{key:"getDefinition",value:function(){var m=WebGL2RenderingContext,v=m.FLOAT,p=m.UNSIGNED_BYTE,b=m.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:d.vertexShader,FRAGMENT_SHADER_SOURCE:d.fragmentShader,METHOD:b,UNIFORMS:d.uniforms,ATTRIBUTES:[{name:"a_nodeIndex",size:1,type:v},{name:"a_charOffset",size:2,type:v},{name:"a_charSize",size:2,type:v},{name:"a_texCoords",size:4,type:v},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_margin",size:1,type:v},{name:"a_positionMode",size:1,type:v},{name:"a_labelWidth",size:1,type:v},{name:"a_labelHeight",size:1,type:v},{name:"a_verticalCenter",size:1,type:v},{name:"a_textHeight",size:1,type:v},{name:"a_labelAngle",size:1,type:v}],CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:v}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(m,v){if(v.hidden||!v.text){this.labelGlyphCache.delete(m);return}var p=v.text,b=v.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(p,b);var y=[],T=[],x=0,_=0,A=0,E=z(p),L;try{for(E.s();!(L=E.n()).done;){var w=L.value,F=w.codePointAt(0);if(F===void 0){y.push(void 0),T.push(x);continue}var D=this.atlasManager.getGlyph(F,b);y.push(D),T.push(x),D&&(x+=D.advance,_=Math.max(_,D.bearingY),A=Math.max(A,D.atlasHeight-D.bearingY))}}catch(I){E.e(I)}finally{E.f()}this.labelGlyphCache.set(m,{glyphs:y,xOffsets:T,totalWidth:x,totalHeight:_+A,verticalCenterOffset:(_-A)/2})}},{key:"processCharacter",value:function(m,v,p,b){var y=this.floats,T=this.STRIDE,x=m*T,_=this.labelGlyphCache.get(v.parentKey);if(!_||!_.glyphs[b]){for(var A=0;A<T;A++)y[x+A]=0;return}var E=_.glyphs[b],L=_.xOffsets[b],w=v.size/this.atlasFontSize,F=De(v.color),D=x;y[D++]=v.nodeIndex,y[D++]=(L+E.bearingX)*w,y[D++]=-E.bearingY*w,y[D++]=E.atlasWidth*w,y[D++]=E.atlasHeight*w,y[D++]=E.atlasX,y[D++]=E.atlasY,y[D++]=E.atlasWidth,y[D++]=E.atlasHeight,y[D++]=F,y[D++]=v.margin,y[D++]=wt[v.position],y[D++]=_.totalWidth*w,y[D++]=v.size,y[D++]=_.verticalCenterOffset*w,y[D++]=_.totalHeight*w,y[D++]=v.labelAngle}},{key:"processLabel",value:function(m,v,p){return this.prepareLabelGlyphs(m,p),ge(f,"processLabel",this,3)([m,v,p])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var m=this.normalProgram.gl,v=this.atlasManager.getTextures();if(v.length!==0){var p=v[0];m.bindTexture(m.TEXTURE_2D,this.atlasTexture),m.texImage2D(m.TEXTURE_2D,0,m.RGBA,p.width,p.height,0,m.RGBA,m.UNSIGNED_BYTE,p.data),m.bindTexture(m.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(m,v){var p=v.gl,b=v.uniformLocations;p.uniformMatrix3fv(b.u_matrix,!1,m.matrix),p.uniform1f(b.u_sizeRatio,m.sizeRatio),p.uniform1f(b.u_correctionRatio,m.correctionRatio),p.uniform1f(b.u_cameraAngle,m.cameraAngle),p.uniform2f(b.u_resolution,m.width*m.pixelRatio,m.height*m.pixelRatio);var y=this.atlasManager.getTextures();y.length>0?p.uniform2f(b.u_atlasSize,y[0].width,y[0].height):p.uniform2f(b.u_atlasSize,1,1),p.activeTexture(p.TEXTURE0),p.bindTexture(p.TEXTURE_2D,this.atlasTexture),p.uniform1i(b.u_atlas,0),b.u_nodeDataTexture!==void 0&&p.uniform1i(b.u_nodeDataTexture,m.nodeDataTextureUnit),b.u_nodeDataTextureWidth!==void 0&&p.uniform1i(b.u_nodeDataTextureWidth,m.nodeDataTextureWidth),p.uniform1i(b.u_nodeFrameTexture,m.nodeFrameTextureUnit),p.uniform1i(b.u_nodeFrameTextureWidth,m.nodeFrameTextureWidth),p.uniform1f(b.u_gamma,this.gamma),p.uniform1f(b.u_sdfBuffer,this.sdfBuffer),p.uniform1f(b.u_pixelRatio,m.pixelRatio),p.uniform1f(b.u_zoomLabelSizeRatio,1/f.zoomToLabelSizeRatioFunction(m.zoomRatio)),p.uniform1f(b.u_labelPixelSnapping,m.labelPixelSnapping)}},{key:"renderProgram",value:function(m,v){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),ge(f,"renderProgram",this,3)([m,v])}},{key:"registerFont",value:function(m){var v=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",p=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:m,weight:v,style:p})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"measureLabel",value:function(m,v,p){var b=p||this.defaultFontKey;this.atlasManager.ensureGlyphs(m,b),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var y=0,T=0,x=0,_=z(m),A;try{for(_.s();!(A=_.n()).done;){var E=A.value,L=E.codePointAt(0);if(L!==void 0){var w=this.atlasManager.getGlyph(L,b);w&&(y+=w.advance,T=Math.max(T,w.bearingY),x=Math.max(x,w.atlasHeight-w.bearingY))}}}catch(D){_.e(D)}finally{_.f()}var F=v/this.atlasFontSize;return{width:y*F,height:v,textHeight:(T+x)*F}}},{key:"ensureGlyphsReady",value:function(m,v){var p=v||this.defaultFontKey,b=z(m),y;try{for(b.s();!(y=b.n()).done;){var T=y.value;this.atlasManager.ensureGlyphs(T,p)}}catch(x){b.e(x)}finally{b.f()}this.atlasManager.flush()}},{key:"kill",value:function(){var m=this.normalProgram.gl;this.atlasTexture&&(m.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),ge(f,"kill",this,3)([])}}])})(Ji);return S(c,"labelMargin",l),S(c,"zoomToLabelSizeRatioFunction",u),new c(a,n,t)}var ud=`#version 300 es
precision highp float;

in float v_edgeDist;

// R32F target: only the .r channel is stored.
out vec4 fragColor;

void main() {
  fragColor = vec4(v_edgeDist, 0.0, 0.0, 0.0);
}
`;function dd(a,n,t){var e=za(a),r=en(a).map(function(g){return"uniform ".concat(g.type," ").concat(g.name,";")}).join(`
`),i=Nu(a,t),o=i.code,s=i.multiShape,l=Lt(a,n),u=Object.keys(l.specs).map(function(g){return"float v_".concat(g,";")}).join(`
`),d=At(l,{varPrefix:"nodeAttr",baseTexelExpr:"nodeIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture"}),c=d.fetchCode,h=d.varyingAssignments,f=`#version 300 es
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

`).concat(Ue,`
`).concat(rt,`
`).concat(e,`
`).concat(o,`
`).concat(qi,`

void main() {
  int nodeIdx = int(a_nodeIndex);
`).concat(c,`
`).concat(h,`
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
`);return f}var eo=(function(){function a(n,t){J(this,a),S(this,"uniformLocations",{});var e=t.shapes,r=t.layers,i=t.shapeGlobalIds;if(e.length===0)throw new Error("NodeLabelFramePass: at least one shape must be provided");this.gl=n,this.shapeUniforms=en(e),this.hasAttributeData=Object.keys(Lt(e,r).offsets).length>0,this.vertexShader=Na(n,dd(e,r,i)),this.fragmentShader=Ga(n,ud),this.program=Ma(n,[this.vertexShader,this.fragmentShader]);var o=["u_cameraAngle","u_nodeDataTexture","u_nodeDataTextureWidth","u_frameTextureWidth","u_frameTextureHeight","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode"].concat(V(this.shapeUniforms.map(function(p){return p.name}))),s=z(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;this.uniformLocations[u]=n.getUniformLocation(this.program,u)}}catch(p){s.e(p)}finally{s.f()}var d=a.FLOATS_PER_POINT*4;this.vao=n.createVertexArray(),this.buffer=n.createBuffer(),n.bindVertexArray(this.vao),n.bindBuffer(n.ARRAY_BUFFER,this.buffer);var c=z([["a_nodeIndex",0],["a_positionMode",4],["a_labelAngle",8]]),h;try{for(c.s();!(h=c.n()).done;){var f=ae(h.value,2),g=f[0],m=f[1],v=n.getAttribLocation(this.program,g);v>=0&&(n.enableVertexAttribArray(v),n.vertexAttribPointer(v,1,n.FLOAT,!1,d,m))}}catch(p){c.e(p)}finally{c.f()}n.bindVertexArray(null)}return ee(a,[{key:"run",value:function(t,e,r,i,o){if(e!==0){var s=this.gl;s.useProgram(this.program),s.bindVertexArray(this.vao),s.bindBuffer(s.ARRAY_BUFFER,this.buffer),s.bufferData(s.ARRAY_BUFFER,t.subarray(0,e*a.FLOATS_PER_POINT),s.DYNAMIC_DRAW);var l=this.uniformLocations;s.uniform1f(l.u_cameraAngle,i.cameraAngle),s.uniform1i(l.u_nodeDataTexture,i.nodeDataTextureUnit),s.uniform1i(l.u_nodeDataTextureWidth,i.nodeDataTextureWidth),s.uniform1f(l.u_frameTextureWidth,r.getTextureWidth()),s.uniform1f(l.u_frameTextureHeight,r.getTextureHeight()),this.hasAttributeData&&o&&(o.bind(qe),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,qe),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=z(this.shapeUniforms),d;try{for(u.s();!(d=u.n()).done;){var c=d.value;Ua(s,this.uniformLocations[c.name],c)}}catch(h){u.e(h)}finally{u.f()}r.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,e),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var t=this.gl;t.deleteProgram(this.program),t.deleteShader(this.vertexShader),t.deleteShader(this.fragmentShader),t.deleteBuffer(this.buffer),t.deleteVertexArray(this.vao)}}])})();S(eo,"FLOATS_PER_POINT",3);function to(a,n,t,e,r){var i,o=e.label,s=o===void 0?{}:o,l=e.shapes;if(l.length===0)throw new Error("createNodeProgram: at least one shape must be provided in 'shapes'");var u={},d=[],c;l.forEach(function(A,E){var L=Au(A);E===0&&(c=L),u[A.name]=E,d[E]=Jt(L)});var h=V(e.layers),f=Ni({shapes:l,layers:h,shapeGlobalIds:l.length>1?d:void 0,antialias:r}),g=ld(a,null,t,{label:s}),m=Zu(a,null,t,{shapes:l,layers:h,getAttributeTexture:function(){return _.getAttributeTexture()},label:s,shapeGlobalIds:l.length>1?d:void 0}),v=ed(a,n,t,{label:s}),p=e.labelAttachments&&Object.keys(e.labelAttachments).length>0?Ou(a,null,t,{label:s}):null,b=new eo(a,{shapes:l,layers:h,shapeGlobalIds:l.length>1?d:void 0}),y=Xn(l,h),T=Le(y),x=(i=(function(A){function E(L,w,F){var D;J(this,E),D=le(this,E,[L,w,F]),S(D,"layerLifecycles",new Map),S(D,"layersNeedingRegeneration",new Set),S(D,"attrDescriptors",[]),D._pickingBuffer=w;var I=x.layerTextures.get(L);I||(I=new $n(L,T),x.layerTextures.set(L,I)),D.layerAttributeTexture=I;var R=x.textureRefCounts.get(L)||0;x.textureRefCounts.set(L,R+1),D.packedAttributeData=new Float32Array(T.floatsPerItem),h.forEach(function(G,N){if(G.lifecycle){var k={gl:L,renderer:{refresh:function(){return F.refresh()}},getUniformLocation:function($){return L.getUniformLocation(D.normalProgram.program,$)},requestShaderRegeneration:function(){D.layersNeedingRegeneration.add(N)},requestRefresh:function(){F.refresh()}},C=G.lifecycle(k);D.layerLifecycles.set(N,C)}}),D.layerLifecycles.forEach(function(G){var N;(N=G.init)===null||N===void 0||N.call(G)});var P=new Map;return D.layerLifecycles.forEach(function(G,N){G.getAttributeData&&P.set(N,G)}),D.attrDescriptors=Vn(y,T,P),D}return ue(E,A),ee(E,[{key:"getDefinition",value:function(){var w=WebGL2RenderingContext,F=w.FLOAT,D=w.TRIANGLE_STRIP;return{VERTICES:4,VERTEX_SHADER_SOURCE:f.vertexShader,FRAGMENT_SHADER_SOURCE:f.fragmentShader,METHOD:D,UNIFORMS:f.uniforms,ATTRIBUTES:f.attributes,CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:F}],CONSTANT_DATA:[[-1,-1],[1,-1],[-1,1],[1,1]]}}},{key:"maybeRegenerateShaders",value:function(){var w=this;if(this.layersNeedingRegeneration.size!==0){h=h.map(function(N,k){if(w.layersNeedingRegeneration.has(k)){var C=w.layerLifecycles.get(k);if(C!=null&&C.regenerate){var M=C.regenerate();return B(B({},M),{},{lifecycle:N.lifecycle})}}return N}),this.layersNeedingRegeneration.clear(),f=Ni({shapes:l,layers:h,shapeGlobalIds:l.length>1?d:void 0,antialias:this.renderer.getSetting("antialiasNodes")});var F=this.normalProgram.gl,D=this.normalProgram,I=D.program,R=D.buffer,P=D.vertexShader,G=D.fragmentShader;F.deleteProgram(I),F.deleteBuffer(R),F.deleteShader(P),F.deleteShader(G),this.normalProgram=this.getProgramInfo("normal",F,f.vertexShader,f.fragmentShader,this._pickingBuffer)}}},{key:"getAttributeTexture",value:function(){return this.layerAttributeTexture}},{key:"uploadLayerTexture",value:function(){this.layerAttributeTexture.upload()}},{key:"process",value:function(w,F,D,I){var R=F*this.STRIDE;if(D.visibility==="hidden"){for(var P=R+this.STRIDE;R<P;R++)this.floats[R]=0;this.floats[F*this.STRIDE]=Yi;return}this.processVisibleItem(yt(w),R,D,I)}},{key:"processVisibleItem",value:function(w,F,D,I){var R,P=this.floats,G=this.ints;if(P[F++]=I,G[F++]=w,P[F++]=(R=D.opacity)!==null&&R!==void 0?R:1,T.floatsPerItem!==0){var N=this.packedAttributeData;jn(this.attrDescriptors,D,N,D.color,this.layerLifecycles,0),this.layerAttributeTexture.updateAllAttributesAtRow(I,N)}}},{key:"setUniforms",value:function(w,F){var D=this,I=F.gl,R=F.uniformLocations;R.u_matrix&&I.uniformMatrix3fv(R.u_matrix,!1,w.matrix),R.u_sizeRatio&&I.uniform1f(R.u_sizeRatio,w.sizeRatio),R.u_correctionRatio&&I.uniform1f(R.u_correctionRatio,w.correctionRatio),R.u_pickingPadding&&I.uniform1f(R.u_pickingPadding,w.nodePickingPadding),R.u_cameraAngle&&I.uniform1f(R.u_cameraAngle,w.cameraAngle),R.u_nodeDataTexture&&I.uniform1i(R.u_nodeDataTexture,w.nodeDataTextureUnit),R.u_nodeDataTextureWidth&&I.uniform1i(R.u_nodeDataTextureWidth,w.nodeDataTextureWidth),R.u_layerAttributeTexture&&(this.layerAttributeTexture.bind(qe),I.uniform1i(R.u_layerAttributeTexture,qe)),R.u_layerAttributeTextureWidth&&I.uniform1i(R.u_layerAttributeTextureWidth,this.layerAttributeTexture.getTextureWidth()),R.u_layerAttributeTexelsPerNode&&I.uniform1i(R.u_layerAttributeTexelsPerNode,this.layerAttributeTexture.getTexelsPerItem()),l.forEach(function(P){P.uniforms.forEach(function(G){D.setTypedUniform(G,F)})}),h.forEach(function(P){P.uniforms.forEach(function(G){D.setTypedUniform(G,F)})})}},{key:"render",value:function(w,F,D){this.maybeRegenerateShaders(),ge(E,"render",this,3)([w,F,D])}},{key:"renderProgram",value:function(w,F){var D=F.gl,I=F.program;D.useProgram(I),F===this.normalProgram&&this.layerLifecycles.forEach(function(R){var P;(P=R.beforeRender)===null||P===void 0||P.call(R)}),ge(E,"renderProgram",this,3)([w,F])}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(D){var I;(I=D.kill)===null||I===void 0||I.call(D)}),this.layerLifecycles.clear();var w=this.normalProgram.gl,F=(x.textureRefCounts.get(w)||1)-1;F<=0?(this.layerAttributeTexture.kill(),x.layerTextures.delete(w),x.textureRefCounts.delete(w)):x.textureRefCounts.set(w,F),ge(E,"kill",this,3)([])}}])})(it),S(i,"layerTextures",new WeakMap),S(i,"textureRefCounts",new WeakMap),i),_=new x(a,null,t);return{nodeProgram:_,labelProgram:g,backdropProgram:m,labelBackgroundProgram:v,attachmentProgram:p,framePass:b,shapeSlug:c,shapeNameToIndex:l.length>1?u:void 0,shapeGlobalIds:l.length>1?d:void 0}}var Ke=6;function nn(a){var n=a.offsets,t=a.specs,e=a.floatsPerItem,r=Object.keys(n);if(r.length===0||e===0)return{uniformDeclarations:"",uniformNames:[],vertexVaryingDeclarations:"",fragmentVaryingDeclarations:"",fetchCode:"",varyingAssignments:""};var i=`
uniform sampler2D u_edgeAttributeTexture;
uniform int u_edgeAttributeTextureWidth;
uniform int u_edgeAttributeTexelsPerEdge;`,o=["u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"],s=r.map(function(f){var g=t[f].size===1?"float":"vec".concat(t[f].size);return"".concat(g," v_").concat(f,";")}),l=s.map(function(f){return"out ".concat(f)}).join(`
`),u=s.map(function(f){return"in ".concat(f)}).join(`
`),d=At(a,{varPrefix:"attr",baseTexelExpr:"edgeIdx * u_edgeAttributeTexelsPerEdge",textureWidthUniform:"u_edgeAttributeTextureWidth",textureSamplerUniform:"u_edgeAttributeTexture"}),c=d.fetchCode,h=d.varyingAssignments;return{uniformDeclarations:i,uniformNames:o,vertexVaryingDeclarations:l,fragmentVaryingDeclarations:u,fetchCode:c,varyingAssignments:h}}function cd(a){return`
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
`)}function hd(a){return`
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
`)}function no(a){return`
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
`)}function fd(a){return`
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
`)}function gd(a){return`
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
`)}function pd(a){return`
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
`)}function vd(a){return`
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
`)}function zn(a,n){var t=new RegExp("\\b(float|vec[234]|void|int|bool)\\s+".concat(n,"\\s*\\("));return t.test(a)}function ao(a,n){var t=[];return zn(n,"path_".concat(a,"_length"))||t.push(fd(a)),zn(n,"path_".concat(a,"_closest_t"))||t.push(gd(a)),zn(n,"path_".concat(a,"_distance"))||t.push(pd(a)),zn(n,"path_".concat(a,"_t_at_distance"))||t.push(vd(a)),t.length>0?`
// ============================================================================
// Auto-generated fallback functions (path only provided position)
// ============================================================================
`.concat(t.join(`
`)):""}function ro(a){var n,t,e,r=a.paths,i=a.layers,o=(n=a.extremities)!==null&&n!==void 0?n:[];if(r.length===0)throw new Error("At least one path is required in 'paths'");if(i.length===0)throw new Error("At least one layer is required in 'layers'");var s=(t=a.defaultHead)!==null&&t!==void 0?t:"none",l=(e=a.defaultTail)!==null&&e!==void 0?e:"none";return{paths:r,extremities:o,layers:i,path:r[0],layer:i[0],defaultHead:s,defaultTail:l}}var io=WebGL2RenderingContext,Qt=io.FLOAT,zi=io.UNSIGNED_BYTE;function md(a,n,t){var e=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"]),r=new Set(e);return a.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),n.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),t.forEach(function(i){return i.uniforms.forEach(function(o){return r.add(o.name)})}),Array.from(r)}function Oa(a){var n=new Set,t=[],e=z(a),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;n.has(i.name)||(n.add(i.name),t.push("// Path: ".concat(i.name)),t.push(i.glsl),t.push(no(i.name)),t.push(ao(i.name,i.glsl)))}}catch(o){e.e(o)}finally{e.f()}return t.join(`

`)}function bd(a){var n=[],t=z(a),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.push("// Extremity: ".concat(r.name)),n.push(r.glsl)}}catch(i){t.e(i)}finally{t.f()}return n.join(`

`)}var yd=[{queryName:"queryPathPosition",pathFunc:"position",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathTangent",pathFunc:"tangent",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathNormal",pathFunc:"normal",returnType:"vec2",params:"float t, vec2 source, vec2 target",args:"t, source, target"},{queryName:"queryPathLength",pathFunc:"length",returnType:"float",params:"vec2 source, vec2 target",args:"source, target"},{queryName:"queryPathClosestT",pathFunc:"closest_t",returnType:"float",params:"vec2 p, vec2 source, vec2 target",args:"p, source, target"}];function xd(a,n){var t=n.queryName,e=n.pathFunc,r=n.returnType,i=n.params,o=n.args;if(a.length===1)return"".concat(r," ").concat(t,"(int pathId, ").concat(i,`) {
  return path_`).concat(a[0].name,"_").concat(e,"(").concat(o,`);
}`);var s=a.map(function(l,u){return"    case ".concat(u,": return path_").concat(l.name,"_").concat(e,"(").concat(o,");")}).join(`
`);return"".concat(r," ").concat(t,"(int pathId, ").concat(i,`) {
  switch (pathId) {
`).concat(s,`
    default: return path_`).concat(a[0].name,"_").concat(e,"(").concat(o,`);
  }
}`)}function Ha(a){return yd.map(function(n){return xd(a,n)}).join(`

`)}function _d(a){if(a.length===1)return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  return extremity_`.concat(a[0].name,`(uv, lengthRatio, widthRatio);
}`);var n=a.map(function(t,e){return"    case ".concat(e,": return extremity_").concat(t.name,"(uv, lengthRatio, widthRatio);")}).join(`
`);return`float queryExtremitySDF(int extremityId, vec2 uv, float lengthRatio, float widthRatio) {
  switch (extremityId) {
`.concat(n,`
    default: return extremity_`).concat(a[0].name,`(uv, lengthRatio, widthRatio);
  }
}`)}function Td(a){var n=[],t=z(a),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;n.push(cd(r.name)),n.push(hd(r.name))}}catch(s){t.e(s)}finally{t.f()}if(a.length===1)n.push(`
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

`)}function Sd(a,n,t,e,r){var i=Le([].concat(V(a),V(t))),o=nn(i),s=Lt(e,r),l=Object.keys(s.offsets),u=function(A){var E=s.specs[A].size;return E===1?"float":"vec".concat(E)},d=l.map(function(_){return"".concat(u(_)," v_source_").concat(_,`;
`).concat(u(_)," v_target_").concat(_,";")}).join(`
`),c=At(s,{varPrefix:"srcNodeAttr",baseTexelExpr:"srcIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_source_"}),h=At(s,{varPrefix:"tgtNodeAttr",baseTexelExpr:"tgtIdx * u_layerAttributeTexelsPerNode",textureWidthUniform:"u_layerAttributeTextureWidth",textureSamplerUniform:"u_layerAttributeTexture",outputPrefix:"v_target_"}),f=l.map(function(_){return"".concat(u(_)," g_").concat(_,";")}).join(`
`),g=l.map(function(_){return"  g_".concat(_," = v_source_").concat(_,";")}).join(`
`),m=l.map(function(_){return"  g_".concat(_," = v_target_").concat(_,";")}).join(`
`),v=new Set,p=[],b=function(A){v.has(A.name)||(v.add(A.name),p.push("uniform ".concat(A.type," ").concat(A.name,";")))};a.forEach(function(_){return _.uniforms.forEach(b)}),n.forEach(function(_){return _.uniforms.forEach(b)});var y=n.map(function(_){return he(_.widthFactor)}).join(", "),T=Math.max.apply(Math,V(a.map(function(_){return _.minBodyLengthRatio||0}))),x=`#version 300 es

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
`).concat(f,`

// Scattered output written to the edge-frame texel: [tStart, tEnd, straightenFactor, pathLength]
out vec4 v_clamp;

// Extremity width factor array (needed for extremityScale computation)
const float EXTREMITY_WIDTH_FACTORS[`).concat(n.length,"] = float[](").concat(y,`);

// Node-data fetch helpers (geometry texel + rotation-flags texel)
`).concat(Ue,`
`).concat(rt,`

// Shape SDFs for node boundary clamping
`).concat(wu(),`
`).concat(Du(new Set(l)),`

// Path functions
`).concat(Oa(a),`
`).concat(Ha(a),`
`).concat(Td(a),`

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
`).concat(c.fetchCode,`
`).concat(c.varyingAssignments,`
`).concat(h.fetchCode,`
`).concat(h.varyingAssignments,`

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
  float minBodyLengthRatio = `).concat(he(T),`;

  // Thickness in WebGL units (needed to compute clamping margin and extremity lengths)
  float pixelsThickness = max(a_thickness, u_minEdgeThickness * u_sizeRatio);
  float webGLThickness = pixelsThickness * u_correctionRatio / u_sizeRatio;

  // SDF clamping: find where the edge body meets the node boundaries. Always
  // searched (ungated) so labels get a true boundary clamp; the body re-applies
  // its extremity gating in-shader.
`).concat(g,`
  float tStart = queryFindSourceClampT(pathId, a_source, a_sourceSize, int(a_sourceShapeId), a_sourceRotateAlign, a_target, 0.0);
`).concat(m,`
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
`).concat(m,`
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
`);return x}var Bn=0,Bi=1,Wn=2;function Ed(a,n,t){var e=[];t&&(e.push([Bn,0,-1],[Bn,0,1]),e.push([Bn,1,-1],[Bn,1,1]));for(var r=0;r<=a;r++){var i=r/a;e.push([Bi,i,-1],[Bi,i,1])}return n&&(e.push([Wn,0,-1],[Wn,0,1]),e.push([Wn,1,-1],[Wn,1,1])),{data:e,attributes:[{name:"a_zone",size:1,type:Qt},{name:"a_zoneT",size:1,type:Qt},{name:"a_side",size:1,type:Qt}],verticesPerEdge:e.length}}function Ad(a,n,t,e){var r=Le([].concat(V(a),V(t))),i=nn(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding","u_nodeDataTexture"]),s=new Set,l=[],u=function(p){!o.has(p.name)&&!s.has(p.name)&&(s.add(p.name),l.push("uniform ".concat(p.type," ").concat(p.name,";")))};a.forEach(function(v){return v.uniforms.forEach(u)}),n.forEach(function(v){return v.uniforms.forEach(u)}),t.forEach(function(v){return v.uniforms.forEach(u)});var d=e.map(function(v){var p=v.size===1?"float":"vec".concat(v.size);return"in ".concat(p," ").concat(v.name,";")}).join(`
`),c=n.map(function(v){return he(v.widthFactor)}).join(", "),h=Math.max.apply(Math,V(a.map(function(v){return v.minBodyLengthRatio||0}))),f=t.some(function(v){return v.needsNodeColors}),g=a.some(function(v){return v.needsNodeSize}),m=`#version 300 es

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
const float EXTREMITY_WIDTH_FACTORS[`).concat(n.length,"] = float[](").concat(c,`);

// All path functions
`).concat(Oa(a),`

// Path selector functions
`).concat(Ha(a),`

// Node-data fetch helper (geometry texel of the two-texel node stride)
`).concat(Ue,`
`).concat(f?Iu:"",`
// Per-edge clamp from the frame-pass (tStart, tEnd, straightenFactor, pathLength)
`).concat(Rt,`

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
  float minBodyLengthRatio = `).concat(he(h),`;

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
`);return m}function wd(a,n,t,e){var r=Le([].concat(V(a),V(t))),i=nn(r),o=new Set(["u_matrix","u_sizeRatio","u_correctionRatio","u_zoomRatio","u_pixelRatio","u_cameraAngle","u_minEdgeThickness","u_pickingPadding"]),s=new Set,l=[],u=function(b){!o.has(b.name)&&!s.has(b.name)&&(s.add(b.name),l.push("uniform ".concat(b.type," ").concat(b.name,";")))};a.forEach(function(p){return p.uniforms.forEach(u)}),n.forEach(function(p){return p.uniforms.forEach(u)}),t.forEach(function(p){return p.uniforms.forEach(u)});var d=n.map(function(p){var b;return he((b=p.baseRatio)!==null&&b!==void 0?b:.5)}).join(", "),c=function(b){var y,T;return n.length>1?"EXTREMITY_BASE_RATIOS[".concat(b,"]"):he((y=(T=n[0])===null||T===void 0?void 0:T.baseRatio)!==null&&y!==void 0?y:.5)},h=n.some(function(p){return Be(p.length)?!0:p.length>0}),f=t.some(function(p){return p.needsNodeColors}),g=a.some(function(p){return p.needsNodeSize}),m=t.map(function(p,b){return"  // Layer ".concat(b+1,": ").concat(p.name,`
  color = blendOver(color, layer_`).concat(p.name,"(context));")}).join(`

`),v=`#version 300 es
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
`).concat(Oa(a),`

// Path selector functions
`).concat(Ha(a),`

// All extremity functions
`).concat(bd(n),`

// Extremity SDF selector (shared pool for head/tail)
`).concat(_d(n),`

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

`).concat(h?`  // Base ratios, from baseRatioLookup
  float headBaseRatio = `.concat(c("v_headId"),`;
  float tailBaseRatio = `).concat(c("v_tailId"),`;

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

`).concat(m,`

    // Mix with transparent to fade both color AND alpha (pre-multiplied alpha for correct blending)
    fragColor = mix(vec4(0.0), color, alpha);
  #endif
}
`);return v}function Dd(a,n,t){var e=Be(n.length)?!0:n.length>0,r=Be(t.length)?!0:t.length>0;if(a.generateConstantData){var i=a.generateConstantData();return{data:i.data,attributes:i.attributes,verticesPerEdge:i.verticesPerEdge}}return Ed(a.segments,e,r)}function Wi(a){var n,t=ro(a),e=t.paths,r=t.extremities,i=t.layers,o=(n=a.antialias)!==null&&n!==void 0?n:!0,s=new Map,l=new Map,u=new Map,d=z(e),c;try{for(d.s();!(c=d.n()).done;){var h=c.value,f=z(r),g;try{for(f.s();!(g=f.n()).done;){var m=g.value,v=z(r),p;try{for(v.s();!(p=v.n()).done;){var b=p.value,y="".concat(h.name,":").concat(m.name,":").concat(b.name),T=Dd(h,m,b);s.set(y,T.verticesPerEdge),l.set(y,T.data);var x=z(T.attributes),_;try{for(x.s();!(_=x.n()).done;){var A=_.value;u.has(A.name)||u.set(A.name,A)}}catch(j){x.e(j)}finally{x.f()}}}catch(j){v.e(j)}finally{v.f()}}}catch(j){f.e(j)}finally{f.f()}}}catch(j){d.e(j)}finally{d.f()}var E=Array.from(u.values()),L={};E.forEach(function(j,q){L[j.name]=q});var w=0,F="",D=z(s),I;try{for(D.s();!(I=D.n()).done;){var R=ae(I.value,2),P=R[0],G=R[1];G>w&&(w=G,F=P)}}catch(j){D.e(j)}finally{D.f()}var N=l.get(F)||[],k=F.split(":"),C=ae(k,1),M=C[0],$=e.find(function(j){return j.name===M}),H=[];$!=null&&$.generateConstantData?H=$.generateConstantData().attributes:H=[{name:"a_zone"},{name:"a_zoneT"},{name:"a_side"}];var Y=N.map(function(j){var q=new Array(E.length).fill(0);return H.forEach(function(be,ve){var ye=L[be.name];ye!==void 0&&ve<j.length&&(q[ye]=j[ve])}),q});return{vertexShader:Ad(e,r,i,E),fragmentShader:wd(e,r,i,o),uniforms:md(e,r,i),attributes:Rd(),verticesPerEdge:w,constantData:Y,constantAttributes:E,vertexCountsPerCombination:s,constantDataPerCombination:l}}function Rd(a,n,t){return[{name:"a_edgeIndex",size:1,type:Qt},{name:"a_color",size:4,type:zi,normalized:!0},{name:"a_id",size:4,type:zi,normalized:!0},{name:"a_opacity",size:1,type:Qt}]}var Ld=`#version 300 es
precision highp float;

in vec4 v_clamp;

// RGBA32F target: stores (tStart, tEnd, straightenFactor, pathLength).
out vec4 fragColor;

void main() {
  fragColor = v_clamp;
}
`,Cd=(function(){function a(n,t){J(this,a),S(this,"uniformLocations",{});var e=t.paths,r=t.extremities,i=t.layers,o=t.nodeShapes,s=t.nodeLayers;this.gl=n,this.hasAttributeData=Le([].concat(V(e),V(i))).floatsPerItem>0,this.hasNodeAttributeData=Object.keys(Lt(o,s).offsets).length>0,this.vertexShader=Na(n,Sd(e,r,i,o,s)),this.fragmentShader=Ga(n,Ld),this.program=Ma(n,[this.vertexShader,this.fragmentShader]);var l=new Set;this.customUniforms=[];for(var u=0,d=[].concat(V(e),V(r));u<d.length;u++){var c=d[u],h=z(c.uniforms),f;try{for(h.s();!(f=h.n()).done;){var g=f.value;l.has(g.name)||(l.add(g.name),this.customUniforms.push(g))}}catch(y){h.e(y)}finally{h.f()}}var m=["u_sizeRatio","u_correctionRatio","u_cameraAngle","u_minEdgeThickness","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge","u_layerAttributeTexture","u_layerAttributeTextureWidth","u_layerAttributeTexelsPerNode","u_frameTextureWidth","u_frameTextureHeight"].concat(V(this.customUniforms.map(function(y){return y.name}))),v=z(m),p;try{for(v.s();!(p=v.n()).done;){var b=p.value;this.uniformLocations[b]=n.getUniformLocation(this.program,b)}}catch(y){v.e(y)}finally{v.f()}this.vao=n.createVertexArray()}return ee(a,[{key:"run",value:function(t,e,r,i,o){if(r!==0){var s=this.gl,l=this.uniformLocations;s.useProgram(this.program),s.bindVertexArray(this.vao),l.u_sizeRatio&&s.uniform1f(l.u_sizeRatio,t.sizeRatio),l.u_correctionRatio&&s.uniform1f(l.u_correctionRatio,t.correctionRatio),l.u_cameraAngle&&s.uniform1f(l.u_cameraAngle,t.cameraAngle),l.u_minEdgeThickness&&s.uniform1f(l.u_minEdgeThickness,t.minEdgeThickness),l.u_nodeDataTexture&&s.uniform1i(l.u_nodeDataTexture,t.nodeDataTextureUnit),l.u_nodeDataTextureWidth&&s.uniform1i(l.u_nodeDataTextureWidth,t.nodeDataTextureWidth),l.u_edgeDataTexture&&s.uniform1i(l.u_edgeDataTexture,t.edgeDataTextureUnit),l.u_edgeDataTextureWidth&&s.uniform1i(l.u_edgeDataTextureWidth,t.edgeDataTextureWidth),l.u_frameTextureWidth&&s.uniform1f(l.u_frameTextureWidth,e.getTextureWidth()),l.u_frameTextureHeight&&s.uniform1f(l.u_frameTextureHeight,e.getTextureHeight()),this.hasAttributeData&&i&&(i.bind(Ke),l.u_edgeAttributeTexture&&s.uniform1i(l.u_edgeAttributeTexture,Ke),l.u_edgeAttributeTextureWidth&&s.uniform1i(l.u_edgeAttributeTextureWidth,i.getTextureWidth()),l.u_edgeAttributeTexelsPerEdge&&s.uniform1i(l.u_edgeAttributeTexelsPerEdge,i.getTexelsPerItem())),this.hasNodeAttributeData&&o&&(o.bind(qe),l.u_layerAttributeTexture&&s.uniform1i(l.u_layerAttributeTexture,qe),l.u_layerAttributeTextureWidth&&s.uniform1i(l.u_layerAttributeTextureWidth,o.getTextureWidth()),l.u_layerAttributeTexelsPerNode&&s.uniform1i(l.u_layerAttributeTexelsPerNode,o.getTexelsPerItem()));var u=z(this.customUniforms),d;try{for(u.s();!(d=u.n()).done;){var c=d.value;Ua(s,this.uniformLocations[c.name],c)}}catch(h){u.e(h)}finally{u.f()}e.bindAsRenderTarget(),s.disable(s.BLEND),s.disable(s.DEPTH_TEST),s.drawArrays(s.POINTS,0,r),s.bindFramebuffer(s.FRAMEBUFFER,null),s.bindVertexArray(null)}}},{key:"kill",value:function(){var t=this.gl;t.deleteProgram(this.program),t.deleteShader(this.vertexShader),t.deleteShader(this.fragmentShader),t.deleteVertexArray(this.vao)}}])})();function Yt(a,n,t,e,r,i){if(a.length===1)return"".concat(e," ").concat(n,"(int pathId, ").concat(r,`) {
  return path_`).concat(a[0].name,"_").concat(t,"(").concat(i,`);
}`);var o=a.map(function(s,l){return"    case ".concat(l,": return path_").concat(s.name,"_").concat(t,"(").concat(i,");")}).join(`
`);return"".concat(e," ").concat(n,"(int pathId, ").concat(r,`) {
  switch (pathId) {
`).concat(o,`
    default: return path_`).concat(a[0].name,"_").concat(t,"(").concat(i,`);
  }
}`)}var kd=`
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
`;function Pd(a,n){var t=he(a),e=he(n);return`
float computeEdgeLabelAlpha(float bodyLength, float textWidthWebGL) {
  float ratio = textWidthWebGL > 0.0001 ? min(bodyLength / textWidthWebGL, 1.0) : 1.0;
  if (ratio < `.concat(t,`) return 0.0;
  if (ratio < `).concat(e,") return (ratio - ").concat(t,") / (").concat(e," - ").concat(t,`);
  return 1.0;
}
`)}var Fd=`
float computeEdgeLabelScreenDir(int pathId, float t, vec2 source, vec2 target, mat3 matrix) {
  vec2 tangent = queryPathTangent(pathId, t, source, target);
  return (matrix * vec3(tangent, 0.0)).x < 0.0 ? -1.0 : 1.0;
}
`,Id=`
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
`;function oo(a){var n=a.paths,t=a.minVisibilityThreshold,e=a.fullVisibilityThreshold,r=n.some(function(s){return s.hasSharpCorners}),i=n.map(function(s){return"// --- Path: ".concat(s.name,` ---
`).concat(s.glsl,`

// Tangent/normal functions: analytical if provided, otherwise numerical
`).concat(s.analyticalTangentGlsl||no(s.name),`

// Auto-generated fallbacks for any missing path functions
`).concat(ao(s.name,s.glsl),`

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

`.concat(Ue,`
`).concat(Rt,`

// ============================================================================
// Path Functions (one block per path)
// ============================================================================

`).concat(i,`

// ============================================================================
// Path Query Selectors (dispatch by pathId)
// ============================================================================

`).concat(Yt(n,"queryPathPosition","position","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Yt(n,"queryPathTangent","tangent","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Yt(n,"queryPathNormal","normal","vec2","float t, vec2 source, vec2 target","t, source, target"),`

`).concat(Yt(n,"queryPathLength","length","float","vec2 source, vec2 target","source, target"),`

`).concat(Yt(n,"queryPathTAtDistance","t_at_distance","float","float dist, vec2 source, vec2 target","dist, source, target"),`

`).concat(o,`

// ============================================================================
// Shared helpers (body bounds, alpha ramp, screen direction, perpendicular offset)
// ============================================================================

`).concat(kd,`
`).concat(Pd(t,e),`
`).concat(Fd,`
`).concat(Id,`
`)}var Nd=Fe.fontSize,Gd=new Map,so=24,Ui=(so+1)*2;function Md(a){var n=a.paths,t=a.fontSizeMode,e=a.headLengthRatio,r=a.tailLengthRatio,i=a.minVisibilityThreshold,o=a.fullVisibilityThreshold,s=a.keepUpright,l=t==="scaled",u=tt(),d=Le([].concat(V(n),[u])),c=nn(d),h=`#version 300 es

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

`).concat(c.uniformDeclarations,`

out vec4 v_color;
out vec4 v_id;
out float v_alphaModifier;

const float ATLAS_FONT_SIZE = `).concat(he(Nd),`;
const float HEAD_RATIO = `).concat(he(e),`;
const float TAIL_RATIO = `).concat(he(r),`;
const int RIBBON_SEGMENTS = `).concat(so,`;

// Path attribute varyings (declared as plain locals since this shader has no FS inputs for them)
`).concat(c.vertexVaryingDeclarations.replace(/out /g,""),`

// Node size variables used by some path functions (self-loops, etc.)
float v_sourceNodeSize;
float v_targetNodeSize;

// Shared preamble: shape SDFs, path functions + selectors, clamp, helpers.
`).concat(oo({paths:n,minVisibilityThreshold:i,fullVisibilityThreshold:o}),`

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
`).concat(c.fetchCode,`
`).concat(c.varyingAssignments,`
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
`);return h}var zd=`#version 300 es
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
`;function Bd(a,n,t,e){var r=e.shaderConfig,i=r.paths,o=r.fontSizeMode;if(i.length===0)throw new Error("createEdgeLabelBackgroundProgram: shaderConfig must declare at least one path");var s=tt(),l=Le([].concat(V(i),[s])),u=Vn([].concat(V(i),[s]),l),d=Md(r),c=(function(h){function f(g,m,v){var p;return J(this,f),p=le(this,f,[g,m,v]),S(p,"totalCount",0),S(p,"bufferCapacity",0),S(p,"edgeAttributeTexture",null),p.edgeAttributeTexture=new $n(g,l),p.packedAttributeData=new Float32Array(l.floatsPerItem),p}return ue(f,h),ee(f,[{key:"getDefinition",value:function(){for(var m=WebGL2RenderingContext,v=m.FLOAT,p=m.UNSIGNED_BYTE,b=m.TRIANGLE_STRIP,y=[],T=0;T<Ui;T++)y.push([T]);var x=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_resolution","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth"];return o==="scaled"&&x.push("u_zoomSizeRatio"),l.floatsPerItem>0&&x.push("u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"),{VERTICES:Ui,VERTEX_SHADER_SOURCE:d,FRAGMENT_SHADER_SOURCE:zd,METHOD:b,UNIFORMS:x,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:v},{name:"a_edgeAttrIndex",size:1,type:v},{name:"a_baseFontSize",size:1,type:v},{name:"a_totalTextWidth",size:1,type:v},{name:"a_positionMode",size:1,type:v},{name:"a_margin",size:1,type:v},{name:"a_padding",size:1,type:v},{name:"a_color",size:4,type:p,normalized:!0},{name:"a_id",size:4,type:p,normalized:!0}],CONSTANT_ATTRIBUTES:[{name:"a_vertexIndex",size:1,type:v}],CONSTANT_DATA:y}}},{key:"processEdgeLabelBackground",value:function(m,v,p){var b=0;if(this.edgeAttributeTexture&&l.floatsPerItem>0){b=this.edgeAttributeTexture.allocate(v);var y=this.packedAttributeData;jn(u,p.edgeAttributes,y,"",Gd,0),this.edgeAttributeTexture.updateAllAttributes(v,y)}var T=this.floats,x=this.ints,_=m*this.STRIDE;T[_++]=p.edgeIndex,T[_++]=b,T[_++]=p.baseFontSize,T[_++]=p.totalTextWidth,T[_++]=p.positionMode,T[_++]=p.margin,T[_++]=p.padding,T[_++]=p.color,x[_++]=p.id}},{key:"setUniforms",value:function(m,v){var p=v.gl,b=v.uniformLocations;if(p.uniformMatrix3fv(b.u_matrix,!1,m.matrix),p.uniform1f(b.u_sizeRatio,m.sizeRatio),p.uniform1f(b.u_correctionRatio,m.correctionRatio),p.uniform1f(b.u_pixelRatio,m.pixelRatio),p.uniform1f(b.u_cameraAngle,m.cameraAngle),p.uniform2f(b.u_resolution,m.width,m.height),b.u_nodeDataTexture!==void 0&&p.uniform1i(b.u_nodeDataTexture,m.nodeDataTextureUnit),b.u_nodeDataTextureWidth!==void 0&&p.uniform1i(b.u_nodeDataTextureWidth,m.nodeDataTextureWidth),b.u_edgeDataTexture!==void 0&&p.uniform1i(b.u_edgeDataTexture,m.edgeDataTextureUnit),b.u_edgeDataTextureWidth!==void 0&&p.uniform1i(b.u_edgeDataTextureWidth,m.edgeDataTextureWidth),b.u_edgeFrameTexture!==void 0&&p.uniform1i(b.u_edgeFrameTexture,m.edgeFrameTextureUnit),b.u_edgeFrameTextureWidth!==void 0&&p.uniform1i(b.u_edgeFrameTextureWidth,m.edgeFrameTextureWidth),o==="scaled"&&b.u_zoomSizeRatio!==void 0){var y=this.renderer.getSetting("zoomToSizeRatioFunction");p.uniform1f(b.u_zoomSizeRatio,1/y(m.zoomRatio))}this.edgeAttributeTexture&&l.floatsPerItem>0&&b.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(Ke),p.uniform1i(b.u_edgeAttributeTexture,Ke),p.uniform1i(b.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),p.uniform1i(b.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()))}},{key:"renderProgram",value:function(m,v){this.edgeAttributeTexture&&l.floatsPerItem>0&&this.edgeAttributeTexture.upload(),ge(f,"renderProgram",this,3)([m,v])}},{key:"kill",value:function(){this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),ge(f,"kill",this,3)([])}},{key:"drawWebGL",value:function(m,v){var p=v.gl;this.totalCount!==0&&p.drawArraysInstanced(p.TRIANGLE_STRIP,0,this.VERTICES,this.totalCount)}},{key:"reallocate",value:function(m){this.totalCount=m,m>this.bufferCapacity&&(this.bufferCapacity=Math.max(m,Math.ceil(this.bufferCapacity*1.5)||10),ge(f,"reallocate",this,3)([this.bufferCapacity]))}}])})(it);return new c(a,n,t)}function lo(a){var n,t,e,r,i,o;return{paths:a.paths,headLengthRatio:(n=a.headLengthRatio)!==null&&n!==void 0?n:0,tailLengthRatio:(t=a.tailLengthRatio)!==null&&t!==void 0?t:0,fontSizeMode:(e=a.fontSizeMode)!==null&&e!==void 0?e:"fixed",minVisibilityThreshold:(r=a.minVisibilityThreshold)!==null&&r!==void 0?r:.7,fullVisibilityThreshold:(i=a.fullVisibilityThreshold)!==null&&i!==void 0?i:.8,keepUpright:(o=a.keepUpright)!==null&&o!==void 0?o:!0}}var Wd=Fe.fontSize,Ud=17/64;function Od(a){var n=a.paths,t=a.hasBorder,e=t===void 0?!1:t,r=a.fontSizeMode,i=r===void 0?"fixed":r,o=a.minVisibilityThreshold,s=o===void 0?.5:o,l=a.fullVisibilityThreshold,u=l===void 0?.6:l,d=a.keepUpright,c=d===void 0?!0:d,h=i==="scaled",f=n.some(function(b){return b.hasSharpCorners}),g=tt(),m=Le([].concat(V(n),[g])),v=nn(m),p=`#version 300 es

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
`).concat(h?"uniform float u_zoomSizeRatio;  // Zoom-based size ratio from zoomToSizeRatioFunction":"",`

// Edge path attribute texture uniforms (for curvature and other path attributes)
`).concat(v.uniformDeclarations,`

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
const float ATLAS_FONT_SIZE = `).concat(he(Wd),`;  // Base font size used in SDF atlas
const float VERTICAL_CENTER_RATIO = `).concat(he(Ud),`;  // Baseline to visual center ratio

// ============================================================================
// Path Attribute Variables (set in main, used by path functions)
// ============================================================================
// Path attributes are fetched from the edge attribute texture and stored in
// variables with v_ prefix (e.g., v_curvature) for path functions to access.
`).concat(v.vertexVaryingDeclarations.replace(/out /g,""),`

// Node size variables (set in main, used by some path functions like loops).
// These mirror the v_sourceNodeSize / v_targetNodeSize varyings in generator.ts,
// but are plain floats here since the label shader is vertex-only.
float v_sourceNodeSize;
float v_targetNodeSize;

// Shared preamble: shape SDFs, path functions + selectors, clamp, helpers.
`).concat(oo({paths:n,minVisibilityThreshold:s,fullVisibilityThreshold:u}),`

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
`).concat(v.fetchCode,`
`).concat(v.varyingAssignments,`
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
  `).concat(h?`// Scaled mode: font scales with zoom
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
  float readingSign = `).concat(c?"screenDir":"1.0",`;

  // Character center position relative to label center (on centerline, before curvature adjustment)
  float charCenterOffset = readingSign * (charOffsetWebGL + charAdvanceWebGL * 0.5 - textWidthWebGL * 0.5);

  // -------------------------------------------------------------------------
  // Step 6: Compute perpendicular offset based on position mode
  // -------------------------------------------------------------------------
  // Position modes: 0=over, 1=above, 2=below, 3=auto
  // Needed early for curvature-adaptive character spacing in Step 7
  float halfThickness = webGLThickness * 0.5;
  `).concat(h?`// Scaled mode: margin and text height scale with zoom (same factor as font)
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
    `.concat(h?"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * u_zoomSizeRatio * u_correctionRatio / u_sizeRatio;":"float skipDistGraph = STEP_INNER_CORNER_SKIP_FACTOR * baseFontSize * pixelToGraph;",`

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
  `).concat(h?"vec2 quadPosWebGL = quadPos * u_correctionRatio / u_sizeRatio; // Scaled mode":"vec2 quadPosWebGL = quadPos * pixelToGraph; // Fixed mode: use pixelToGraph for zoom-independent size",`

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
  `).concat(h?"float fadeWidthWebGL = FADE_WIDTH_PIXELS * u_correctionRatio / u_sizeRatio;":"float fadeWidthWebGL = FADE_WIDTH_PIXELS * pixelToGraph;",`

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
`);return p}function Hd(){var a=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},n=a.hasBorder,t=n===void 0?!1:n,e=`#version 300 es
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
`);return e}function $d(a){var n=arguments.length>1&&arguments[1]!==void 0?arguments[1]:!1,t=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"fixed",e=["u_matrix","u_sizeRatio","u_correctionRatio","u_pixelRatio","u_cameraAngle","u_sdfBufferPixels","u_resolution","u_atlasSize","u_atlas","u_gamma","u_sdfBuffer","u_nodeDataTexture","u_nodeDataTextureWidth","u_edgeDataTexture","u_edgeDataTextureWidth","u_edgeFrameTexture","u_edgeFrameTextureWidth","u_edgeAttributeTexture","u_edgeAttributeTextureWidth","u_edgeAttributeTexelsPerEdge"];t==="scaled"&&e.push("u_zoomSizeRatio"),n&&e.push("u_borderWidth");var r=z(a),i;try{for(r.s();!(i=r.n()).done;){var o=i.value,s=z(o.uniforms),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;e.includes(u.name)||e.push(u.name)}}catch(d){s.e(d)}finally{s.f()}}}catch(d){r.e(d)}finally{r.f()}return e}function Vd(a){var n=a.hasBorder,t=n===void 0?!1:n,e=a.fontSizeMode,r=e===void 0?"fixed":e;return{vertexShader:Od(a),fragmentShader:Hd({hasBorder:t}),uniforms:$d(a.paths,t,r)}}var jd=new Map;function Xd(a){switch(a){case"over":return 0;case"above":return 1;case"below":return 2;case"auto":return 3;default:return 0}}function qd(a,n,t,e){var r=e.color,i=e.margin,o=e.textBorder,s=lo(e),l=s.paths,u=s.fontSizeMode,d=s.minVisibilityThreshold,c=s.fullVisibilityThreshold,h=s.keepUpright,f=!!o,g=Vd({paths:l,hasBorder:f,fontSizeMode:u,minVisibilityThreshold:d,fullVisibilityThreshold:c,keepUpright:h}),m=(function(v){function p(b,y,T){var x;J(this,p),x=le(this,p,[b,y,T]),S(x,"atlasTexture",null),S(x,"atlasNeedsUpdate",!1),S(x,"refreshTimeout",null),S(x,"labelGlyphCache",new Map),S(x,"edgeAttributeTexture",null);var _=tt();if(x.attributeLayout=Le([].concat(V(l),[_])),x.attrDescriptors=Vn([].concat(V(l),[_]),x.attributeLayout),x.edgeAttributeTexture=new $n(b,x.attributeLayout),x.packedAttributeData=new Float32Array(x.attributeLayout.floatsPerItem),x.atlasManager=new at,x.gamma=.025,x.sdfBuffer=Fe.cutoff,x.atlasTexture=b.createTexture(),!x.atlasTexture)throw new Error("EdgeLabelProgram: failed to create atlas texture");b.bindTexture(b.TEXTURE_2D,x.atlasTexture),b.texParameteri(b.TEXTURE_2D,b.TEXTURE_WRAP_S,b.CLAMP_TO_EDGE),b.texParameteri(b.TEXTURE_2D,b.TEXTURE_WRAP_T,b.CLAMP_TO_EDGE),b.texParameteri(b.TEXTURE_2D,b.TEXTURE_MIN_FILTER,b.LINEAR),b.texParameteri(b.TEXTURE_2D,b.TEXTURE_MAG_FILTER,b.LINEAR),b.bindTexture(b.TEXTURE_2D,null),x.atlasManager.on(at.ATLAS_UPDATED_EVENT,function(){x.atlasNeedsUpdate=!0,x.refreshTimeout!==null&&clearTimeout(x.refreshTimeout),x.refreshTimeout=setTimeout(function(){x.refreshTimeout=null,x.renderer.refresh()},0)});var A={family:"sans-serif",weight:"normal",style:"normal"};return x.defaultFontKey=x.atlasManager.registerFont(A),x}return ue(p,v),ee(p,[{key:"getDefinition",value:function(){var y=WebGL2RenderingContext,T=y.FLOAT,x=y.UNSIGNED_BYTE,_=y.TRIANGLE_STRIP,A=new Set,E=[],L=z(l),w;try{for(L.s();!(w=L.n()).done;){var F=w.value,D=z(F.attributes),I;try{for(D.s();!(I=D.n()).done;){var R=I.value,P=R.name.startsWith("a_")?R.name:"a_".concat(R.name);A.has(P)||(A.add(P),E.push({name:P,size:R.size,type:R.type}))}}catch(G){D.e(G)}finally{D.f()}}}catch(G){L.e(G)}finally{L.f()}return{VERTICES:4,VERTEX_SHADER_SOURCE:g.vertexShader,FRAGMENT_SHADER_SOURCE:g.fragmentShader,METHOD:_,UNIFORMS:g.uniforms,ATTRIBUTES:[{name:"a_edgeIndex",size:1,type:T},{name:"a_edgeAttrIndex",size:1,type:T},{name:"a_baseFontSize",size:1,type:T},{name:"a_charMetrics",size:4,type:T},{name:"a_charDims",size:4,type:T},{name:"a_texCoords",size:4,type:T},{name:"a_labelParams",size:2,type:T},{name:"a_color",size:4,type:x,normalized:!0}].concat(V(f?[{name:"a_borderColor",size:4,type:x,normalized:!0}]:[]),V(E.filter(function(G){return!["a_curvature","curvature"].includes(G.name)}))),CONSTANT_ATTRIBUTES:[{name:"a_quadCorner",size:2,type:T}],CONSTANT_DATA:[[0,0],[1,0],[0,1],[1,1]]}}},{key:"prepareLabelGlyphs",value:function(y,T){if(T.hidden||!T.text){this.labelGlyphCache.delete(y);return}var x=T.text,_=T.fontKey||this.defaultFontKey;this.atlasManager.ensureGlyphs(x,_);var A=[],E=[],L=0,w=z(x),F;try{for(w.s();!(F=w.n()).done;){var D=F.value,I=D.codePointAt(0);if(I===void 0){A.push(void 0),E.push(L);continue}var R=this.atlasManager.getGlyph(I,_);A.push(R),E.push(L),R&&(L+=R.advance)}}catch(P){w.e(P)}finally{w.f()}this.labelGlyphCache.set(y,{glyphs:A,xOffsets:E,totalWidth:L})}},{key:"processCharacter",value:function(y,T,x,_){var A,E,L,w=this.floats,F=this.STRIDE,D=y*F,I=this.labelGlyphCache.get(T.parentKey);if(!I||!I.glyphs[_]){for(var R=0;R<F;R++)w[D+R]=0;return}var P=I.glyphs[_],G=I.xOffsets[_],N=p.labelColor,k=N&&te(N)==="object"&&"color"in N&&N.color?N.color:T.color,C=De(k),M=D;w[M++]=T.edgeIndex,w[M++]=(A=(E=this.edgeAttributeTexture)===null||E===void 0?void 0:E.getIndex(T.parentKey))!==null&&A!==void 0?A:0,w[M++]=T.size,w[M++]=G,w[M++]=P.advance,w[M++]=I.totalWidth,w[M++]=Xd(T.position),w[M++]=P.atlasWidth,w[M++]=P.atlasHeight,w[M++]=P.bearingX,w[M++]=-P.bearingY,w[M++]=P.atlasX,w[M++]=P.atlasY,w[M++]=P.atlasWidth,w[M++]=P.atlasHeight;var $=(L=p.labelMargin)!==null&&L!==void 0?L:T.margin;if(w[M++]=$,w[M++]=0,w[M++]=C,f&&o){var H;typeof o.color=="string"?H=o.color:H=o.color.color||"#ffffff",w[M++]=De(H)}var Y=new Set,j=z(l),q;try{for(j.s();!(q=j.n()).done;){var be=q.value,ve=z(be.attributes),ye;try{for(ve.s();!(ye=ve.n()).done;){var Oe=ye.value,He=Oe.name.startsWith("a_")?Oe.name.slice(2):Oe.name;if(He!=="curvature"&&!Y.has(He)){Y.add(He);for(var xe=0;xe<Oe.size;xe++)w[M++]=0}}}catch(Qe){ve.e(Qe)}finally{ve.f()}}}catch(Qe){j.e(Qe)}finally{j.f()}}},{key:"processEdgeLabel",value:function(y,T,x){if(this.prepareLabelGlyphs(y,x),this.edgeAttributeTexture&&!x.hidden&&x.text){this.edgeAttributeTexture.allocate(y);var _=this.packedAttributeData;jn(this.attrDescriptors,x.edgeAttributes,_,"",jd,0),this.edgeAttributeTexture.updateAllAttributes(y,_)}return ge(p,"processLabel",this,3)([y,T,x])}},{key:"updateAtlasTexture",value:function(){if(this.atlasNeedsUpdate){var y=this.normalProgram.gl,T=this.atlasManager.getTextures();if(T.length!==0){var x=T[0];y.bindTexture(y.TEXTURE_2D,this.atlasTexture),y.texImage2D(y.TEXTURE_2D,0,y.RGBA,x.width,x.height,0,y.RGBA,y.UNSIGNED_BYTE,x.data),y.bindTexture(y.TEXTURE_2D,null),this.atlasNeedsUpdate=!1}}}},{key:"setUniforms",value:function(y,T){var x=T.gl,_=T.uniformLocations,A=this.atlasManager.getTextures(),E=A.length>0?[A[0].width,A[0].height]:[1,1];if(x.uniformMatrix3fv(_.u_matrix,!1,y.matrix),x.uniform1f(_.u_sizeRatio,y.sizeRatio),x.uniform1f(_.u_correctionRatio,y.correctionRatio),x.uniform1f(_.u_pixelRatio,y.pixelRatio),x.uniform1f(_.u_cameraAngle,y.cameraAngle),x.uniform1f(_.u_sdfBufferPixels,Fe.buffer),x.uniform2f(_.u_resolution,y.width,y.height),x.uniform2f(_.u_atlasSize,E[0],E[1]),x.uniform1f(_.u_gamma,this.gamma),x.uniform1f(_.u_sdfBuffer,this.sdfBuffer),x.activeTexture(x.TEXTURE0),x.bindTexture(x.TEXTURE_2D,this.atlasTexture),x.uniform1i(_.u_atlas,0),_.u_nodeDataTexture!==void 0&&x.uniform1i(_.u_nodeDataTexture,y.nodeDataTextureUnit),_.u_nodeDataTextureWidth!==void 0&&x.uniform1i(_.u_nodeDataTextureWidth,y.nodeDataTextureWidth),_.u_edgeDataTexture!==void 0&&x.uniform1i(_.u_edgeDataTexture,y.edgeDataTextureUnit),_.u_edgeDataTextureWidth!==void 0&&x.uniform1i(_.u_edgeDataTextureWidth,y.edgeDataTextureWidth),_.u_edgeFrameTexture!==void 0&&x.uniform1i(_.u_edgeFrameTexture,y.edgeFrameTextureUnit),_.u_edgeFrameTextureWidth!==void 0&&x.uniform1i(_.u_edgeFrameTextureWidth,y.edgeFrameTextureWidth),f&&o&&_.u_borderWidth!==void 0){var L=o.width/Fe.buffer*this.sdfBuffer;x.uniform1f(_.u_borderWidth,L)}if(this.edgeAttributeTexture&&_.u_edgeAttributeTexture!==void 0&&(this.edgeAttributeTexture.bind(Ke),x.uniform1i(_.u_edgeAttributeTexture,Ke),x.uniform1i(_.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),x.uniform1i(_.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem())),u==="scaled"&&_.u_zoomSizeRatio!==void 0){var w=this.renderer.getSetting("zoomToSizeRatioFunction"),F=1/w(y.zoomRatio);x.uniform1f(_.u_zoomSizeRatio,F)}}},{key:"renderProgram",value:function(y,T){this.updateAtlasTexture(),this.atlasManager.hasPendingGlyphs()&&(this.atlasManager.flush(),this.updateAtlasTexture()),this.edgeAttributeTexture&&this.edgeAttributeTexture.upload(),ge(p,"renderProgram",this,3)([y,T])}},{key:"registerFont",value:function(y){var T=arguments.length>1&&arguments[1]!==void 0?arguments[1]:"normal",x=arguments.length>2&&arguments[2]!==void 0?arguments[2]:"normal";return this.atlasManager.registerFont({family:y,weight:T,style:x})}},{key:"getAtlasManager",value:function(){return this.atlasManager}},{key:"ensureGlyphsReady",value:function(y,T){var x=T||this.defaultFontKey,_=z(y),A;try{for(_.s();!(A=_.n()).done;){var E=A.value;this.atlasManager.ensureGlyphs(E,x)}}catch(L){_.e(L)}finally{_.f()}this.atlasManager.flush()}},{key:"measureLabelAtlasWidth",value:function(y,T){var x=T||this.defaultFontKey;this.atlasManager.ensureGlyphs(y,x),this.atlasManager.hasPendingGlyphs()&&this.atlasManager.flush();var _=0,A=z(y),E;try{for(A.s();!(E=A.n()).done;){var L=E.value,w=L.codePointAt(0);if(w!==void 0){var F=this.atlasManager.getGlyph(w,x);F&&(_+=F.advance)}}}catch(D){A.e(D)}finally{A.f()}return _}},{key:"measureLabel",value:function(y,T,x){var _=this.measureLabelAtlasWidth(y,x),A=T/Fe.fontSize;return{width:_*A,height:T,textHeight:T}}},{key:"kill",value:function(){var y=this.normalProgram.gl;this.refreshTimeout!==null&&(clearTimeout(this.refreshTimeout),this.refreshTimeout=null),this.atlasTexture&&(y.deleteTexture(this.atlasTexture),this.atlasTexture=null),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),this.atlasManager.destroy(),this.labelGlyphCache.clear(),ge(p,"kill",this,3)([])}}])})(Ji);return S(m,"labelColor",r),S(m,"labelMargin",i),new m(a,n,t)}function Kd(){var a=`
// No extremity - always returns positive (outside)
float extremity_none(vec2 uv, float lengthRatio, float widthRatio) {
  return 1.0;
}
`;return{name:"none",glsl:a,length:0,widthFactor:1,margin:0,uniforms:[],attributes:[]}}function uo(a,n,t,e,r,i,o){var s,l,u=ro(e),d=u.paths,c=u.layers,h=u.defaultHead,f=u.defaultTail,g=[Kd()].concat(V(u.extremities)),m={},v={};d.forEach(function(D,I){return m[D.name]=I}),g.forEach(function(D,I){return v[D.name]=I});var p=(s=v[h])!==null&&s!==void 0?s:0,b=(l=v[f])!==null&&l!==void 0?l:0,y=null,T=Le([].concat(V(d),V(c))),x=(function(D){function I(R,P,G){var N;J(this,I),y||(y=Wi({paths:d,extremities:g,layers:c,antialias:r})),N=le(this,I,[R,P,G]),S(N,"layerLifecycles",new Map),S(N,"needsShaderRegeneration",!1),S(N,"edgeAttributeTexture",null),S(N,"layout",T),S(N,"attrDescriptors",[]),S(N,"lifecycleIndexOffset",d.length),N._pickingBuffer=P,N.edgeAttributeTexture=new $n(R,N.layout),N.packedAttributeData=new Float32Array(N.layout.floatsPerItem),c.forEach(function(C,M){if(C.lifecycle){var $={gl:R,renderer:{refresh:function(){return G.refresh()}},getUniformLocation:function(Y){return R.getUniformLocation(N.normalProgram.program,Y)},requestShaderRegeneration:function(){N.needsShaderRegeneration=!0},requestRefresh:function(){G.refresh()}};N.layerLifecycles.set(M,C.lifecycle($))}}),N.layerLifecycles.forEach(function(C){var M;return(M=C.init)===null||M===void 0?void 0:M.call(C)});var k=new Map;return N.layerLifecycles.forEach(function(C,M){C.getAttributeData&&k.set(d.length+M,C)}),N.attrDescriptors=Vn([].concat(V(d),V(c)),N.layout,k),N}return ue(I,D),ee(I,[{key:"getAttributeTexture",value:function(){return this.edgeAttributeTexture}},{key:"resolveEdgeIds",value:function(P,G,N){var k=P,C=0,M=p,$=b,H=G?k.selfLoopPath:N&&k.parallelPath?k.parallelPath:k.path;H&&m[H]!==void 0&&(C=m[H]),k.head&&k.head!=="none"&&v[k.head]!==void 0&&(M=v[k.head]),k.tail&&k.tail!=="none"&&v[k.tail]!==void 0&&($=v[k.tail]);var Y=g[M],j=g[$];return{pathId:C,headId:M,tailId:$,headLengthRatio:Be(Y.length)?0:Y.length,tailLengthRatio:Be(j.length)?0:j.length}}},{key:"getDefinition",value:function(){var P=WebGL2RenderingContext,G=P.TRIANGLE_STRIP,N=G,k=y;return{VERTICES:k.verticesPerEdge,VERTEX_SHADER_SOURCE:k.vertexShader,FRAGMENT_SHADER_SOURCE:k.fragmentShader,METHOD:N,UNIFORMS:k.uniforms,ATTRIBUTES:k.attributes,CONSTANT_ATTRIBUTES:k.constantAttributes,CONSTANT_DATA:k.constantData}}},{key:"maybeRegenerateShaders",value:function(){var P=this;if(this.needsShaderRegeneration){this.needsShaderRegeneration=!1;var G=c.map(function(Y,j){var q=P.layerLifecycles.get(j);return q!=null&&q.regenerate?q.regenerate():Y});y=Wi({paths:d,extremities:g,layers:G,antialias:this.renderer.getSetting("antialiasEdges")});var N=this.normalProgram.gl,k=this.normalProgram,C=k.program,M=k.buffer,$=k.vertexShader,H=k.fragmentShader;N.deleteProgram(C),N.deleteBuffer(M),N.deleteShader($),N.deleteShader(H),this.normalProgram=this.getProgramInfo("normal",N,y.vertexShader,y.fragmentShader,this._pickingBuffer)}}},{key:"process",value:function(P,G,N,k,C,M){var $=G*this.STRIDE;if(C.visibility==="hidden"||N.visibility==="hidden"||k.visibility==="hidden"){for(var H=$+this.STRIDE;$<H;$++)this.floats[$]=0;this.floats[G*this.STRIDE]=Yi;return}this.processVisibleItem(yt(P),$,N,k,C,M)}},{key:"processVisibleItem",value:function(P,G,N,k,C,M){var $,H=this.floats,Y=this.ints;H[G++]=M,H[G++]=De(C.color),Y[G++]=P,H[G++]=($=C.opacity)!==null&&$!==void 0?$:1;var j=this.packedAttributeData;jn(this.attrDescriptors,C,j,C.color,this.layerLifecycles,this.lifecycleIndexOffset),this.edgeAttributeTexture.updateAllAttributesAtRow(M,j)}},{key:"setUniforms",value:function(P,G){var N=this,k=G.gl,C=G.uniformLocations;C.u_matrix&&k.uniformMatrix3fv(C.u_matrix,!1,P.matrix),C.u_sizeRatio&&k.uniform1f(C.u_sizeRatio,P.sizeRatio),C.u_correctionRatio&&k.uniform1f(C.u_correctionRatio,P.correctionRatio),C.u_zoomRatio&&k.uniform1f(C.u_zoomRatio,P.zoomRatio),C.u_pixelRatio&&k.uniform1f(C.u_pixelRatio,P.pixelRatio),C.u_cameraAngle&&k.uniform1f(C.u_cameraAngle,P.cameraAngle),C.u_feather&&k.uniform1f(C.u_feather,P.antiAliasingFeather),C.u_minEdgeThickness&&k.uniform1f(C.u_minEdgeThickness,P.minEdgeThickness),C.u_pickingPadding&&k.uniform1f(C.u_pickingPadding,P.edgePickingPadding),C.u_nodeDataTexture&&k.uniform1i(C.u_nodeDataTexture,P.nodeDataTextureUnit),C.u_nodeDataTextureWidth&&k.uniform1i(C.u_nodeDataTextureWidth,P.nodeDataTextureWidth),C.u_edgeDataTexture&&k.uniform1i(C.u_edgeDataTexture,P.edgeDataTextureUnit),C.u_edgeDataTextureWidth&&k.uniform1i(C.u_edgeDataTextureWidth,P.edgeDataTextureWidth),C.u_edgeFrameTexture&&k.uniform1i(C.u_edgeFrameTexture,P.edgeFrameTextureUnit),C.u_edgeFrameTextureWidth&&k.uniform1i(C.u_edgeFrameTextureWidth,P.edgeFrameTextureWidth),this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&(this.edgeAttributeTexture.bind(Ke),C.u_edgeAttributeTexture&&k.uniform1i(C.u_edgeAttributeTexture,Ke),C.u_edgeAttributeTextureWidth&&k.uniform1i(C.u_edgeAttributeTextureWidth,this.edgeAttributeTexture.getTextureWidth()),C.u_edgeAttributeTexelsPerEdge&&k.uniform1i(C.u_edgeAttributeTexelsPerEdge,this.edgeAttributeTexture.getTexelsPerItem()));var M=new Set;d.forEach(function($){$.uniforms.forEach(function(H){M.has(H.name)||(M.add(H.name),N.setTypedUniform(H,G))})}),g.forEach(function($){$.uniforms.forEach(function(H){M.has(H.name)||(M.add(H.name),N.setTypedUniform(H,G))})}),c.forEach(function($){$.uniforms.forEach(function(H){M.has(H.name)||(M.add(H.name),N.setTypedUniform(H,G))})})}},{key:"render",value:function(P,G,N){this.maybeRegenerateShaders(),ge(I,"render",this,3)([P,G,N])}},{key:"renderProgram",value:function(P,G){this.layerLifecycles.forEach(function(N){var k;return(k=N.beforeRender)===null||k===void 0?void 0:k.call(N)}),ge(I,"renderProgram",this,3)([P,G])}},{key:"uploadAttributeTexture",value:function(){this.edgeAttributeTexture&&this.layout.floatsPerItem>0&&this.edgeAttributeTexture.upload()}},{key:"kill",value:function(){this.layerLifecycles.forEach(function(P){var G;return(G=P.kill)===null||G===void 0?void 0:G.call(P)}),this.edgeAttributeTexture&&(this.edgeAttributeTexture.kill(),this.edgeAttributeTexture=null),ge(I,"kill",this,3)([])}}])})(it),_=g[p],A=g[b],E=B({paths:d,headLengthRatio:Be(_.length)?0:_.length,tailLengthRatio:Be(A.length)?0:A.length},e.label),L=lo(E),w=function(){return{labelProgram:qd(a,null,t,E),labelBackgroundProgram:Bd(a,n,t,{shaderConfig:L})}},F=new Cd(a,{paths:d,extremities:g,layers:c,nodeShapes:i,nodeLayers:o});return{edgeProgram:new x(a,null,t),createLabelPrograms:w,framePass:F}}function Yd(a){return te(a)==="object"&&"uniforms"in a&&Array.isArray(a.uniforms)}function Zd(a){return te(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function Qd(a){return te(a)==="object"&&"uniforms"in a&&"attributes"in a&&"segments"in a}function Jd(a){return te(a)==="object"&&"uniforms"in a&&"attributes"in a&&"length"in a}function ec(a){return te(a)==="object"&&"uniforms"in a&&"attributes"in a&&"glsl"in a}function tc(a){return te(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function nc(a){return te(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function ac(a){return te(a)==="object"&&"glsl"in a&&"name"in a&&!("uniforms"in a)}function rc(a){if(Yd(a))return a;if(tc(a))return{name:a.name,glsl:a.glsl,inradiusFactor:a.inradiusFactor,uniforms:[]};throw new Error("Invalid node shape specification: ".concat(JSON.stringify(a)))}function ic(a){if(Zd(a))return a;if(gi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid node layer specification: ".concat(JSON.stringify(a)))}function oc(a){if(Qd(a))return a;if(nc(a))return{name:a.name,glsl:a.glsl,segments:a.segments,uniforms:[],attributes:[]};throw new Error("Invalid edge path specification: ".concat(JSON.stringify(a)))}function sc(a){if(ec(a))return a;if(pi(a))return{name:a.name,glsl:a.glsl,uniforms:[],attributes:[]};throw new Error("Invalid edge layer specification: ".concat(JSON.stringify(a)))}function lc(a){if(Jd(a))return a;if(ac(a))return{name:a.name,glsl:a.glsl,length:a.length,widthFactor:a.widthFactor,margin:0,uniforms:[],attributes:[]};throw new Error("Invalid edge extremity specification: ".concat(JSON.stringify(a)))}function co(a){var n,t,e=(n=a?.shapes)!==null&&n!==void 0?n:$t.shapes,r=(t=a?.layers)!==null&&t!==void 0?t:$t.layers,i=e.map(rc),o=r.map(ic);if(i.length===0)throw new Error("At least one node shape must be specified.");if(o.length===0)throw new Error("At least one node layer must be specified.");return{shapes:i,layers:o}}function uc(a){var n,t,e,r=(n=a?.paths)!==null&&n!==void 0?n:xt.paths,i=(t=a?.extremities)!==null&&t!==void 0?t:xt.extremities,o=(e=a?.layers)!==null&&e!==void 0?e:xt.layers,s=r.map(oc),l=i.map(lc).filter(function(d){return d!==null}),u=o.map(sc);if(s.length===0)throw new Error("At least one edge path must be specified.");if(u.length===0)throw new Error("At least one edge layer must be specified.");return{paths:s,extremities:l,layers:u}}function ho(a,n){var t={},e=z(a),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;i.variables&&Object.assign(t,i.variables)}}catch(o){e.e(o)}finally{e.f()}return Object.assign(t,n||{}),t}function fo(a,n,t,e,r){var i=co(e),o=i.shapes,s=i.layers,l=ho(o,e?.variables),u=to(a,n,t,{shapes:o,layers:s,label:e?.label,backdrop:e?.backdrop,labelAttachments:e?.labelAttachments},r);return B(B({},u),{},{variables:l})}function go(a,n,t,e,r,i){var o=uc(e),s=o.paths,l=o.extremities,u=o.layers,d=co(i),c=d.shapes,h=d.layers,f=ho(s,e?.variables),g=uo(a,n,t,{paths:s,extremities:l,layers:u,defaultHead:e?.defaultHead,defaultTail:e?.defaultTail,label:e?.label},r,c,h);return B(B({},g),{},{variables:f,paths:s})}var dc=2;function po(a,n,t){var e=arguments.length>3&&arguments[3]!==void 0?arguments[3]:dc,r=n.canvas,i=r.width,o=r.height,s=t.x,l=t.y,u=t.rowHeight,d=t.maxRowWidth,c={},h=[],f=z(a),g;try{for(f.s();!(g=f.n()).done;){var m=g.value,v=m.width+e,p=m.height+e;if(v>i||p>o||s+v>i&&l+u+p>o){h.push(m);continue}s+v>i&&(d=Math.max(d,s),s=0,l+=u,u=p),m.draw(n,s,l),c[m.key]={x:s,y:l,width:m.width,height:m.height},s+=v,u=Math.max(u,p)}}catch(b){f.e(b)}finally{f.f()}return d=Math.max(d,s),{atlas:c,cursor:{x:s,y:l,rowHeight:u,maxRowWidth:d},remaining:h}}function _e(a,n,t,e){var r=Object.defineProperty;try{r({},"",{})}catch{r=0}_e=function(i,o,s,l){function u(d,c){_e(i,d,function(h){return this._invoke(d,c,h)})}o?r?r(i,o,{value:s,enumerable:!l,configurable:!l,writable:!l}):i[o]=s:(u("next",0),u("throw",1),u("return",2))},_e(a,n,t,e)}function Te(){var a,n,t=typeof Symbol=="function"?Symbol:{},e=t.iterator||"@@iterator",r=t.toStringTag||"@@toStringTag";function i(f,g,m,v){var p=g&&g.prototype instanceof s?g:s,b=Object.create(p.prototype);return _e(b,"_invoke",(function(y,T,x){var _,A,E,L=0,w=x||[],F=!1,D={p:0,n:0,v:a,a:I,f:I.bind(a,4),d:function(R,P){return _=R,A=0,E=a,D.n=P,o}};function I(R,P){for(A=R,E=P,n=0;!F&&L&&!G&&n<w.length;n++){var G,N=w[n],k=D.p,C=N[2];R>3?(G=C===P)&&(E=N[(A=N[4])?5:(A=3,3)],N[4]=N[5]=a):N[0]<=k&&((G=R<2&&k<N[1])?(A=0,D.v=P,D.n=N[1]):k<C&&(G=R<3||N[0]>P||P>C)&&(N[4]=R,N[5]=P,D.n=C,A=0))}if(G||R>1)return o;throw F=!0,P}return function(R,P,G){if(L>1)throw TypeError("Generator is already running");for(F&&P===1&&I(P,G),A=P,E=G;(n=A<2?a:E)||!F;){_||(A?A<3?(A>1&&(D.n=-1),I(A,E)):D.n=E:D.v=E);try{if(L=2,_){if(A||(R="next"),n=_[R]){if(!(n=n.call(_,E)))throw TypeError("iterator result is not an object");if(!n.done)return n;E=n.value,A<2&&(A=0)}else A===1&&(n=_.return)&&n.call(_),A<2&&(E=TypeError("The iterator does not provide a '"+R+"' method"),A=1);_=a}else if((n=(F=D.n<0)?E:y.call(T,D))!==o)break}catch(N){_=a,A=1,E=N}finally{L=1}}return{value:n,done:F}}})(f,m,v),!0),b}var o={};function s(){}function l(){}function u(){}n=Object.getPrototypeOf;var d=[][e]?n(n([][e]())):(_e(n={},e,function(){return this}),n),c=u.prototype=s.prototype=Object.create(d);function h(f){return Object.setPrototypeOf?Object.setPrototypeOf(f,u):(f.__proto__=u,_e(f,r,"GeneratorFunction")),f.prototype=Object.create(c),f}return l.prototype=u,_e(c,"constructor",u),_e(u,"constructor",l),l.displayName="GeneratorFunction",_e(u,r,"GeneratorFunction"),_e(c),_e(c,r,"Generator"),_e(c,e,function(){return this}),_e(c,"toString",function(){return"[object Generator]"}),(Te=function(){return{w:i,m:h}})()}function vo(a,n,t,e,r,i,o){try{var s=a[i](o),l=s.value}catch(u){return void t(u)}s.done?n(l):Promise.resolve(l).then(e,r)}function kt(a){return function(){var n=this,t=arguments;return new Promise(function(e,r){var i=a.apply(n,t);function o(l){vo(i,e,r,o,s,"next",l)}function s(l){vo(i,e,r,o,s,"throw",l)}o(void 0)})}}var qn=new Map,mo=!1;function cc(a){return new Promise(function(n,t){var e=new FileReader;e.onload=function(){return n(e.result)},e.onerror=t,e.readAsDataURL(a)})}function hc(a){if(!qn.has(a)){var n=fetch(a).then(function(t){return t.blob()}).then(cc).catch(function(){return qn.delete(a),null});qn.set(a,n)}return qn.get(a)}function fc(a){return $a.apply(this,arguments)}function $a(){return $a=kt(Te().m(function a(n){var t,e,r;return Te().w(function(i){for(;;)switch(i.n){case 0:return t=document.createElement("div"),t.innerHTML=n,e=Array.from(t.querySelectorAll("img[src]")),r=e.filter(function(o){return!o.getAttribute("src").startsWith("data:")}),i.n=1,Promise.all(r.map((function(){var o=kt(Te().m(function s(l){var u;return Te().w(function(d){for(;;)switch(d.n){case 0:return d.n=1,hc(l.getAttribute("src"));case 1:u=d.v,u&&l.setAttribute("src",u);case 2:return d.a(2)}},s)}));return function(s){return o.apply(this,arguments)}})()));case 1:return i.a(2,t.innerHTML)}},a)})),$a.apply(this,arguments)}function bo(a){return Va.apply(this,arguments)}function Va(){return Va=kt(Te().m(function a(n){var t,e,r,i,o,s,l,u,d,c,h,f,g,m=arguments;return Te().w(function(v){for(;;)switch(v.n){case 0:if(t=m.length>1&&m[1]!==void 0?m[1]:1,e=n instanceof SVGElement?new XMLSerializer().serializeToString(n):n,r=new DOMParser,i=r.parseFromString(e,"image/svg+xml"),o=i.querySelector("svg"),o){v.n=1;break}return v.a(2,null);case 1:if(s=parseFloat(o.getAttribute("width")||""),l=parseFloat(o.getAttribute("height")||""),(!(s>0)||!(l>0))&&(u=o.getAttribute("viewBox"),u&&(d=u.trim().split(/[\s,]+/),s=parseFloat(d[2]),l=parseFloat(d[3]))),!(!(s>0)||!(l>0))){v.n=2;break}return console.warn("Sigma: SVG label attachment has no parseable dimensions \u2014 skipped."),v.a(2,null);case 2:return c=Math.ceil(s*t),h=Math.ceil(l*t),f=new Blob([e],{type:"image/svg+xml"}),g=URL.createObjectURL(f),v.a(2,new Promise(function(p){var b=new Image;b.onload=function(){var y=document.createElement("canvas");y.width=c,y.height=h,y.getContext("2d").drawImage(b,0,0,c,h),URL.revokeObjectURL(g);try{y.getContext("2d").getImageData(0,0,1,1)}catch(T){if(T instanceof DOMException&&T.name==="SecurityError"){mo||(mo=!0,console.warn('Sigma: A label attachment was skipped because the rendered canvas is tainted. SVG with <foreignObject> (used by the "html" attachment type) is blocked in Chromium and Safari. Use type: "canvas" with Canvas 2D rendering instead.')),p(null);return}throw T}p(y)},b.onerror=function(){URL.revokeObjectURL(g),p(null)},b.src=g}))}},a)})),Va.apply(this,arguments)}function gc(a,n){var t=document.createElement("div");t.style.cssText="position:fixed;left:-99999px;top:0;visibility:hidden;width:max-content;height:max-content",t.innerHTML=(n?"<style>".concat(n,"</style>"):"")+a,document.body.appendChild(t);var e=t.getBoundingClientRect(),r=e.width,i=e.height;return document.body.removeChild(t),{width:Math.ceil(r),height:Math.ceil(i)}}function pc(a,n,t,e){return ja.apply(this,arguments)}function ja(){return ja=kt(Te().m(function a(n,t,e,r){var i,o,s,l,u,d,c,h,f=arguments;return Te().w(function(g){for(;;)switch(g.n){case 0:return i=f.length>4&&f[4]!==void 0?f[4]:1,o=n instanceof HTMLElement?n.outerHTML:n,g.n=1,fc(o);case 1:return s=g.v,(e==null||r==null)&&(l=gc(s,t),e=e??l.width,r=r??l.height),u=Math.ceil(e*i),d=Math.ceil(r*i),c=t?"<style>".concat(t,"</style>"):"",h='<svg xmlns="http://www.w3.org/2000/svg" '+'width="'.concat(u,'" height="').concat(d,'" viewBox="0 0 ').concat(e," ").concat(r,'">')+'<foreignObject width="'.concat(e,'" height="').concat(r,'">')+'<body xmlns="http://www.w3.org/1999/xhtml" style="margin:0;padding:0">'.concat(c).concat(s,"</body>")+"</foreignObject></svg>",g.a(2,bo(h,1))}},a)})),ja.apply(this,arguments)}function vc(a){return Xa.apply(this,arguments)}function Xa(){return Xa=kt(Te().m(function a(n){var t,e=arguments,r;return Te().w(function(i){for(;;)switch(i.n){case 0:t=e.length>1&&e[1]!==void 0?e[1]:1,r=n.type,i.n=r==="canvas"?1:r==="svg"?2:r==="html"?3:4;break;case 1:return i.a(2,n.canvas);case 2:return i.a(2,bo(n.svg,t));case 3:return i.a(2,pc(n.html,n.css,n.width,n.height,t));case 4:return i.a(2)}},a)})),Xa.apply(this,arguments)}var Ct=2048,yo=(function(){function a(n,t,e){J(this,a),S(this,"cache",new Map),S(this,"pending",new Set),S(this,"atlas",{}),S(this,"glTexture",null),S(this,"dirty",!0),this.gl=n,this.renderers=t,this.scheduleRender=e,this.packCanvas=document.createElement("canvas"),this.packCanvas.width=Ct,this.packCanvas.height=Ct,this.packCtx=this.packCanvas.getContext("2d")}return ee(a,[{key:"renderAttachment",value:function(t,e,r){var i=this,o="".concat(t,":").concat(e);if(!(this.cache.has(o)||this.pending.has(o))){var s=this.renderers[e];if(s){var l=s(r);if(l){this.pending.add(o);var u=r.pixelRatio;Promise.resolve(l).then((function(){var d=kt(Te().m(function c(h){var f;return Te().w(function(g){for(;;)switch(g.n){case 0:if(i.pending.has(o)){g.n=1;break}return g.a(2);case 1:if(i.pending.delete(o),h){g.n=2;break}return g.a(2);case 2:return g.n=3,vc(h,u);case 3:if(f=g.v,!(!f||f.width===0||f.height===0)){g.n=4;break}return g.a(2);case 4:i.cache.set(o,{image:f,width:f.width,height:f.height}),i.dirty=!0,i.scheduleRender();case 5:return g.a(2)}},c)}));return function(c){return d.apply(this,arguments)}})())}}}}},{key:"regenerateAtlas",value:function(){if(this.dirty){this.dirty=!1;var t=[];if(this.cache.forEach(function(u,d){t.push({key:d,width:u.width,height:u.height,draw:function(h,f,g){h.drawImage(u.image,f,g)}})}),t.length===0){this.atlas={},this.deleteGLTexture();return}var e={x:0,y:0,rowHeight:0,maxRowWidth:0};this.packCtx.clearRect(0,0,Ct,Ct);var r=po(t,this.packCtx,e),i=r.atlas,o=r.remaining;this.atlas=i,o.length>0&&console.warn("Sigma: ".concat(o.length," label attachment(s) could not fit in the ").concat(Ct,"x").concat(Ct," atlas and will not be rendered.")),this.deleteGLTexture();var s=this.gl,l=s.createTexture();s.activeTexture(s.TEXTURE0+tn),s.bindTexture(s.TEXTURE_2D,l),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!0),s.texImage2D(s.TEXTURE_2D,0,s.RGBA,s.RGBA,s.UNSIGNED_BYTE,this.packCanvas),s.pixelStorei(s.UNPACK_PREMULTIPLY_ALPHA_WEBGL,!1),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MIN_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_MAG_FILTER,s.LINEAR),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_S,s.CLAMP_TO_EDGE),s.texParameteri(s.TEXTURE_2D,s.TEXTURE_WRAP_T,s.CLAMP_TO_EDGE),this.glTexture=l}}},{key:"bindTexture",value:function(t){if(this.glTexture){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.glTexture)}}},{key:"getEntry",value:function(t,e){var r="".concat(t,":").concat(e);return this.atlas[r]||null}},{key:"invalidateNode",value:function(t){var e="".concat(t,":"),r=z(this.cache.keys()),i;try{for(r.s();!(i=r.n()).done;){var o=i.value;o.startsWith(e)&&(this.cache.delete(o),this.dirty=!0)}}catch(d){r.e(d)}finally{r.f()}var s=z(this.pending),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;u.startsWith(e)&&this.pending.delete(u)}}catch(d){s.e(d)}finally{s.f()}}},{key:"restore",value:function(){this.glTexture=null,this.dirty=!0}},{key:"clear",value:function(){this.cache.clear(),this.pending.clear(),this.atlas={},this.dirty=!0,this.deleteGLTexture()}},{key:"kill",value:function(){this.clear(),this.packCanvas=null,this.packCtx=null,this.gl=null}},{key:"deleteGLTexture",value:function(){this.glTexture&&(this.gl.deleteTexture(this.glTexture),this.glTexture=null)}}])})(),qa=(function(){function a(n){J(this,a),S(this,"buckets",new Map),S(this,"keyDepth",new Map);var t=z(n),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;this.buckets.set(r,new Set)}}catch(i){t.e(i)}finally{t.f()}}return ee(a,[{key:"has",value:function(t){return this.keyDepth.has(t)}},{key:"getBucket",value:function(t){return this.buckets.get(t)}},{key:"set",value:function(t,e){var r,i=this.keyDepth.get(t);if(i!==e){var o=this.buckets.get(e);if(!o)throw new Error('Sigma: "'.concat(e,'" is not a declared depth layer'));i!==void 0&&((r=this.buckets.get(i))===null||r===void 0||r.delete(t)),o.add(t),this.keyDepth.set(t,e)}}},{key:"remove",value:function(t){var e=this.keyDepth.get(t);e!==void 0&&(this.buckets.get(e).delete(t),this.keyDepth.delete(t))}},{key:"clearAll",value:function(){var t=z(this.buckets.values()),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;r.clear()}}catch(i){t.e(i)}finally{t.f()}this.keyDepth.clear()}},{key:"getSorted",value:function(t,e){var r=this.buckets.get(t);return!r||r.size===0?[]:V(r).sort(function(i,o){return e(i)-e(o)})}}])})(),xo=(function(a){function n(t,e){return J(this,n),le(this,n,[t,2,e])}return ue(n,a),ee(n,[{key:"updateNode",value:function(e,r,i,o,s,l,u,d){var c=this.indexMap.get(e);if(c===void 0)throw new Error('Node "'.concat(e,'" not allocated in NodeDataTexture'));var h=je(d),f=ae(h,4),g=f[0],m=f[1],v=f[2],p=f[3],b=c*2*4;this.data[b]=r,this.data[b+1]=i,this.data[b+2]=o,this.data[b+3]=s,this.data[b+4]=l,this.data[b+5]=u,this.data[b+6]=g*65536+m*256+v,this.data[b+7]=p/255,this.markDirty(c)}}])})(Hn),mc=1024,bc=1.5,Ka=(function(){function a(n,t){var e;if(J(this,a),S(this,"texture",null),S(this,"framebuffer",null),!n.getExtension("EXT_color_buffer_float"))throw new Error("sigma: EXT_color_buffer_float is required for label/edge placement but is unavailable in this WebGL2 context.");this.gl=n,this.maxTextureSize=n.getParameter(n.MAX_TEXTURE_SIZE),this.channels=t.channels,this.capacity=this.roundUpToPowerOfTwo((e=t.initialCapacity)!==null&&e!==void 0?e:mc);var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.create()}return ee(a,[{key:"roundUpToPowerOfTwo",value:function(t){return Math.pow(2,Math.ceil(Math.log2(Math.max(1,t))))}},{key:"computeDimensions",value:function(t){var e=Math.min(t,this.maxTextureSize),r=Math.ceil(t/e);return{width:e,height:r}}},{key:"create",value:function(){var t=this.gl,e=this.channels===1?t.R32F:t.RGBA32F,r=this.channels===1?t.RED:t.RGBA;if(this.texture=t.createTexture(),t.activeTexture(t.TEXTURE0),t.bindTexture(t.TEXTURE_2D,this.texture),t.texImage2D(t.TEXTURE_2D,0,e,this.textureWidth,this.textureHeight,0,r,t.FLOAT,null),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MIN_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_MAG_FILTER,t.NEAREST),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_S,t.CLAMP_TO_EDGE),t.texParameteri(t.TEXTURE_2D,t.TEXTURE_WRAP_T,t.CLAMP_TO_EDGE),this.framebuffer=t.createFramebuffer(),t.bindFramebuffer(t.FRAMEBUFFER,this.framebuffer),t.framebufferTexture2D(t.FRAMEBUFFER,t.COLOR_ATTACHMENT0,t.TEXTURE_2D,this.texture,0),t.checkFramebufferStatus(t.FRAMEBUFFER)!==t.FRAMEBUFFER_COMPLETE)throw new Error("sigma: float framebuffer for frame-pass placement is incomplete.");t.bindFramebuffer(t.FRAMEBUFFER,null),t.bindTexture(t.TEXTURE_2D,null)}},{key:"ensureCapacity",value:function(t){if(!(t<=this.capacity)){var e=this.gl;this.capacity=this.roundUpToPowerOfTwo(Math.ceil(t*bc));var r=this.computeDimensions(this.capacity);this.textureWidth=r.width,this.textureHeight=r.height,this.texture&&e.deleteTexture(this.texture),this.framebuffer&&e.deleteFramebuffer(this.framebuffer),this.create()}}},{key:"bindAsRenderTarget",value:function(){var t=this.gl;t.bindFramebuffer(t.FRAMEBUFFER,this.framebuffer),t.viewport(0,0,this.textureWidth,this.textureHeight)}},{key:"bind",value:function(t){var e=this.gl;e.activeTexture(e.TEXTURE0+t),e.bindTexture(e.TEXTURE_2D,this.texture)}},{key:"getTexture",value:function(){return this.texture}},{key:"getTextureWidth",value:function(){return this.textureWidth}},{key:"getTextureHeight",value:function(){return this.textureHeight}},{key:"restore",value:function(){this.gl.getExtension("EXT_color_buffer_float"),this.create()}},{key:"kill",value:function(){var t=this.gl;this.texture&&(t.deleteTexture(this.texture),this.texture=null),this.framebuffer&&(t.deleteFramebuffer(this.framebuffer),this.framebuffer=null)}}])})(),_o=(function(a){function n(t,e){return J(this,n),le(this,n,[t,2,e])}return ue(n,a),ee(n,[{key:"updateEdge",value:function(e,r,i,o,s,l){var u=arguments.length>6&&arguments[6]!==void 0?arguments[6]:0,d=arguments.length>7&&arguments[7]!==void 0?arguments[7]:0,c=arguments.length>8&&arguments[8]!==void 0?arguments[8]:0,h=this.indexMap.get(e);if(h===void 0)throw new Error('Edge "'.concat(e,'" not allocated in EdgeDataTexture'));var f=h*this.TEXELS_PER_ITEM*4;this.data[f+0]=r,this.data[f+1]=i,this.data[f+2]=o,this.data[f+3]=0,this.data[f+4]=s,this.data[f+5]=l,this.data[f+6]=u,this.data[f+7]=(d&15)<<4|c&15,this.markDirty(h)}}])})(Hn);var Lf=Je(ct()),Cf=Je(La());function yc(a,n){if(a==null)return{};var t={};for(var e in a)if({}.hasOwnProperty.call(a,e)){if(n.indexOf(e)!==-1)continue;t[e]=a[e]}return t}function To(a,n){if(a==null)return{};var t,e,r=yc(a,n);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(a);for(e=0;e<i.length;e++)t=i[e],n.indexOf(t)===-1&&{}.propertyIsEnumerable.call(a,t)&&(r[t]=a[t])}return r}var xc=["factor"],_c=["factor"],So=1.5,Ya={x:.5,y:.5,angle:0,ratio:1},Eo=(function(a){function n(){var t;J(this,n);for(var e=arguments.length,r=new Array(e),i=0;i<e;i++)r[i]=arguments[i];return t=le(this,n,[].concat(r)),S(t,"minRatio",null),S(t,"maxRatio",null),S(t,"enabled",!0),S(t,"enabledZooming",!0),S(t,"enabledPanning",!0),S(t,"enabledRotation",!0),S(t,"constrainState",null),S(t,"state",B({},Ya)),S(t,"previousState",B({},Ya)),S(t,"nextFrame",null),S(t,"currentAnimationFrom",null),S(t,"currentAnimationTo",null),S(t,"animationCallback",null),t}return ue(n,a),ee(n,[{key:"x",get:function(){return this.state.x}},{key:"y",get:function(){return this.state.y}},{key:"angle",get:function(){return this.state.angle}},{key:"ratio",get:function(){return this.state.ratio}},{key:"getState",value:function(){return B({},this.state)}},{key:"getPreviousState",value:function(){return B({},this.previousState)}},{key:"getBoundedRatio",value:function(e){var r=e;return typeof this.minRatio=="number"&&(r=Math.max(r,this.minRatio)),typeof this.maxRatio=="number"&&(r=Math.min(r,this.maxRatio)),r}},{key:"validateState",value:function(e){var r=this.getState();return this.enabledPanning&&typeof e.x=="number"&&(r.x=e.x),this.enabledPanning&&typeof e.y=="number"&&(r.y=e.y),this.enabledZooming&&typeof e.ratio=="number"&&(r.ratio=this.getBoundedRatio(e.ratio)),this.enabledRotation&&typeof e.angle=="number"&&(r.angle=e.angle),this.constrainState?this.constrainState(r):r}},{key:"isAnimating",value:function(){return this.nextFrame!==null}},{key:"setState",value:function(e){if(!this.enabled)return this;var r=this.validateState(e);return Di(this.state,r)?this:(this.previousState=this.state,this.state=r,this.emit("updated",this.getState()),this)}},{key:"updateState",value:function(e){return this.setState(e(this.getState())),this}},{key:"animate",value:function(e,r){var i=this;return this.enabled?new Promise(function(o){return i.runAnimation(e,B(B({},Ci),r),o)}):Promise.resolve()}},{key:"cancelAnimation",value:function(){return this.nextFrame!==null&&(cancelAnimationFrame(this.nextFrame),this.nextFrame=null),this.resolveAnimation(),this.emitAnimationEnd(!1),this}},{key:"zoomIn",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=e.factor,i=r===void 0?So:r,o=To(e,xc);return this.animate({ratio:this.ratio/i},o)}},{key:"zoomOut",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:{},r=e.factor,i=r===void 0?So:r,o=To(e,_c);return this.animate({ratio:this.ratio*i},o)}},{key:"reset",value:function(e){return this.animate(Ya,e)}},{key:"runAnimation",value:function(e,r,i){var o=this,s=wn(r.easing),l=Date.now(),u=this.getState(),d=this.validateState(e),c=function(){if(!o.enabled){o.cancelAnimation();return}var f=r.duration>0?(Date.now()-l)/r.duration:1;if(f>=1){o.nextFrame=null,o.setState(d),o.resolveAnimation(),o.emitAnimationEnd(!0);return}var g=s(f);o.setState({x:u.x+(d.x-u.x)*g,y:u.y+(d.y-u.y)*g,angle:u.angle+(d.angle-u.angle)*g,ratio:u.ratio+(d.ratio-u.ratio)*g}),o.nextFrame=requestAnimationFrame(c)};this.cancelAnimation(),this.currentAnimationFrom=u,this.currentAnimationTo=d,this.animationCallback=i,this.emit("animationStart",{from:u,to:d}),c()}},{key:"resolveAnimation",value:function(){var e=this.animationCallback;this.animationCallback=null,e&&e()}},{key:"emitAnimationEnd",value:function(e){var r=this.currentAnimationFrom,i=this.currentAnimationTo;!r||!i||(this.currentAnimationFrom=null,this.currentAnimationTo=null,this.emit("animationEnd",{from:r,to:i,completed:e}))}}],[{key:"from",value:function(e){var r=new n;return r.setState(e)}}])})(Xt);function Se(a,n){var t=n.getBoundingClientRect();return{x:a.clientX-t.left,y:a.clientY-t.top}}function Ie(a,n){var t=B(B({},Se(a,n)),{},{sigmaDefaultPrevented:!1,preventSigmaDefault:function(){t.sigmaDefaultPrevented=!0},original:a});return t}function ot(a){var n="x"in a?a:B(B({},a.touches[0]||a.previousTouches[0]),{},{original:a.original,sigmaDefaultPrevented:a.sigmaDefaultPrevented,preventSigmaDefault:function(){a.sigmaDefaultPrevented=!0,n.sigmaDefaultPrevented=!0}});return n}function Tc(a,n){var t=Ie(a,n);return t.delta=Wo(a),t}var Sc=2;function Zn(a){for(var n=[],t=0,e=Math.min(a.length,Sc);t<e;t++)n.push(a[t]);return n}function an(a,n,t){var e={touches:Zn(a.touches).map(function(r){return Se(r,t)}),previousTouches:n.map(function(r){return Se(r,t)}),sigmaDefaultPrevented:!1,preventSigmaDefault:function(){e.sigmaDefaultPrevented=!0},original:a};return e}function Wo(a){if(typeof a.deltaY<"u")return a.deltaY*-3/360;if(typeof a.detail<"u")return a.detail/-9;throw new Error("Captor: could not extract delta from event.")}var Uo=(function(a){function n(t,e){var r;return J(this,n),r=le(this,n),r.container=t,r.renderer=e,r}return ue(n,a),ee(n)})(Xt),Ec=["doubleClickTimeout","doubleClickZoomingDuration","doubleClickZoomingRatio","dragTimeout","draggedEventsTolerance","enableCameraMouseRotation","gestureTarget","inertiaDuration","inertiaRatio","zoomDuration","zoomingRatio"],Ac=Ec.reduce(function(a,n){return B(B({},a),{},S({},n,Fn[n]))},{});function rn(a){return a instanceof PointerEvent&&a.pointerType==="touch"}var wc=(function(a){function n(t,e){var r;return J(this,n),r=le(this,n,[t,e]),S(r,"enabled",!0),S(r,"draggedEvents",0),S(r,"downStartTime",null),S(r,"lastMouseX",null),S(r,"lastMouseY",null),S(r,"isMouseDown",!1),S(r,"isMoving",!1),S(r,"isPanningStage",!1),S(r,"movingTimeout",null),S(r,"startCameraState",null),S(r,"clicks",0),S(r,"doubleClickTimeout",null),S(r,"isRightMouseDown",!1),S(r,"startRotationAngle",null),S(r,"startCameraAngle",null),S(r,"currentWheelDirection",0),S(r,"lastWheelAnimationId",0),S(r,"settings",Ac),r.handleRightClick=r.handleRightClick.bind(r),r.handleDown=r.handleDown.bind(r),r.handleUp=r.handleUp.bind(r),r.handleMove=r.handleMove.bind(r),r.handleWheel=r.handleWheel.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleEnter=r.handleEnter.bind(r),t.addEventListener("contextmenu",r.handleRightClick,{capture:!1}),t.addEventListener("pointerdown",r.handleDown,{capture:!1}),t.addEventListener("wheel",r.handleWheel,{capture:!1,passive:!1}),t.addEventListener("pointerleave",r.handleLeave,{capture:!1}),t.addEventListener("pointerenter",r.handleEnter,{capture:!1}),document.addEventListener("pointermove",r.handleMove,{capture:!1}),document.addEventListener("pointerup",r.handleUp,{capture:!1}),r}return ue(n,a),ee(n,[{key:"kill",value:function(){var e=this.container;e.removeEventListener("contextmenu",this.handleRightClick),e.removeEventListener("pointerdown",this.handleDown),e.removeEventListener("wheel",this.handleWheel),e.removeEventListener("pointerleave",this.handleLeave),e.removeEventListener("pointerenter",this.handleEnter),document.removeEventListener("pointermove",this.handleMove),document.removeEventListener("pointerup",this.handleUp)}},{key:"handleClick",value:function(e){var r=this;if(this.enabled){if(this.clicks++,this.clicks===2)return this.clicks=0,typeof this.doubleClickTimeout=="number"&&(clearTimeout(this.doubleClickTimeout),this.doubleClickTimeout=null),this.handleDoubleClick(e);setTimeout(function(){r.clicks=0,r.doubleClickTimeout=null},this.settings.doubleClickTimeout),this.draggedEvents<this.settings.draggedEventsTolerance&&this.emit("click",Ie(e,this.container))}}},{key:"handleRightClick",value:function(e){this.enabled&&(this.settings.enableCameraMouseRotation&&e.preventDefault(),this.emit("rightClick",Ie(e,this.container)))}},{key:"handleDoubleClick",value:function(e){if(this.enabled){e.preventDefault(),e.stopPropagation();var r=Ie(e,this.container);if(this.emit("doubleClick",r),!r.sigmaDefaultPrevented){var i=this.renderer.getCamera(),o=i.getBoundedRatio(i.getState().ratio/this.settings.doubleClickZoomingRatio);i.animate(this.renderer.getViewportZoomedState(Se(e,this.container),o),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}}},{key:"handleDown",value:function(e){if(!(!this.enabled||rn(e))){if(e.button===0){this.startCameraState=this.renderer.getCamera().getState();var r=Se(e,this.container),i=r.x,o=r.y;this.lastMouseX=i,this.lastMouseY=o,this.draggedEvents=0,this.downStartTime=Date.now(),this.isMouseDown=!0}if(e.button===2&&this.settings.enableCameraMouseRotation){var s=Se(e,this.container),l=s.x,u=s.y,d=this.container.offsetWidth/2,c=this.container.offsetHeight/2;this.startRotationAngle=Math.atan2(u-c,l-d),this.startCameraAngle=this.renderer.getCamera().getState().angle,this.isRightMouseDown=!0}this.emit("mousedown",Ie(e,this.container))}}},{key:"handleUp",value:function(e){var r=this;if(!(!this.enabled||rn(e)||!this.isMouseDown&&!this.isRightMouseDown)){if(this.isRightMouseDown){this.isRightMouseDown=!1,this.startRotationAngle=null,this.startCameraAngle=null,this.emit("mouseup",Ie(e,this.container));return}var i=this.renderer.getCamera();this.isMouseDown=!1,this.isPanningStage&&(this.isPanningStage=!1,this.renderer._setPanning(!1)),typeof this.movingTimeout=="number"&&(clearTimeout(this.movingTimeout),this.movingTimeout=null);var o=Se(e,this.container),s=o.x,l=o.y,u=i.getState(),d=i.getPreviousState();this.isMoving?i.animate({x:u.x+this.settings.inertiaRatio*(u.x-d.x),y:u.y+this.settings.inertiaRatio*(u.y-d.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"}):(this.lastMouseX!==s||this.lastMouseY!==l)&&i.setState({x:u.x,y:u.y}),this.isMoving=!1,setTimeout(function(){var c=r.draggedEvents>0;r.draggedEvents=0;var h=r.renderer.getSetting("hideEdgesOnMove")||r.renderer.getSetting("hideLabelsOnMove");c&&h&&r.renderer.refresh()},0),this.emit("mouseup",Ie(e,this.container)),(e.target===this.container||e.composedPath()[0]===this.container)&&this.handleClick(e)}}},{key:"handleMove",value:function(e){var r=this;if(!(!this.enabled||rn(e))){var i=Ie(e,this.container);if(this.emit("mousemovebody",i),(e.target===this.container||e.composedPath()[0]===this.container)&&this.emit("mousemove",i),this.isMouseDown&&this.draggedEvents++,!i.sigmaDefaultPrevented){if(this.isMouseDown){this.isMoving=!0,typeof this.movingTimeout=="number"&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.movingTimeout=null,r.isMoving=!1},this.settings.dragTimeout);var o=this.renderer.getCamera(),s=Se(e,this.container),l=s.x,u=s.y,d=this.renderer.viewportToFramedGraph({x:this.lastMouseX,y:this.lastMouseY}),c=this.renderer.viewportToFramedGraph({x:l,y:u}),h=d.x-c.x,f=d.y-c.y,g=o.getState(),m=g.x+h,v=g.y+f;this.isPanningStage||(this.isPanningStage=!0,this.renderer._setPanning(!0)),o.setState({x:m,y:v}),this.lastMouseX=l,this.lastMouseY=u,e.preventDefault(),e.stopPropagation()}if(this.isRightMouseDown){var p=Se(e,this.container),b=p.x,y=p.y,T=this.container.offsetWidth/2,x=this.container.offsetHeight/2,_=Math.atan2(y-x,b-T),A=_-this.startRotationAngle,E=this.renderer.getCamera();E.setState({angle:this.startCameraAngle+A}),e.preventDefault(),e.stopPropagation()}}}}},{key:"handleLeave",value:function(e){rn(e)||this.emit("mouseleave",Ie(e,this.container))}},{key:"handleEnter",value:function(e){rn(e)||this.emit("mouseenter",Ie(e,this.container))}},{key:"handleWheel",value:function(e){var r=this,i=this.renderer.getCamera();if(!(!this.enabled||!i.enabledZooming)){var o=Tc(e,this.container);if(this.emit("wheel",o),!o.sigmaDefaultPrevented){var s=this.settings.gestureTarget;if(s!=="page"){if(s==="shared"&&!e.ctrlKey&&!e.metaKey){this.renderer._showGestureHint("wheel");return}var l=Wo(e);if(l){e.preventDefault(),e.stopPropagation();var u=i.getState().ratio,d=l>0?1/this.settings.zoomingRatio:this.settings.zoomingRatio,c=i.getBoundedRatio(u*d),h=l>0?1:-1,f=Date.now();if(u!==c&&!(this.currentWheelDirection===h&&this.lastWheelTriggerTime&&f-this.lastWheelTriggerTime<this.settings.zoomDuration/5)){var g=++this.lastWheelAnimationId;i.animate(this.renderer.getViewportZoomedState(Se(e,this.container),c),{easing:"quadraticOut",duration:this.settings.zoomDuration}).then(function(){r.lastWheelAnimationId===g&&(r.currentWheelDirection=0)}),this.currentWheelDirection=h,this.lastWheelTriggerTime=f}}}}}}},{key:"setSettings",value:function(e){this.settings=e}}])})(Uo),Dc=["dragTimeout","gestureTarget","inertiaDuration","inertiaRatio","doubleClickTimeout","doubleClickZoomingRatio","doubleClickZoomingDuration","tapMoveTolerance"],Rc=Dc.reduce(function(a,n){return B(B({},a),{},S({},n,Fn[n]))},{}),Lc=(function(a){function n(t,e){var r;return J(this,n),r=le(this,n,[t,e]),S(r,"enabled",!0),S(r,"isMoving",!1),S(r,"hasMoved",!1),S(r,"isPanningStage",!1),S(r,"isZoomingStage",!1),S(r,"touchMode",0),S(r,"startTouchesPositions",[]),S(r,"lastTouches",[]),S(r,"lastTap",null),S(r,"settings",Rc),r.handleStart=r.handleStart.bind(r),r.handleLeave=r.handleLeave.bind(r),r.handleMove=r.handleMove.bind(r),t.addEventListener("touchstart",r.handleStart,{capture:!1}),t.addEventListener("touchcancel",r.handleLeave,{capture:!1}),document.addEventListener("touchend",r.handleLeave,{capture:!1,passive:!1}),document.addEventListener("touchmove",r.handleMove,{capture:!1,passive:!1}),r}return ue(n,a),ee(n,[{key:"kill",value:function(){var e=this.container;e.removeEventListener("touchstart",this.handleStart),e.removeEventListener("touchcancel",this.handleLeave),document.removeEventListener("touchend",this.handleLeave),document.removeEventListener("touchmove",this.handleMove)}},{key:"getDimensions",value:function(){return{width:this.container.offsetWidth,height:this.container.offsetHeight}}},{key:"doesCapture",value:function(e){var r=this.settings.gestureTarget;return r==="graph"?!0:r==="page"?!1:e>=2}},{key:"syncStageFlags",value:function(){var e=this.touchMode===1&&this.hasMoved,r=this.touchMode===2;e!==this.isPanningStage&&(this.isPanningStage=e,this.renderer._setPanning(e)),r!==this.isZoomingStage&&(this.isZoomingStage=r,this.renderer._setZooming(r))}},{key:"handleStart",value:function(e){var r=this;if(this.enabled){var i=Zn(e.touches);if(this.touchMode=i.length,this.startCameraState=this.renderer.getCamera().getState(),this.startTouchesPositions=i.map(function(f){return Se(f,r.container)}),this.touchMode===2){var o=ae(this.startTouchesPositions,2),s=o[0],l=s.x,u=s.y,d=o[1],c=d.x,h=d.y;this.startTouchesAngle=Math.atan2(h-u,c-l),this.startTouchesDistance=Math.sqrt(Math.pow(c-l,2)+Math.pow(h-u,2))}this.syncStageFlags(),this.emit("touchdown",an(e,this.lastTouches,this.container)),this.lastTouches=i,this.lastTouchesPositions=this.startTouchesPositions,e.cancelable&&(this.doesCapture(e.touches.length)||this.renderer._hasNodeDrag())&&e.preventDefault()}}},{key:"handleLeave",value:function(e){if(!(!this.enabled||!this.startTouchesPositions.length)){switch(e.cancelable&&this.doesCapture(this.touchMode)&&e.preventDefault(),this.movingTimeout&&(this.isMoving=!1,clearTimeout(this.movingTimeout)),this.touchMode){case 2:if(e.touches.length===1){this.handleStart(e);break}case 1:if(this.isMoving){var r=this.renderer.getCamera(),i=r.getState(),o=r.getPreviousState();r.animate({x:i.x+this.settings.inertiaRatio*(i.x-o.x),y:i.y+this.settings.inertiaRatio*(i.y-o.y)},{duration:this.settings.inertiaDuration,easing:"quadraticOut"})}this.hasMoved=!1,this.isMoving=!1,this.touchMode=0;break}if(this.syncStageFlags(),this.emit("touchup",an(e,this.lastTouches,this.container)),!e.touches.length){var s=Se(this.lastTouches[0],this.container),l=this.startTouchesPositions[0],u=Math.pow(s.x-l.x,2)+Math.pow(s.y-l.y,2);if(!e.touches.length&&u<Math.pow(this.settings.tapMoveTolerance,2))if(this.lastTap&&Date.now()-this.lastTap.time<this.settings.doubleClickTimeout){var d=an(e,this.lastTouches,this.container);if(this.emit("doubletap",d),this.lastTap=null,!d.sigmaDefaultPrevented&&this.settings.gestureTarget!=="page"){var c=this.renderer.getCamera(),h=c.getBoundedRatio(c.getState().ratio/this.settings.doubleClickZoomingRatio);c.animate(this.renderer.getViewportZoomedState(s,h),{easing:"quadraticInOut",duration:this.settings.doubleClickZoomingDuration})}}else{var f=an(e,this.lastTouches,this.container);this.emit("tap",f),this.lastTap={time:Date.now(),position:f.touches[0]||f.previousTouches[0]}}}this.lastTouches=Zn(e.touches),this.startTouchesPositions=[]}}},{key:"handleMove",value:function(e){var r=this;if(!(!this.enabled||!this.startTouchesPositions.length)){var i=this.doesCapture(e.touches.length);i&&e.preventDefault();var o=Zn(e.touches),s=o.map(function(H){return Se(H,r.container)}),l=this.lastTouches;this.lastTouches=o,this.lastTouchesPositions=s;var u=an(e,l,this.container);if(this.emit("touchmove",u),!u.sigmaDefaultPrevented&&(this.hasMoved||(this.hasMoved=s.some(function(H,Y){var j=r.startTouchesPositions[Y];return j&&(H.x!==j.x||H.y!==j.y)})),!!this.hasMoved)){if(!i){this.settings.gestureTarget==="shared"&&this.renderer._showGestureHint("touch");return}this.isMoving=!0,this.syncStageFlags(),this.movingTimeout&&clearTimeout(this.movingTimeout),this.movingTimeout=window.setTimeout(function(){r.isMoving=!1},this.settings.dragTimeout);var d=this.renderer.getCamera(),c=this.startCameraState,h=this.renderer.getSetting("stagePadding");switch(this.touchMode){case 1:{var f=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0]),g=f.x,m=f.y,v=this.renderer.viewportToFramedGraph(s[0]),p=v.x,b=v.y;d.setState({x:c.x+g-p,y:c.y+m-b});break}case 2:{var y={x:.5,y:.5,angle:0,ratio:1},T=s[0],x=T.x,_=T.y,A=s[1],E=A.x,L=A.y,w=Math.atan2(L-_,E-x)-this.startTouchesAngle,F=Math.hypot(L-_,E-x)/this.startTouchesDistance,D=d.getBoundedRatio(c.ratio/F);y.ratio=D,y.angle=c.angle+w;var I=this.getDimensions(),R=this.renderer.viewportToFramedGraph((this.startTouchesPositions||[])[0],{cameraState:c}),P=Math.min(I.width,I.height)-2*h,G=P/I.width,N=P/I.height,k=D/P,C=x-P/2/G,M=_-P/2/N,$=ha({x:C,y:M},y.angle);C=$.x,M=$.y,y.x=R.x-C*k,y.y=R.y+M*k,d.setState(y);break}}}}}},{key:"setSettings",value:function(e){this.settings=e}}])})(Uo),Cc=(function(){function a(n,t,e,r){J(this,a),S(this,"pendingNode",null),S(this,"session",null),this.graph=n,this.viewportToGraph=t,this.setNodesState=e,this.emit=r}return ee(a,[{key:"start",value:function(t,e,r,i,o){var s=r(t),l=!1;if(this.emit("nodeDragStart",{node:t,allDraggedNodes:s,event:e,preventSigmaDefault:function(){l=!0}}),l)return!1;var u=new Map,d=z(s),c;try{for(d.s();!(c=d.n()).done;){var h=c.value;u.set(h,{x:this.graph.getNodeAttribute(h,i),y:this.graph.getNodeAttribute(h,o)})}}catch(f){d.e(f)}finally{d.f()}return this.session={node:t,allNodes:s,startPosition:this.viewportToGraph(e),startNodePositions:u,xAttr:i,yAttr:o},this.setNodesState(s,{isDragged:!0}),!0}},{key:"applyMove",value:function(t,e){var r=this.session,i=r.allNodes,o=r.startNodePositions,s=r.startPosition,l=r.xAttr,u=r.yAttr,d=this.viewportToGraph(t),c={x:d.x-s.x,y:d.y-s.y},h=z(i),f;try{for(h.s();!(f=h.n()).done;){var g=f.value,m=o.get(g);if(!(!m||!this.graph.hasNode(g))){var v={x:m.x+c.x,y:m.y+c.y};e?this.graph.mergeNodeAttributes(g,e(v,g)):(this.graph.setNodeAttribute(g,l,v.x),this.graph.setNodeAttribute(g,u,v.y))}}}catch(p){h.e(p)}finally{h.f()}}},{key:"end",value:function(){if(this.pendingNode=null,!this.session)return null;var t=this.session,e=t.node,r=t.allNodes;return this.setNodesState(r,{isDragged:!1}),this.session=null,{node:e,allNodes:r}}},{key:"removeNode",value:function(t){t===this.pendingNode&&(this.pendingNode=null),this.session&&(this.session.node===t?(this.setNodesState(this.session.allNodes,{isDragged:!1}),this.session=null):this.session.allNodes.includes(t)&&(this.session.allNodes=this.session.allNodes.filter(function(e){return e!==t}),this.session.startNodePositions.delete(t)))}},{key:"clear",value:function(){this.pendingNode=null,this.session=null}}])})();function Ao(a,n){return a<n?"".concat(a,"\0").concat(n):"".concat(n,"\0").concat(a)}var kc=(function(){function a(n,t){J(this,a),S(this,"groups",new Map),S(this,"edgeToGroupKey",new Map),this.graph=n,this.onGroupChanged=t}return ee(a,[{key:"getGroupKey",value:function(t){return Ao(this.graph.source(t),this.graph.target(t))}},{key:"sortGroup",value:function(t,e){var r=this,i=e.split("\0")[0];t.sort(function(o,s){var l=r.graph.source(o)===i||!r.graph.isDirected(o)?0:1,u=r.graph.source(s)===i||!r.graph.isDirected(s)?0:1;return l-u})}},{key:"register",value:function(t){var e=this.getGroupKey(t),r=this.groups.get(e);r||(r=[],this.groups.set(e,r)),r.includes(t)||r.push(t),this.edgeToGroupKey.set(t,e),r.length>1&&this.sortGroup(r,e),this.onGroupChanged(r,r.length)}},{key:"unregister",value:function(t){var e=this.edgeToGroupKey.get(t);if(e){this.edgeToGroupKey.delete(t);var r=this.groups.get(e);if(r){var i=r.indexOf(t);i!==-1&&r.splice(i,1),r.length===0?this.groups.delete(e):this.onGroupChanged(r,r.length)}}}},{key:"getGroup",value:function(t){var e,r=this.edgeToGroupKey.get(t);return r?(e=this.groups.get(r))!==null&&e!==void 0?e:[]:[]}},{key:"getSiblings",value:function(t){return this.getGroup(t).filter(function(e){return e!==t})}},{key:"rebuild",value:function(){var t=this;this.groups.clear(),this.edgeToGroupKey.clear(),this.graph.forEachEdge(function(l,u,d,c){var h=Ao(d,c),f=t.groups.get(h);f||(f=[],t.groups.set(h,f)),f.push(l),t.edgeToGroupKey.set(l,h)});var e=z(this.groups),r;try{for(e.s();!(r=e.n()).done;){var i=ae(r.value,2),o=i[0],s=i[1];s.length>1&&this.sortGroup(s,o),this.onGroupChanged(s,s.length)}}catch(l){e.e(l)}finally{e.f()}}},{key:"clear",value:function(){this.groups.clear(),this.edgeToGroupKey.clear()}}])})();function wo(a,n){var t;if(a===!1||a==="extend"||a==="separate")return a;var e=a[n];return e!==void 0?e:(t=a.default)!==null&&t!==void 0?t:!1}function Do(a){return a===!1?!1:a==="extend"||a==="separate"?!0:Object.values(a).some(function(n){return n==="extend"||n==="separate"})}function Ro(a){return a==="separate"?!0:a===!1||a==="extend"?!1:a.default==="separate"?!0:Object.values(a).includes("separate")}var me={node:{parent:null,eventSuffix:"Node",payloadKey:"node",isEnabled:function(){return!0},writesPickingThisFrame:function(){return!0},setHover:function(n,t,e){return n.setNodeState(t,{isHovered:e})},getCursor:function(n,t){var e;return(e=n.nodeDataCache[t])===null||e===void 0?void 0:e.cursor},isHitValid:function(n,t){var e;return((e=n.nodeDataCache[t])===null||e===void 0?void 0:e.visibility)!=="hidden"}},edge:{parent:null,eventSuffix:"Edge",payloadKey:"edge",isEnabled:function(n){return!!n.settings.enableEdgeEvents},writesPickingThisFrame:function(n){return!!n.settings.enableEdgeEvents},setHover:function(n,t,e){return n.setEdgeState(t,{isHovered:e})},getCursor:function(n,t){var e;return(e=n.edgeDataCache[t])===null||e===void 0?void 0:e.cursor}},nodeLabel:{parent:"node",eventSuffix:"NodeLabel",payloadKey:"node",isEnabled:function(n){return Ro(n.settings.nodeLabelEvents)},writesPickingThisFrame:function(n){return Do(n.settings.nodeLabelEvents)},setHover:function(n,t,e){return n.setNodeState(t,{isLabelHovered:e})},getCursor:function(n,t){var e;return(e=n.nodeDataCache[t])===null||e===void 0?void 0:e.labelCursor},resolveForVerb:function(n,t,e){var r=wo(e.settings.nodeLabelEvents,t);return r===!1?null:r==="extend"?{kind:"node",key:n.key}:n}},edgeLabel:{parent:"edge",eventSuffix:"EdgeLabel",payloadKey:"edge",isEnabled:function(n){return Ro(n.settings.edgeLabelEvents)},writesPickingThisFrame:function(n){return Do(n.settings.edgeLabelEvents)},setHover:function(n,t,e){return n.setEdgeState(t,{isLabelHovered:e})},getCursor:function(n,t){var e;return(e=n.edgeDataCache[t])===null||e===void 0?void 0:e.labelCursor},resolveForVerb:function(n,t,e){var r=wo(e.settings.edgeLabelEvents,t);return r===!1?null:r==="extend"?{kind:"edge",key:n.key}:n}}},ut=["node","edge","nodeLabel","edgeLabel"];function Pc(){return{lookup:[null],idsByKind:{node:new Map,edge:new Map,nodeLabel:new Map,edgeLabel:new Map},nextId:1}}function lt(a,n,t){var e;return(e=a.idsByKind[n].get(t))!==null&&e!==void 0?e:0}function Kn(a,n){var t=z(ut),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;if(r===n||me[r].parent===n){var i=z(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(h){i.e(h)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(h){t.e(h)}finally{t.f()}var l=1,u=z(ut),d;try{for(u.s();!(d=u.n()).done;){var c=d.value;if(c===n)break;l+=a.idsByKind[c].size}}catch(h){u.e(h)}finally{u.f()}a.nextId=l}function Lo(a,n,t){var e=a.nextId++;return a.idsByKind[n].set(t,e),a.lookup[e]={kind:n,key:t},e}function Co(a,n,t){for(var e=0,r=Qn(n);e<r.length;e++){var i=r[e],o=a.idsByKind[i].get(t);o!==void 0&&(a.lookup[o]=null,a.idsByKind[i].delete(t))}}function Fc(a,n){var t=z(ut),e;try{for(t.s();!(e=t.n()).done;){var r=e.value;if(me[r].parent!==null){var i=z(a.idsByKind[r].values()),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;a.lookup[s]=null}}catch(x){i.e(x)}finally{i.f()}a.idsByKind[r]=new Map}}}catch(x){t.e(x)}finally{t.f()}a.nextId=1;var l=z(ut),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;if(me[d].parent===null){var c=z(a.idsByKind[d].values()),h;try{for(c.s();!(h=c.n()).done;){var f=h.value;f>=a.nextId&&(a.nextId=f+1)}}catch(x){c.e(x)}finally{c.f()}}}}catch(x){l.e(x)}finally{l.f()}var g=z(ut),m;try{for(g.s();!(m=g.n()).done;){var v=m.value,p=me[v];if(!(p.parent===null||!p.isEnabled(n))){var b=z(a.idsByKind[p.parent].keys()),y;try{for(b.s();!(y=b.n()).done;){var T=y.value;a.idsByKind[v].set(T,a.nextId),a.lookup[a.nextId]={kind:v,key:T},a.nextId++}}catch(x){b.e(x)}finally{b.f()}}}}catch(x){g.e(x)}finally{g.f()}}function Ic(a,n){var t=z(ut),e;try{for(t.s();!(e=t.n()).done;){var r=e.value,i=a[r],o=n[r];if(i!==o){if(i.size!==o.size)return!1;var s=z(i),l;try{for(s.s();!(l=s.n()).done;){var u=ae(l.value,2),d=u[0],c=u[1];if(o.get(d)!==c)return!1}}catch(h){s.e(h)}finally{s.f()}}}}catch(h){t.e(h)}finally{t.f()}return!0}function ko(a,n,t){me[n.kind].setHover(a,n.key,t)}function Nc(a,n){return me[n.kind].getCursor(a,n.key)}function on(a,n){return"".concat(n).concat(me[a].eventSuffix)}function sn(a,n){return B(B({},n),{},S({},me[a.kind].payloadKey,a.key))}function Gc(a,n,t){var e=me[n.kind].resolveForVerb;return e?e(n,t,a):n}function Mc(a,n){var t=me[n.kind].isHitValid;return t?t(a,n.key):!0}function Qn(a){return ut.filter(function(n){return n===a||me[n].parent===a})}function Oo(a,n,t){return n&&(n=Gc(a,n,t)),n&&!Mc(a,n)&&(n=null),n}function Za(a,n,t){var e=t==="wheel"?a.stateManager.hovered:a.getHitAtPosition(n);return Oo(a,e,t)}function zc(a,n){return!a||!n?a===n:a.kind===n.kind&&a.key===n.key}function Ho(a,n,t){var e={event:t,preventSigmaDefault:function(){return t.preventSigmaDefault()}},r=a.dragManager.session,i=r?{kind:"node",key:r.node}:Oo(a,n,"enter"),o=a.stateManager,s=o.hovered;zc(s,i)||(s&&(ko(a,s,!1),a.emit(on(s.kind,"leave"),sn(s,e))),o.setHovered(i),i&&(ko(a,i,!0),a.emit(on(i.kind,"enter"),sn(i,e))),a.updateContainerCursor())}function Bc(a,n,t,e){e.handleResize=function(){return a.scheduleRefresh()},window.addEventListener("resize",e.handleResize),e.handleMove=function(i){a.hoverResolver.pointerMoved(ot(i))},e.handleMoveBody=function(i){var o=ot(i),s=a.dragManager;if(s.pendingNode&&!s.session){var l=a.nodeStyleAnalysis,u=l.xAttribute,d=l.yAttribute,c=a.settings;s.start(s.pendingNode,o,c.getDraggedNodes,u||"x",d||"y"),s.pendingNode=null}s.session&&(s.applyMove(o,a.settings.dragPositionToAttributes),a.emit("nodeDrag",{node:s.session.node,allDraggedNodes:s.session.allNodes,event:o}),o.preventSigmaDefault()),a.emit("moveBody",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})},e.handleLeave=function(i){var o=ot(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}};a.hoverResolver.pointerLeft(),Ho(a,null,o),a.emit("leaveStage",s)},e.handleEnter=function(i){var o=ot(i);a.emit("enterStage",{event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}})};var r=function(o){return function(s){var l=ot(s),u={event:l,preventSigmaDefault:function(){return l.preventSigmaDefault()}},d=Za(a,l,o);if(d){a.emit(on(d.kind,o),sn(d,u));return}a.emit("".concat(o,"Stage"),u)}};e.handleClick=r("click"),e.handleRightClick=r("rightClick"),e.handleDoubleClick=r("doubleClick"),e.handleWheel=r("wheel"),e.handleDown=function(i){var o=ot(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=Za(a,o,"down");if(l){l.kind==="node"&&a.settings.enableNodeDrag&&(a.dragManager.pendingNode=l.key),a.emit(on(l.kind,"down"),sn(l,s));return}a.emit("downStage",s)},e.handleUp=function(i){var o=ot(i),s={event:o,preventSigmaDefault:function(){return o.preventSigmaDefault()}},l=a.dragManager.end();l&&a.emit("nodeDragEnd",B({node:l.node,allDraggedNodes:l.allNodes},s));var u=Za(a,o,"up");if(u){a.emit(on(u.kind,"up"),sn(u,s));return}a.emit("upStage",s)},n.on("mousemove",e.handleMove),n.on("mousemovebody",e.handleMoveBody),n.on("click",e.handleClick),n.on("rightClick",e.handleRightClick),n.on("doubleClick",e.handleDoubleClick),n.on("wheel",e.handleWheel),n.on("mousedown",e.handleDown),n.on("mouseup",e.handleUp),n.on("mouseleave",e.handleLeave),n.on("mouseenter",e.handleEnter),t.on("touchdown",e.handleDown),t.on("touchdown",e.handleMove),t.on("touchup",e.handleUp),t.on("touchmove",e.handleMove),t.on("tap",e.handleClick),t.on("doubletap",e.handleDoubleClick),t.on("touchmove",e.handleMoveBody)}function Wc(a,n){var t=a.graph,e=new Set(["x","y","zIndex","type"]);n.eachNodeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=!o||o.some(function(l){return e.has(l)});a.refresh({partialGraph:{nodes:t.nodes()},skipIndexation:!s,schedule:!0})},n.eachEdgeAttributesUpdatedGraphUpdate=function(r){var i,o=(i=r.hints)===null||i===void 0?void 0:i.attributes,s=o&&["zIndex","type"].some(function(l){return o?.includes(l)});a.refresh({partialGraph:{edges:t.edges()},skipIndexation:!s,schedule:!0})},n.addNodeGraphUpdate=function(r){a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.updateNodeGraphUpdate=function(r){a.refresh({partialGraph:{nodes:[r.key]},skipIndexation:!1,schedule:!0})},n.dropNodeGraphUpdate=function(r){a.removeNode(r.key),a.scheduleProcess()},n.addEdgeGraphUpdate=function(r){var i=r.key;a.markNodesDirty([r.source,r.target]),a.edgeGroups.register(i);var o=a.edgeGroups.getSiblings(i);a.refresh({partialGraph:{edges:[i].concat(V(o))},schedule:!0})},n.updateEdgeGraphUpdate=function(r){a.refresh({partialGraph:{edges:[r.key]},skipIndexation:!1,schedule:!0})},n.dropEdgeGraphUpdate=function(r){var i=r.key;a.markNodesDirty([r.source,r.target]);var o=a.edgeGroups.getSiblings(i);a.edgeGroups.unregister(i),a.removeEdge(i);var s=z(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;a.addEdge(u)}}catch(d){s.e(d)}finally{s.f()}a.scheduleProcess()},n.clearEdgesGraphUpdate=function(){a.clearEdgeState(),a.clearEdgeIndices(),a.refresh({schedule:!0})},n.clearGraphUpdate=function(){a.clearEdgeState(),a.clearNodeState(),a.clearEdgeIndices(),a.clearNodeIndices(),a.refresh({schedule:!0})},t.on("nodeAdded",n.addNodeGraphUpdate),t.on("nodeDropped",n.dropNodeGraphUpdate),t.on("nodeAttributesUpdated",n.updateNodeGraphUpdate),t.on("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),t.on("edgeAdded",n.addEdgeGraphUpdate),t.on("edgeDropped",n.dropEdgeGraphUpdate),t.on("edgeAttributesUpdated",n.updateEdgeGraphUpdate),t.on("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),t.on("edgesCleared",n.clearEdgesGraphUpdate),t.on("cleared",n.clearGraphUpdate)}function Uc(a,n){a.removeListener("nodeAdded",n.addNodeGraphUpdate),a.removeListener("nodeDropped",n.dropNodeGraphUpdate),a.removeListener("nodeAttributesUpdated",n.updateNodeGraphUpdate),a.removeListener("eachNodeAttributesUpdated",n.eachNodeAttributesUpdatedGraphUpdate),a.removeListener("edgeAdded",n.addEdgeGraphUpdate),a.removeListener("edgeDropped",n.dropEdgeGraphUpdate),a.removeListener("edgeAttributesUpdated",n.updateEdgeGraphUpdate),a.removeListener("eachEdgeAttributesUpdated",n.eachEdgeAttributesUpdatedGraphUpdate),a.removeListener("edgesCleared",n.clearEdgesGraphUpdate),a.removeListener("cleared",n.clearGraphUpdate)}var Oc={x:.5,y:.5,angle:0,ratio:1},Hc=8,$c=.001;function Vc(a){var n=a.coords,t=a.nodeData,e=a.dimensions,r=a.stagePadding,i=a.zoomToSizeRatioFunction,o=a.itemSizesReference,s=a.fitLabels,l=a.nodeLabelBox,u=Object.keys(n);if(!u.length)return a.extent;for(var d=i(1)||1,c=o==="positions",h=e.width,f=e.height,g=a.extent,m=0;m<Hc;m++){for(var v=Pn(g),p={width:g.x[1]-g.x[0]||1,height:g.y[1]-g.y[0]||1},b=Ve(Oc,e,p,r),y=Po(b,v({x:0,y:0}),h,f),T=Po(b,v({x:1,y:0}),h,f),x=Math.hypot(T.x-y.x,T.y-y.y)||1,_=1/0,A=-1/0,E=1/0,L=-1/0,w=0,F=u.length;w<F;w++){var D=u[w],I=t[D],R=I.size/d*(c?x:1),P=-R,G=R,N=-R,k=R;if(s){var C=l(I,R);C&&(C.minX<P&&(P=C.minX),C.maxX>G&&(G=C.maxX),C.minY<N&&(N=C.minY),C.maxY>k&&(k=C.maxY))}var M=n[D],$=M.x,H=M.y;_=Math.min(_,$+P/x),A=Math.max(A,$+G/x),E=Math.min(E,H-k/x),L=Math.max(L,H-N/x)}var Y={x:[_,A],y:[E,L]},j=Math.max(Y.x[1]-Y.x[0],Y.y[1]-Y.y[0])||1,q=Math.max(Math.abs(Y.x[0]-g.x[0]),Math.abs(Y.x[1]-g.x[1]),Math.abs(Y.y[0]-g.y[0]),Math.abs(Y.y[1]-g.y[1]));if(g=Y,q/j<$c)break}return g}function Po(a,n,t,e){var r=bt(a,n);return{x:(1+r.x)*t/2,y:(1-r.y)*e/2}}var jc=typeof navigator<"u"&&/Mac|iP(hone|ad|od)/.test(navigator.platform),Xc=1500,qc=`.sigma-gesture-hint {
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
}`;function Fo(a){var n,t,e=Et("style");e.textContent=a,document.head.appendChild(e);var r=((n=(t=e.sheet)===null||t===void 0?void 0:t.cssRules.length)!==null&&n!==void 0?n:0)>0;return e.remove(),r}var st=null;function Kc(){return st===null&&(st=qc,Fo("@scope {}")&&(st=`@scope {
`.concat(st,`
}`)),Fo("@layer sigma-gesture-hint {}")&&(st=`@layer sigma-gesture-hint {
`.concat(st,`
}`))),st}var Yc=(function(){function a(n){J(this,a),S(this,"hideTimeout",null),this.styleElement=Et("style"),this.styleElement.textContent=Kc(),n.appendChild(this.styleElement),this.element=Et("div",{opacity:"0"},{class:"sigma-gesture-hint"}),n.appendChild(this.element)}return ee(a,[{key:"show",value:function(t,e){var r=this,i=t==="touch"?e.sharedGestureTouchMessage:jc?e.sharedGestureAppleWheelMessage:e.sharedGestureWheelMessage;i&&(this.element.textContent=i,this.element.style.opacity="1",typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.hideTimeout=window.setTimeout(function(){r.hideTimeout=null,r.element.style.opacity="0"},Xc))}},{key:"kill",value:function(){typeof this.hideTimeout=="number"&&clearTimeout(this.hideTimeout),this.styleElement.remove(),this.element.remove()}}])})(),Zc=(function(){function a(n){J(this,a),S(this,"readBuffer",null),S(this,"fence",null),S(this,"result",new Uint8Array(4)),S(this,"generation",0),S(this,"readGeneration",0),S(this,"lastEvent",null),S(this,"active",!1),S(this,"dirty",!1),S(this,"rafId",null),S(this,"killed",!1),this.options=n}return ee(a,[{key:"pointerMoved",value:function(t){this.lastEvent=t,this.active=!0,!this.options.isRenderPending()&&this.request()}},{key:"pointerLeft",value:function(){this.active=!1,this.dirty=!1}},{key:"frameRendered",value:function(){this.request()}},{key:"invalidate",value:function(){this.generation++}},{key:"request",value:function(){if(!(this.killed||!this.active||!this.lastEvent)){if(this.fence){this.dirty=!0;return}this.startRead()}}},{key:"startRead",value:function(){var t=this.options,e=t.gl,r=t.getFrameBuffer,i=t.getPixelRatio,o=t.getDownSizingRatio;this.readGeneration=this.generation;var s=this.lastEvent,l=fa(e,s.x,s.y,i(),o()),u=ae(l,2),d=u[0],c=u[1];this.readBuffer||(this.readBuffer=e.createBuffer()),e.bindFramebuffer(e.FRAMEBUFFER,r()),e.bindBuffer(e.PIXEL_PACK_BUFFER,this.readBuffer),e.bufferData(e.PIXEL_PACK_BUFFER,4,e.STREAM_READ),e.readPixels(d,c,1,1,e.RGBA,e.UNSIGNED_BYTE,0),e.bindBuffer(e.PIXEL_PACK_BUFFER,null),e.bindFramebuffer(e.FRAMEBUFFER,null),this.fence=e.fenceSync(e.SYNC_GPU_COMMANDS_COMPLETE,0),e.flush(),this.schedulePoll(s)}},{key:"schedulePoll",value:function(t){var e=this;this.rafId=requestAnimationFrame(function(){e.rafId=null,e.poll(t)})}},{key:"poll",value:function(t){var e=this.options,r=e.gl,i=e.onIndex;if(!(this.killed||!this.fence)){var o=r.clientWaitSync(this.fence,0,0);if(o===r.TIMEOUT_EXPIRED){this.schedulePoll(t);return}if(r.deleteSync(this.fence),this.fence=null,o!==r.WAIT_FAILED&&this.readGeneration===this.generation){r.bindBuffer(r.PIXEL_PACK_BUFFER,this.readBuffer),r.getBufferSubData(r.PIXEL_PACK_BUFFER,0,this.result),r.bindBuffer(r.PIXEL_PACK_BUFFER,null);var s=ae(this.result,4),l=s[0],u=s[1],d=s[2],c=s[3];this.active&&i(ga(l,u,d,c),t)}this.dirty&&(this.dirty=!1,this.active&&this.lastEvent&&!this.options.isRenderPending()&&this.startRead())}}},{key:"reset",value:function(){this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null,this.fence=null,this.readBuffer=null,this.dirty=!1}},{key:"kill",value:function(){this.killed=!0,this.rafId!==null&&cancelAnimationFrame(this.rafId),this.rafId=null;var t=this.options.gl;this.fence&&t.deleteSync(this.fence),this.fence=null,this.readBuffer&&t.deleteBuffer(this.readBuffer),this.readBuffer=null}}])})(),Io=(function(){function a(n,t){J(this,a),this.key=n,this.size=t}return ee(a,null,[{key:"compare",value:function(t,e){return t.size>e.size?-1:t.size<e.size||t.key>e.key?1:-1}}])})(),Yn=(function(){function a(){J(this,a),S(this,"width",0),S(this,"height",0),S(this,"cellSize",0),S(this,"columns",0),S(this,"rows",0),S(this,"cells",{})}return ee(a,[{key:"resizeAndClear",value:function(t,e){this.width=t.width,this.height=t.height,this.cellSize=e,this.columns=Math.ceil(t.width/e),this.rows=Math.ceil(t.height/e),this.cells={}}},{key:"getIndex",value:function(t){var e=Math.floor(t.x/this.cellSize),r=Math.floor(t.y/this.cellSize);return r*this.columns+e}},{key:"add",value:function(t,e,r){var i=new Io(t,e),o=this.getIndex(r),s=this.cells[o];s||(s=[],this.cells[o]=s),s.push(i)}},{key:"organize",value:function(){for(var t in this.cells){var e=this.cells[t];e.sort(Io.compare)}}},{key:"getLabelsToDisplay",value:function(t,e,r){var i=this.cellSize*this.cellSize,o=i/t/t,s=o*e/i,l=Math.ceil(s),u=[];if(r)for(var d=Math.max(0,Math.floor(r.x1/this.cellSize)),c=Math.min(this.columns-1,Math.floor(r.x2/this.cellSize)),h=Math.max(0,Math.floor(r.y1/this.cellSize)),f=Math.min(this.rows-1,Math.floor(r.y2/this.cellSize)),g=h;g<=f;g++)for(var m=d;m<=c;m++){var v=g*this.columns+m,p=this.cells[v];if(p)for(var b=0;b<Math.min(l,p.length);b++)u.push(p[b].key)}else for(var y in this.cells)for(var T=this.cells[y],x=0;x<Math.min(l,T.length);x++)u.push(T[x].key);return u}}])})();function Qc(a){var n=a.graph,t=a.hoveredNode,e=a.highlightedNodes,r=a.displayedNodeLabels,i=new Set,o=new Set(e);t&&o.add(t);var s=z(o),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;n.hasNode(u)&&n.forEachEdge(u,function(f){i.add(f)})}}catch(f){s.e(f)}finally{s.f()}var d=z(r),c;try{var h=function(){var g=c.value;if(o.has(g))return 1;n.forEachEdge(g,function(m,v,p,b){var y=p===g?b:p;r.has(y)&&i.add(m)})};for(d.s();!(c=d.n()).done;)h()}catch(f){d.e(f)}finally{d.f()}return Array.from(i)}var Pt=150,Ft=50,Jc={both:0,node:1,label:2},eh={over:0,above:1,below:2,auto:3},No=12,th=3,nh=(function(){function a(n){J(this,a),S(this,"labelGrid",new Yn),S(this,"edgeAnchorGrid",new Yn),S(this,"displayedNodeLabels",new Set),S(this,"displayedEdgeLabels",new Set),S(this,"edgeLabelCandidates",[]),S(this,"renderedNodeLabels",new Set),S(this,"labelSizeCache",new Map),S(this,"framePassPoints",new Float32Array(0)),S(this,"labelsDirty",!0),this.internals=n}return ee(a,[{key:"resetFrame",value:function(){this.displayedNodeLabels=new Set,this.renderedNodeLabels=new Set,this.labelSizeCache.clear()}},{key:"clearEdgeLabels",value:function(){this.displayedEdgeLabels=new Set}},{key:"resetLabelGrid",value:function(){this.labelGrid=new Yn,this.edgeAnchorGrid=new Yn}},{key:"processWebGLLabels",value:function(t){for(var e,r=this.internals,i=r.labelProgram,o=r.primitives,s=r.nodeDataCache,l=(o==null||(e=o.nodes)===null||e===void 0||(e=e.label)===null||e===void 0||(e=e.font)===null||e===void 0?void 0:e.family)||"sans-serif",u=new Map,d=0,c=t.length;d<c;d++){var h=t[d],f=s[h];if(!(f.visibility==="hidden"||!f.label)){var g=f.labelFont||l,m=u.get(g);m?m.push(f.label):u.set(g,[f.label])}}var v=z(u),p;try{for(v.s();!(p=v.n()).done;){var b=ae(p.value,2),y=b[0],T=b[1],x=Nn(y),_=x.family,A=x.weight,E=x.style,L=i.registerFont(_,A,E);i.ensureGlyphsReady(T,L)}}catch(w){v.e(w)}finally{v.f()}}},{key:"measureNodeLabel",value:function(t){var e,r;if(!t.label)return{width:0,height:0,textHeight:0};var i=this.internals,o=i.labelProgram,s=i.primitives,l=(e=t.labelSize)!==null&&e!==void 0?e:14,u=t.labelFont||(s==null||(r=s.nodes)===null||r===void 0||(r=r.label)===null||r===void 0||(r=r.font)===null||r===void 0?void 0:r.family)||"sans-serif",d="".concat(t.label,"|").concat(l,"|").concat(u);if(this.labelSizeCache.has(d))return this.labelSizeCache.get(d);var c=Nn(u),h=c.family,f=c.weight,g=c.style,m=o.registerFont(h,f,g),v=o.measureLabel(t.label,l,m);return this.labelSizeCache.set(d,v),v}},{key:"nodeLabelBox",value:function(t,e){var r,i,o,s,l;if(t.visibility==="hidden"||t.labelVisibility==="hidden")return null;var u=this.measureNodeLabel(t),d=u.width,c=u.height,h=u.textHeight;if(!d)return null;var f=(r=(i=this.internals.primitives)===null||i===void 0||(i=i.nodes)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:Dt,g=e+f,m=t.labelBackgroundColor?(o=t.labelBackgroundPadding)!==null&&o!==void 0?o:Ba:0,v=d/2,p=c/2,b=v+m,y=p+m,T=h/2,x=0,_=0;switch((s=t.labelPosition)!==null&&s!==void 0?s:"right"){case"left":x=-(g+v);break;case"above":_=-(g+T);break;case"below":_=g+T;break;case"over":break;default:x=g+v}var A=(l=t.labelAngle)!==null&&l!==void 0?l:0,E=ha({x,y:_},A),L=E.x,w=E.y,F=Math.abs(Math.cos(A)),D=Math.abs(Math.sin(A)),I=F*b+D*y,R=D*b+F*y;return{minX:L-I,maxX:L+I,minY:w-R,maxY:w+R}}},{key:"computeDisplayedNodeLabels",value:function(){var t=this.internals.getCameraState(),e=this.internals.getDimensions(),r=e.width,i=e.height,o=this.internals.viewportToFramedGraph({x:-Pt,y:-Ft}),s=this.internals.viewportToFramedGraph({x:r+Pt,y:-Ft}),l=this.internals.viewportToFramedGraph({x:-Pt,y:i+Ft}),u=this.internals.viewportToFramedGraph({x:r+Pt,y:i+Ft}),d=Math.min(o.x,s.x,l.x,u.x),c=Math.max(o.x,s.x,l.x,u.x),h=Math.min(o.y,s.y,l.y,u.y),f=Math.max(o.y,s.y,l.y,u.y),g=Ve({x:.5,y:.5,ratio:1,angle:0},{width:r,height:i},this.internals.getGraphDimensions(),this.internals.getStagePadding()),m=function(C){var M=bt(g,C);return{x:(1+M.x)*r/2,y:(1-M.y)*i/2}},v=m({x:d,y:h}),p=m({x:c,y:h}),b=m({x:d,y:f}),y=m({x:c,y:f}),T={x1:Math.min(v.x,p.x,b.x,y.x),y1:Math.min(v.y,p.y,b.y,y.y),x2:Math.max(v.x,p.x,b.x,y.x),y2:Math.max(v.y,p.y,b.y,y.y)},x=this.internals,_=x.settings,A=x.nodeDataCache,E=x.nodesWithForcedLabels,L=this.labelGrid.getLabelsToDisplay(t.ratio,_.labelDensity,T);Ra(L,E);for(var w=0,F=L.length;w<F;w++){var D=L[w],I=A[D];if(!this.displayedNodeLabels.has(D)&&!(I.visibility==="hidden"||I.labelVisibility==="hidden")&&I.label&&!(I.x<d||I.x>c||I.y<h||I.y>f)){var R=this.internals.framedGraphToViewport(I),P=R.x,G=R.y,N=this.internals.scaleSize(I.size);!Xe(I)&&N<_.labelRenderedSizeThreshold||P<-Pt-N||P>r+Pt+N||G<-Ft-N||G>i+Ft+N||this.displayedNodeLabels.add(D)}}}},{key:"buildFramePassPoints",value:function(){var t=this.internals,e=t.nodeDataCache,r=t.nodeDataTexture;if(!r)return{data:this.framePassPoints,count:0};var i=this.displayedNodeLabels.size*3;this.framePassPoints.length<i&&(this.framePassPoints=new Float32Array(i));var o=this.framePassPoints,s=0,l=z(this.displayedNodeLabels),u;try{for(l.s();!(u=l.n()).done;){var d,c,h=u.value,f=e[h];if(f){var g=r.getIndex(h);g<0||(o[s*3]=g,o[s*3+1]=(d=wt[f.labelPosition||We.labelPosition])!==null&&d!==void 0?d:0,o[s*3+2]=(c=f.labelAngle)!==null&&c!==void 0?c:0,s++)}}}catch(m){l.e(m)}finally{l.f()}return{data:o,count:s}}},{key:"renderWebGLLabels",value:function(t,e){var r=this.internals,i=r.nodeDataCache,o=r.labelProgram,s=r.primitives,l=r.nodeDataTexture,u=[],d=z(this.displayedNodeLabels),c;try{for(d.s();!(c=d.n()).done;){var h=c.value;if(!this.renderedNodeLabels.has(h)){var f=i[h];e&&f.labelDepth!==e||(this.renderedNodeLabels.add(h),u.push(h))}}}catch(ve){d.e(ve)}finally{d.f()}if(u.length!==0){if(this.labelsDirty){for(var g,m,v,p=0,b=0,y=u.length;b<y;b++){var T=i[u[b]];p+=T.label.length}o.reallocate(p);for(var x=14,_=(g=s==null||(m=s.nodes)===null||m===void 0||(m=m.label)===null||m===void 0?void 0:m.margin)!==null&&g!==void 0?g:Dt,A=We.labelPosition,E=(s==null||(v=s.nodes)===null||v===void 0||(v=v.label)===null||v===void 0||(v=v.font)===null||v===void 0?void 0:v.family)||"sans-serif",L=new Map,w=0,F=0,D=u.length;F<D;F++){var I,R,P,G,N=u[F],k=i[N],C=k.labelFont||E,M=L.get(C);if(M===void 0){var $=Nn(C),H=$.family,Y=$.weight,j=$.style;M=o.registerFont(H,Y,j),L.set(C,M)}var q={text:k.label,x:k.x,y:k.y,size:(I=k.labelSize)!==null&&I!==void 0?I:x,color:k.labelColor,nodeSize:k.size,margin:_,position:(R=k.labelPosition)!==null&&R!==void 0?R:A,hidden:!1,forceLabel:Xe(k),type:"default",zIndex:(P=k.zIndex)!==null&&P!==void 0?P:0,parentType:"node",parentKey:N,fontKey:M,labelAngle:(G=k.labelAngle)!==null&&G!==void 0?G:0,nodeIndex:l.getIndex(N)},be=o.processLabel(N,w,q);w+=be}o.invalidateBuffers()}o.render(t)}}},{key:"renderBackdrops",value:function(t,e){var r=this.internals,i=r.backdropProgram,o=r.nodeDataCache,s=r.nodesWithBackdrop,l=r.attachmentManager,u=r.pixelRatio,d=r.nodeDataTexture,c=[],h=z(s),f;try{for(h.s();!(f=h.n()).done;){var g,m=f.value,v=o[m];if(!(!v||v.visibility==="hidden")&&!(e&&v.depth!==e)){var p=(g=d?.getIndex(m))!==null&&g!==void 0?g:-1;p<0||c.push({key:m,nodeIndex:p})}}}catch(Ge){h.e(Ge)}finally{h.f()}if(c.length!==0){i.reallocate(c.length);for(var b=0;b<c.length;b++){var y,T,x,_,A,E,L,w,F=c[b],D=F.key,I=F.nodeIndex,R=o[D],P=this.displayedNodeLabels.has(D),G=P?this.measureNodeLabel(R):{width:0,height:0,textHeight:0},N=G.textHeight,k=G.width,C=G.height,M=0,$=0;if(P&&R.labelAttachment&&l){var H=l.getEntry(D,R.labelAttachment);if(H){var Y=R.labelAttachmentPlacement||"below";if(Y==="below"||Y==="above"){var j=H.height/u;C+=j+Ye,k=Math.max(k,H.width/u),$=Y==="below"?(j+Ye)/2:-(j+Ye)/2}else{var q=H.width/u;k+=q+Ye,C=Math.max(C,H.height/u),M=Y==="right"?(q+Ye)/2:-(q+Ye)/2}}}var be=R.backdropColor?je(R.backdropColor):[255,255,255,255],ve=R.backdropShadowColor?je(R.backdropShadowColor):[0,0,0,128],ye=be.map(function(Ge){return Ge/255}),Oe=ve.map(function(Ge){return Ge/255}),He=(y=R.backdropShadowBlur)!==null&&y!==void 0?y:12,xe=(T=R.backdropPadding)!==null&&T!==void 0?T:6,Qe=R.backdropBorderColor?je(R.backdropBorderColor):[0,0,0,0],ln=Qe.map(function(Ge){return Ge/255}),ta=(x=R.backdropBorderWidth)!==null&&x!==void 0?x:0,It=(_=R.backdropCornerRadius)!==null&&_!==void 0?_:0,ne=(A=R.backdropLabelPadding)!==null&&A!==void 0?A:-1,Pe=ne<0?xe:ne,Ne=(E=Jc[(L=R.backdropArea)!==null&&L!==void 0?L:"both"])!==null&&E!==void 0?E:0,na={key:D,nodeIndex:I,label:R.label,labelWidth:k,labelHeight:C,textHeight:N,type:"default",position:R.labelPosition||We.labelPosition,labelAngle:(w=R.labelAngle)!==null&&w!==void 0?w:0,backdropColor:ye,backdropShadowColor:Oe,backdropShadowBlur:He,backdropPadding:xe,backdropBorderColor:ln,backdropBorderWidth:ta,backdropCornerRadius:It,backdropLabelPadding:Pe,backdropArea:Ne,labelBoxOffset:[M,$]};i.processBackdrop(b,na)}i.invalidateBuffers(),i.render(t)}}},{key:"renderLabelBackgrounds",value:function(t,e){var r=this.internals,i=r.labelBackgroundProgram,o=r.nodeDataCache,s=r.nodeDataTexture,l=me.nodeLabel.writesPickingThisFrame(this.internals),u=[],d=z(this.displayedNodeLabels),c;try{for(d.s();!(c=d.n()).done;){var h=c.value,f=o[h];!f||f.visibility==="hidden"||e&&f.labelDepth!==e||!l&&!f.labelBackgroundColor||u.push(h)}}catch(I){d.e(I)}finally{d.f()}if(u.length!==0){i.reallocate(u.length);for(var g=0;g<u.length;g++){var m,v,p,b,y=u[g],T=o[y],x=(m=s?.getIndex(y))!==null&&m!==void 0?m:-1;if(!(x<0)){var _=this.measureNodeLabel(T),A=_.width,E=_.height,L=_.textHeight,w=lt(this.internals.pickingState,"nodeLabel",y)||lt(this.internals.pickingState,"node",y),F=T.labelBackgroundColor?De(T.labelBackgroundColor):De("transparent"),D={nodeIndex:x,id:yt(w),color:F,labelWidth:A,labelHeight:E,textHeight:L,positionMode:(v=wt[T.labelPosition||We.labelPosition])!==null&&v!==void 0?v:0,labelAngle:(p=T.labelAngle)!==null&&p!==void 0?p:0,padding:(b=T.labelBackgroundPadding)!==null&&b!==void 0?b:Ba};i.processLabelBackground(g,D)}}i.invalidateBuffers(),i.render(t)}}},{key:"cacheAttachments",value:function(t){var e=this.internals,r=e.attachmentManager,i=e.pixelRatio,o=e.nodeDataCache,s=e.nodesWithBackdrop,l=e.graph;if(r){var u=z(s),d;try{for(u.s();!(d=u.n()).done;){var c=d.value;if(this.displayedNodeLabels.has(c)){var h=o[c];if(!(!h||h.visibility==="hidden")&&!(t&&h.depth!==t)&&h.labelAttachment){var f=l.getNodeAttributes(c),g=this.measureNodeLabel(h),m=g.width,v=g.height,p={node:c,attributes:f,pixelRatio:i,labelWidth:m,labelHeight:v};r.renderAttachment(c,h.labelAttachment,p)}}}}catch(b){u.e(b)}finally{u.f()}r.regenerateAtlas()}}},{key:"renderAttachments",value:function(t,e){var r=this.internals,i=r.attachmentManager,o=r.attachmentProgram,s=r.nodeDataCache,l=r.nodesWithBackdrop,u=r.pixelRatio,d=r.nodeDataTexture;if(!(!i||!o)){var c=0;o.reallocateAttachments(l.size);var h=z(l),f;try{for(h.s();!(f=h.n()).done;){var g,m,v,p,b=f.value;if(this.displayedNodeLabels.has(b)){var y=s[b];if(!(!y||y.visibility==="hidden")&&!(e&&y.labelDepth!==e)&&y.labelAttachment){var T=i.getEntry(b,y.labelAttachment);if(T){var x=(g=d?.getIndex(b))!==null&&g!==void 0?g:-1;if(!(x<0)){var _=this.measureNodeLabel(y),A=_.width,E=_.height,L=_.textHeight,w=(m=Zi[y.labelAttachmentPlacement||"below"])!==null&&m!==void 0?m:0;o.processAttachment(c,{nodeIndex:x,atlasX:T.x,atlasY:T.y,atlasW:T.width,atlasH:T.height,attachWidth:T.width/u,attachHeight:T.height/u,positionMode:(v=wt[y.labelPosition||We.labelPosition])!==null&&v!==void 0?v:0,attachmentPlacement:w,labelWidth:A,labelHeight:E,textHeight:L,labelAngle:(p=y.labelAngle)!==null&&p!==void 0?p:0}),c++}}}}}}catch(F){h.e(F)}finally{h.f()}c!==0&&(o.reallocateAttachments(c),i.bindTexture(tn),o.invalidateBuffers(),o.render(t))}}},{key:"computeDisplayedEdgeLabels",value:function(){var t=this.internals,e=t.graph,r=t.stateManager,i=t.settings,o=t.edgesWithForcedLabels,s=r.getHighlightedNodes(),l=i.edgeLabelAnchors==="allNodes"?new Set(this.edgeAnchorGrid.getLabelsToDisplay(this.internals.getCameraState().ratio,i.labelDensity)):this.displayedNodeLabels,u=r.hovered,d=Qc({graph:e,hoveredNode:u?.kind==="node"?u.key:null,displayedNodeLabels:l,highlightedNodes:s});Ra(d,o),this.edgeLabelCandidates=d,this.displayedEdgeLabels=new Set}},{key:"filterEdgeLabelsForDepth",value:function(t){for(var e=this.internals,r=e.graph,i=e.nodeDataCache,o=e.edgeDataCache,s=[],l=new Set,u=0,d=this.edgeLabelCandidates.length;u<d;u++){var c=this.edgeLabelCandidates[u];if(!l.has(c)){l.add(c);var h=r.extremities(c),f=i[h[0]],g=i[h[1]],m=o[c];!m||!f||!g||m.visibility==="hidden"||m.labelVisibility==="hidden"||f.visibility==="hidden"||g.visibility==="hidden"||t&&m.labelDepth!==t||m.label&&s.push(c)}}return s}},{key:"renderEdgeLabels",value:function(t,e){var r=this.internals,i=r.graph,o=r.nodeDataCache,s=r.edgeDataCache,l=r.primitives,u=r.edgeLabelProgram,d=r.nodeDataTexture,c=r.edgeDataTexture;if(u){var h=this.filterEdgeLabelsForDepth(e),f=z(h),g;try{for(f.s();!(g=f.n()).done;){var m=g.value;this.displayedEdgeLabels.add(m)}}catch(q){f.e(q)}finally{f.f()}if(h.length!==0){if(this.labelsDirty){var v,p,b=0,y=z(h),T;try{for(y.s();!(T=y.n()).done;){var x=T.value;b+=s[x].label.length}}catch(q){y.e(q)}finally{y.f()}u.reallocate(b);var _=(v=l==null||(p=l.edges)===null||p===void 0||(p=p.label)===null||p===void 0?void 0:p.margin)!==null&&v!==void 0?v:5,A="over",E=0,L=z(h),w;try{for(L.s();!(w=L.n()).done;){var F,D,I=w.value,R=i.extremities(I),P=R[0],G=R[1],N=o[P],k=o[G],C=s[I],M=d.getIndex(P),$=d.getIndex(G),H=c.getIndex(I),Y={text:C.label,x:(N.x+k.x)/2,y:(N.y+k.y)/2,size:No,color:C.labelColor,nodeSize:0,nodeIndex:-1,margin:_,position:(F=C.labelPosition)!==null&&F!==void 0?F:A,hidden:!1,forceLabel:Xe(C),type:"default",zIndex:(D=C.zIndex)!==null&&D!==void 0?D:0,parentType:"edge",parentKey:I,fontKey:"",labelAngle:0,sourceX:N.x,sourceY:N.y,targetX:k.x,targetY:k.y,sourceSize:N.size,targetSize:k.size,sourceShape:N.shape||"circle",targetShape:k.shape||"circle",edgeSize:C.size,offset:0,edgeAttributes:C,sourceNodeIndex:M,targetNodeIndex:$,edgeIndex:H},j=u.processEdgeLabel(I,E,Y);E+=j}}catch(q){L.e(q)}finally{L.f()}u.invalidateBuffers()}u.render(t)}}}},{key:"renderEdgeLabelBackgrounds",value:function(t,e){var r,i,o=this.internals,s=o.edgeLabelBackgroundProgram,l=o.edgeLabelProgram,u=o.edgeDataCache,d=o.primitives,c=o.edgeDataTexture;if(!(!c||!l||!s)){var h=me.edgeLabel.writesPickingThisFrame(this.internals),f=(r=d==null||(i=d.edges)===null||i===void 0||(i=i.label)===null||i===void 0?void 0:i.margin)!==null&&r!==void 0?r:5,g="over",m=this.filterEdgeLabelsForDepth(e),v=[],p=z(m),b;try{for(p.s();!(b=p.n()).done;){var y=b.value;!h&&!u[y].labelBackgroundColor||v.push(y)}}catch(N){p.e(N)}finally{p.f()}if(v.length!==0){s.reallocate(v.length);for(var T=0;T<v.length;T++){var x,_,A,E=v[T],L=u[E],w=L.label,F=l.measureLabelAtlasWidth(w),D=(x=L.labelPosition)!==null&&x!==void 0?x:g,I=typeof D=="string"&&(_=eh[D])!==null&&_!==void 0?_:0,R=lt(this.internals.pickingState,"edgeLabel",E)||lt(this.internals.pickingState,"edge",E),P=L.labelBackgroundColor?De(L.labelBackgroundColor):De("transparent"),G={edgeIndex:c.getIndex(E),baseFontSize:No,totalTextWidth:F,positionMode:I,margin:f,padding:(A=L.labelBackgroundPadding)!==null&&A!==void 0?A:th,color:P,id:yt(R),edgeAttributes:L};s.processEdgeLabelBackground(T,E,G)}s.invalidateBuffers(),s.render(t)}}}}])})(),ah=(function(){function a(n,t,e,r){J(this,a),S(this,"nodeStates",new Map),S(this,"edgeStates",new Map),S(this,"hovered",null),S(this,"dirtyNodes",new Set),S(this,"dirtyEdges",new Set),S(this,"graphStateChanged",!1),S(this,"graphStateFlagsDirty",!1),this.scheduleRefresh=n,this.customNodeStateDefaults=t,this.customEdgeStateDefaults=e,this.customGraphStateDefaults=r,this.graphState=Ta(r)}return ee(a,[{key:"getNodeState",value:function(t){t=""+t;var e=this.nodeStates.get(t);return e||(e=_i(this.customNodeStateDefaults),this.nodeStates.set(t,e)),e}},{key:"getEdgeState",value:function(t){t=""+t;var e=this.edgeStates.get(t);return e||(e=Ti(this.customEdgeStateDefaults),this.edgeStates.set(t,e)),e}},{key:"getGraphState",value:function(){return this.flushGraphStateFlags(),this.graphState}},{key:"getHighlightedNodes",value:function(){var t=new Set,e=z(this.nodeStates),r;try{for(e.s();!(r=e.n()).done;){var i=ae(r.value,2),o=i[0],s=i[1];s.isHighlighted&&t.add(o)}}catch(l){e.e(l)}finally{e.f()}return t}},{key:"setNodeState",value:function(t,e){t=""+t;var r=this.getNodeState(t);if(St(r,e)){var i=B(B({},r),e);this.nodeStates.set(t,i),this.dirtyNodes.add(t),this.updateHoveredNodeTracking(t,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setEdgeState",value:function(t,e){t=""+t;var r=this.getEdgeState(t);if(St(r,e)){var i=B(B({},r),e);this.edgeStates.set(t,i),this.dirtyEdges.add(t),this.updateHoveredEdgeTracking(t,r,i),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}}},{key:"setGraphState",value:function(t){if(St(this.graphState,t)){var e=B(B({},this.graphState),t);e.isIdle=!e.isPanning&&!e.isZooming&&!e.isDragging,this.graphState=e,this.graphStateChanged=!0,this.scheduleRefresh()}}},{key:"setNodesState",value:function(t,e){var r=!1,i=z(t),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getNodeState(s);if(St(l,e)){var u=B(B({},l),e);this.nodeStates.set(s,u),this.dirtyNodes.add(s),this.updateHoveredNodeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"setEdgesState",value:function(t,e){var r=!1,i=z(t),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;s=""+s;var l=this.getEdgeState(s);if(St(l,e)){var u=B(B({},l),e);this.edgeStates.set(s,u),this.dirtyEdges.add(s),this.updateHoveredEdgeTracking(s,l,u),r=!0}}}catch(d){i.e(d)}finally{i.f()}r&&(this.graphStateFlagsDirty=!0,this.scheduleRefresh())}},{key:"markNodesDirty",value:function(t){var e=z(t),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;this.dirtyNodes.add(i)}}catch(o){e.e(o)}finally{e.f()}this.scheduleRefresh()}},{key:"removeNode",value:function(t){this.nodeStates.delete(t),this.dirtyNodes.delete(t),this.clearHoveredFor("node",t),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}},{key:"removeEdge",value:function(t){this.edgeStates.delete(t),this.dirtyEdges.delete(t),this.clearHoveredFor("edge",t),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}},{key:"pruneNodes",value:function(t){var e=z(this.nodeStates.keys()),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;t(i)||this.removeNode(i)}}catch(o){e.e(o)}finally{e.f()}}},{key:"pruneEdges",value:function(t){var e=z(this.edgeStates.keys()),r;try{for(e.s();!(r=e.n()).done;){var i=r.value;t(i)||this.removeEdge(i)}}catch(o){e.e(o)}finally{e.f()}}},{key:"clearNodes",value:function(){this.nodeStates.clear(),this.dirtyNodes.clear(),this.clearHoveredForKinds(Qn("node")),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}},{key:"clearEdges",value:function(){this.edgeStates.clear(),this.dirtyEdges.clear(),this.clearHoveredForKinds(Qn("edge")),this.graphStateFlagsDirty=!0,this.scheduleRefresh()}},{key:"resetGraphState",value:function(){this.graphState=Ta(this.customGraphStateDefaults),this.graphStateChanged=!1,this.graphStateFlagsDirty=!1}},{key:"clearDirtyTracking",value:function(){this.dirtyNodes.clear(),this.dirtyEdges.clear(),this.graphStateChanged=!1}},{key:"setHovered",value:function(t){this.hovered=t}},{key:"clearHoveredFor",value:function(t,e){this.hovered&&this.hovered.key===e&&Qn(t).includes(this.hovered.kind)&&(this.hovered=null)}},{key:"clearHoveredForKinds",value:function(t){this.hovered&&t.includes(this.hovered.kind)&&(this.hovered=null)}},{key:"updateGraphStateFromNodes",value:function(){var t,e=!1,r=!1,i=!1,o=z(this.nodeStates),s;try{for(o.s();!(s=o.n()).done;){var l=ae(s.value,2),u=l[1];if(u.isHovered&&(e=!0),u.isHighlighted&&(r=!0),u.isDragged&&(i=!0),e&&r&&i)break}}catch(c){o.e(c)}finally{o.f()}!e&&((t=this.hovered)===null||t===void 0?void 0:t.kind)==="edge"&&(e=!0);var d=!this.graphState.isPanning&&!this.graphState.isZooming&&!i;(this.graphState.hasHovered!==e||this.graphState.hasHighlighted!==r||this.graphState.isDragging!==i||this.graphState.isIdle!==d)&&(this.graphStateChanged=!0),this.graphState=B(B({},this.graphState),{},{hasHovered:e,hasHighlighted:r,isDragging:i,isIdle:d})}},{key:"updateGraphStateFromEdges",value:function(){var t,e=((t=this.hovered)===null||t===void 0?void 0:t.kind)==="node";if(!e){var r=z(this.edgeStates),i;try{for(r.s();!(i=r.n()).done;){var o=ae(i.value,2),s=o[1];if(s.isHovered){e=!0;break}}}catch(l){r.e(l)}finally{r.f()}}this.graphState.hasHovered!==e&&(this.graphStateChanged=!0),this.graphState=B(B({},this.graphState),{},{hasHovered:e})}},{key:"flushGraphStateFlags",value:function(){this.graphStateFlagsDirty&&(this.updateGraphStateFromNodes(),this.updateGraphStateFromEdges(),this.graphStateFlagsDirty=!1)}},{key:"updateHoveredNodeTracking",value:function(t,e,r){var i;if(e.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="node"&&this.hovered.key!==t){var s=this.hovered.key,l=this.getNodeState(s);this.nodeStates.set(s,B(B({},l),{},{isHovered:!1})),this.dirtyNodes.add(s)}this.hovered={kind:"node",key:t}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="node"&&this.hovered.key===t&&(this.hovered=null)}},{key:"updateHoveredEdgeTracking",value:function(t,e,r){var i;if(e.isHovered!==r.isHovered)if(r.isHovered){var o;if(((o=this.hovered)===null||o===void 0?void 0:o.kind)==="edge"&&this.hovered.key!==t){var s=this.hovered.key,l=this.getEdgeState(s);this.edgeStates.set(s,B(B({},l),{},{isHovered:!1})),this.dirtyEdges.add(s)}this.hovered={kind:"edge",key:t}}else((i=this.hovered)===null||i===void 0?void 0:i.kind)==="edge"&&this.hovered.key===t&&(this.hovered=null)}}])})(),Go=1,Mo=2,zo=3,Bo=4,rh=(function(a){function n(t,e){var r,i,o,s,l,u=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};J(this,n),l=le(this,n),S(l,"nodeReducer",null),S(l,"edgeReducer",null),S(l,"stageCanvas",null),S(l,"mouseLayer",null),S(l,"gestureHint",null),S(l,"extraElements",{}),S(l,"webGLContext",null),S(l,"pickingFrameBuffer",null),S(l,"pickingTexture",null),S(l,"pickingDepthBuffer",null),S(l,"activeListeners",{}),S(l,"nodeVariableEntries",[]),S(l,"edgeVariableEntries",[]),S(l,"edgePathsByName",new Map),S(l,"nodeProgramIndex",{}),S(l,"edgeProgramIndex",{}),S(l,"edgeTextureIndexCache",{}),S(l,"nodeGraphCoords",{}),S(l,"nodeExtent",{x:[0,1],y:[0,1]}),S(l,"matrix",we()),S(l,"invMatrix",we()),S(l,"correctionRatio",1),S(l,"frameId",0),S(l,"customBBox",null),S(l,"gpuTimerExt",void 0),S(l,"activeGpuTimerQuery",null),S(l,"pendingGpuTimerQueries",[]),S(l,"normalizationFunction",Pn({x:[0,1],y:[0,1]})),S(l,"graphToViewportRatio",1),S(l,"pickingState",Pc()),S(l,"prevNodeVisibilities",{}),S(l,"width",0),S(l,"height",0),S(l,"autoRescaleFrozen",!1),S(l,"stylesDeclaration",null),S(l,"resolvedStageStyle",{}),S(l,"renderFrame",null),S(l,"contextLost",!1),S(l,"pendingProcess","full"),S(l,"needToRefreshState",!1),S(l,"checkEdgesEventsFrame",null),S(l,"edgeStyleAnalysis",{dependency:"static",xAttribute:null,yAttribute:null}),S(l,"depthLayers",V(An)),S(l,"customLayerPrograms",new Map),S(l,"nodeShapeSlug",null),S(l,"sdfAtlas",null),S(l,"depthRanges",{nodes:{},edges:{}}),S(l,"nodeBaseDepth",{}),S(l,"edgeBaseDepth",{});var d=u.primitives,c=u.styles,h=u.settings,f=h===void 0?{}:h,g=u.nodeReducer,m=u.edgeReducer,v=u.customNodeState,p=u.customEdgeState,b=u.customGraphState;l.stateManager=new ah(function(){return l.scheduleStateRefresh()},v,p,b);var y=d??Da;l.stylesDeclaration=c?{nodes:(r=c.nodes)!==null&&r!==void 0?r:Dn.nodes,edges:(i=c.edges)!==null&&i!==void 0?i:Dn.edges,stage:c.stage}:Dn,l.nodeReducer=g??null,l.edgeReducer=m??null;var T=Rn(l.stylesDeclaration.nodes);l.nodeReducer&&(T.dependency="graph-state"),l.edgeStyleAnalysis=Rn(l.stylesDeclaration.edges),l.edgeReducer&&(l.edgeStyleAnalysis.dependency="graph-state"),l.stylesDeclaration.stage&&(l.resolvedStageStyle=wa(l.stylesDeclaration.stage,l.stateManager.graphState));var x=Pi(f);if(In(x),x.enableNodeDrag){var _=T.xAttribute,A=T.yAttribute;if((!_||!A)&&!x.dragPositionToAttributes)throw new Error('Sigma: `enableNodeDrag` is true but position attribute names could not be inferred from styles. Either use attribute bindings for x/y in your node styles (e.g. `x: { attribute: "x" }`), or provide a `dragPositionToAttributes` setting.')}if(ki(t),!(e instanceof HTMLElement))throw new Error("Sigma: container should be an html element.");l.container=e,l.edgeGroups=new kc(t,function(k,C){for(var M=0;M<k.length;M++){var $=l.stateManager.getEdgeState(k[M]);$.parallelIndex=M,$.parallelCount=C}});var E=new Cc(t,l.viewportToGraph.bind(l),l.setNodesState.bind(l),function(k,C){return l.emit(k,C)});if(l.depthLayers=(o=y.depthLayers)!==null&&o!==void 0?o:V(An),!(c!=null&&c.nodes)&&!Sn.every(function(k){return l.depthLayers.includes(k)}))throw new Error("Sigma: depthLayers must include ".concat(Sn.join(", ")," for the built-in node styles."));if(!(c!=null&&c.edges)&&!En.every(function(k){return l.depthLayers.includes(k)}))throw new Error("Sigma: depthLayers must include ".concat(En.join(", ")," for the built-in edge styles."));l.itemBuckets={nodes:new qa(l.depthLayers),edges:new qa(l.depthLayers)},l.initWebGLContext(),l.mouseLayer=Et("div",{position:"absolute",touchAction:"none",userSelect:"none"},{class:"sigma-mouse"}),l.container.appendChild(l.mouseLayer),l.resolvedStageStyle.background&&(l.container.style.backgroundColor=l.resolvedStageStyle.background),l.resolvedStageStyle.cursor&&(l.container.style.cursor=l.resolvedStageStyle.cursor);var L=new xo(l.webGLContext),w=new Ka(l.webGLContext,{channels:1}),F=new _o(l.webGLContext),D=new Ka(l.webGLContext,{channels:4}),I=l.webGLContext,R=new Zc({gl:I,getFrameBuffer:function(){return l.pickingFrameBuffer},getPixelRatio:function(){return l.internals.pixelRatio},getDownSizingRatio:function(){return l.internals.settings.pickingDownSizingRatio},isRenderPending:function(){return l.renderFrame!==null},onIndex:function(C,M){var $;return Ho(l.internals,($=l.pickingState.lookup[C])!==null&&$!==void 0?$:null,M)}}),P=l.initPrograms(y,x),G=y==null||(s=y.nodes)===null||s===void 0?void 0:s.labelAttachments,N=null;return G&&Object.keys(G).length>0&&(N=new yo(I,G,function(){return l.scheduleRender()})),l.internals=B(B({nodeDataCache:{},edgeDataCache:{},nodesWithForcedLabels:new Set,nodesWithBackdrop:new Set,edgesWithForcedLabels:new Set,settings:x,primitives:y,pixelRatio:qt(),graph:t,stateManager:l.stateManager,dragManager:E,hoverResolver:R,nodeStyleAnalysis:T,pickingState:l.pickingState},P),{},{attachmentManager:N,nodeDataTexture:L,nodeFrameTexture:w,edgeDataTexture:F,edgeFrameTexture:D,getDimensions:function(){return l.getDimensions()},getGraphDimensions:function(){return l.getGraphDimensions()},getStagePadding:function(){return l.getStagePadding()},getCameraState:function(){return l.camera.getState()},getHitAtPosition:function(C){return l.getHitAtPosition(C)},setNodeState:function(C,M){return l.setNodeState(C,M)},setEdgeState:function(C,M){return l.setEdgeState(C,M)},updateContainerCursor:function(){return l.updateContainerCursor()},scheduleRefresh:function(){return l.scheduleRefresh()},viewportToFramedGraph:function(C){return l.viewportToFramedGraph(C)},viewportToGraph:function(C){return l.viewportToGraph(C)},framedGraphToViewport:function(C){return l.framedGraphToViewport(C)},scaleSize:function(C){return l.scaleSize(C)},emit:function(C,M){return l.emit(C,M)}}),l.labelRenderer=new nh(l.internals),l.resize(),l.initializeWebGLLabels(),l.camera=new Eo,l.bindCameraHandlers(),l.mouseCaptor=new wc(l.mouseLayer,l),l.mouseCaptor.setSettings(l.internals.settings),l.touchCaptor=new Lc(l.mouseLayer,l),l.touchCaptor.setSettings(l.internals.settings),l.bindEventHandlers(),l.bindGraphHandlers(),l.handleSettingsUpdate(),l.refresh(),l}return ue(n,a),ee(n,[{key:"initializeWebGLLabels",value:function(){this.sdfAtlas=new at,this.sdfAtlas.registerFont({family:"sans-serif",weight:"normal",style:"normal"})}},{key:"resetWebGLTexture",value:function(){var e=this.webGLContext;if(!this.pickingFrameBuffer)return this;var r=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),i=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);e.bindFramebuffer(e.FRAMEBUFFER,this.pickingFrameBuffer),this.pickingTexture&&e.deleteTexture(this.pickingTexture);var o=e.createTexture();o&&(e.bindTexture(e.TEXTURE_2D,o),e.texImage2D(e.TEXTURE_2D,0,e.RGBA,r,i,0,e.RGBA,e.UNSIGNED_BYTE,null),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.framebufferTexture2D(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,o,0),this.pickingTexture=o),this.pickingDepthBuffer&&e.deleteRenderbuffer(this.pickingDepthBuffer);var s=e.createRenderbuffer();return s&&(e.bindRenderbuffer(e.RENDERBUFFER,s),e.renderbufferStorage(e.RENDERBUFFER,e.DEPTH_COMPONENT16,r,i),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.DEPTH_ATTACHMENT,e.RENDERBUFFER,s),this.pickingDepthBuffer=s),e.bindFramebuffer(e.FRAMEBUFFER,null),this}},{key:"bindCameraHandlers",value:function(){var e=this;return this.activeListeners.camera=function(){e.refreshMatrices(),e.labelRenderer.labelsDirty=!0,e.scheduleRender()},this.activeListeners.cameraAnimationStart=function(r){var i=r.from,o=r.to;i.ratio!==o.ratio&&e.stateManager.setGraphState({isZooming:!0})},this.activeListeners.cameraAnimationEnd=function(){e.stateManager.setGraphState({isZooming:!1})},this.camera.on("updated",this.activeListeners.camera),this.camera.on("animationStart",this.activeListeners.cameraAnimationStart),this.camera.on("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"unbindCameraHandlers",value:function(){return this.camera.removeListener("updated",this.activeListeners.camera),this.camera.removeListener("animationStart",this.activeListeners.cameraAnimationStart),this.camera.removeListener("animationEnd",this.activeListeners.cameraAnimationEnd),this}},{key:"getHitAtPosition",value:function(e){var r;if(this.contextLost)return null;var i=this.webGLContext;i.bindFramebuffer(i.FRAMEBUFFER,this.pickingFrameBuffer);var o=ri(i,this.pickingFrameBuffer,e.x,e.y,this.internals.pixelRatio,this.internals.settings.pickingDownSizingRatio),s=ga.apply(void 0,V(o));return(r=this.pickingState.lookup[s])!==null&&r!==void 0?r:null}},{key:"bindEventHandlers",value:function(){return Bc(this.internals,this.mouseCaptor,this.touchCaptor,this.activeListeners),this}},{key:"bindGraphHandlers",value:function(){var e=this;return Wc({graph:this.internals.graph,edgeGroups:this.edgeGroups,updateNode:this.updateNode.bind(this),removeNode:this.removeNode.bind(this),addEdge:this.addEdge.bind(this),updateEdge:this.updateEdge.bind(this),removeEdge:this.removeEdge.bind(this),clearEdgeState:this.clearEdgeState.bind(this),clearNodeState:this.clearNodeState.bind(this),clearEdgeIndices:this.clearEdgeIndices.bind(this),clearNodeIndices:this.clearNodeIndices.bind(this),refresh:this.refresh.bind(this),scheduleProcess:function(){e.pendingProcess="full",e.scheduleRender()},markNodesDirty:function(i){e.internals.nodeStyleAnalysis.dependency==="graph-state"&&e.stateManager.markNodesDirty(i)}},this.activeListeners),this}},{key:"unbindGraphHandlers",value:function(){Uc(this.internals.graph,this.activeListeners)}},{key:"getNodeShapeId",value:function(e){return this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&e.shape&&e.shape in this.internals.nodeShapeMap?this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[e.shape]]:Jt(e.shape||"circle")}},{key:"processNodes",value:function(){var e=this,r=this.internals.graph,i=this.internals.settings,o=this.getDimensions(),s=i.autoRescale,l=i.autoRescaleContent,u=this.nodeExtent;if(s===!1){var d=o.width,c=o.height;u={x:[-d/2,d/2],y:[-c/2,c/2]}}else(s!=="once"||!this.autoRescaleFrozen)&&(u=this.computeNodeExtent(),l!=="positions"&&!this.customBBox&&(u=Vc({extent:u,coords:this.nodeGraphCoords,nodeData:this.internals.nodeDataCache,dimensions:o,stagePadding:this.getStagePadding(),zoomToSizeRatioFunction:i.zoomToSizeRatioFunction,itemSizesReference:i.itemSizesReference,fitLabels:l==="labels",nodeLabelBox:function($,H){return e.labelRenderer.nodeLabelBox($,H)}})),s==="once"&&(this.autoRescaleFrozen=!0));this.nodeExtent=u,this.normalizationFunction=Pn(this.customBBox||this.nodeExtent);var h=new Eo,f=Ve(h.getState(),o,this.getGraphDimensions(),this.getStagePadding());this.labelRenderer.labelGrid.resizeAndClear(o,i.labelGridCellSize),this.labelRenderer.edgeAnchorGrid.resizeAndClear(o,i.labelGridCellSize);var g=i.renderEdgeLabels&&i.edgeLabelAnchors==="allNodes",m=!1;Kn(this.pickingState,"node");for(var v=r.nodes(),p=0,b=v.length;p<b;p++){var y=v[p],T=this.internals.nodeDataCache[y],x=this.nodeGraphCoords[y];T.x=x.x,T.y=x.y,this.normalizationFunction.applyTo(T),T.visibility!==this.prevNodeVisibilities[y]&&(m=!0),typeof T.label=="string"&&T.visibility!=="hidden"&&T.labelVisibility!=="hidden"&&this.labelRenderer.labelGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:f})),g&&T.visibility!=="hidden"&&this.labelRenderer.edgeAnchorGrid.add(y,T.size,this.framedGraphToViewport(T,{matrix:f}))}this.labelRenderer.labelGrid.organize(),this.labelRenderer.edgeAnchorGrid.organize(),this.nodeProgram.reallocate(v.length);var _=0;this.depthRanges.nodes={},this.nodeBaseDepth={};var A=this.internals.nodeDataCache,E=z(this.depthLayers),L;try{for(E.s();!(L=E.n()).done;){var w=L.value,F=this.itemBuckets.nodes.getSorted(w,function(M){return A[M].zIndex});if(F.length!==0){this.depthRanges.nodes[w]=[{offset:_,count:F.length}];var D=z(F),I;try{for(D.s();!(I=D.n()).done;){var R=I.value;this.nodeBaseDepth[R]=w,Lo(this.pickingState,"node",R),this.addNodeToProgram(R,_++)}}catch(M){D.e(M)}finally{D.f()}}}}catch(M){E.e(M)}finally{E.f()}this.nodeProgram.invalidateBuffers();for(var P=0,G=v.length;P<G;P++)this.prevNodeVisibilities[v[P]]=this.internals.nodeDataCache[v[P]].visibility;this.labelRenderer.processWebGLLabels(v);var N=z(this.customLayerPrograms.values()),k;try{for(N.s();!(k=N.n()).done;){var C=k.value.program;C.cacheData&&C.cacheData()}}catch(M){N.e(M)}finally{N.f()}return m}},{key:"processEdges",value:function(){var e=this.internals.graph,r=e.edges();this.edgeProgram.reallocate(r.length);var i=0;Kn(this.pickingState,"edge"),this.depthRanges.edges={},this.edgeBaseDepth={};var o=this.internals.edgeDataCache,s=z(this.depthLayers),l;try{for(s.s();!(l=s.n()).done;){var u=l.value,d=this.itemBuckets.edges.getSorted(u,function(g){return o[g].zIndex});if(d.length!==0){this.depthRanges.edges[u]=[{offset:i,count:d.length}];var c=z(d),h;try{for(c.s();!(h=c.n()).done;){var f=h.value;this.edgeBaseDepth[f]=u,Lo(this.pickingState,"edge",f),this.addEdgeToProgram(f,i++)}}catch(g){c.e(g)}finally{c.f()}}}}catch(g){s.e(g)}finally{s.f()}this.edgeProgram.invalidateBuffers()}},{key:"updateNodeDepthRanges",value:function(e,r,i){var o=this.nodeProgramIndex[e];o!==void 0&&(ka(this.depthRanges.nodes,r,o),Pa(this.depthRanges.nodes,i,o))}},{key:"updateEdgeDepthRanges",value:function(e,r,i){var o=this.edgeProgramIndex[e];o!==void 0&&(ka(this.depthRanges.edges,r,o),Pa(this.depthRanges.edges,i,o))}},{key:"handleSettingsUpdate",value:function(){var e=this,r=this.internals.settings;return this.camera.minRatio=r.minCameraRatio,this.camera.maxRatio=r.maxCameraRatio,this.camera.enabledZooming=r.enableCameraZooming,this.camera.enabledPanning=r.enableCameraPanning,this.camera.enabledRotation=r.enableCameraRotation,r.cameraPanBoundaries?this.camera.constrainState=function(i){return e.cleanCameraState(i,r.cameraPanBoundaries&&te(r.cameraPanBoundaries)==="object"?r.cameraPanBoundaries:{})}:this.camera.constrainState=null,this.camera.setState(this.camera.getState()),this.mouseLayer.style.touchAction=r.gestureTarget==="graph"?"none":r.gestureTarget==="shared"?"pan-x pan-y":"auto",r.gestureTarget!=="shared"&&this.gestureHint&&(this.gestureHint.kill(),this.gestureHint=null),this.mouseCaptor.setSettings(this.internals.settings),this.touchCaptor.setSettings(this.internals.settings),this}},{key:"cleanCameraState",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=r.tolerance,o=i===void 0?0:i,s=r.boundaries,l=B({},e),u=s||this.nodeExtent,d=ae(u.x,2),c=d[0],h=d[1],f=ae(u.y,2),g=f[0],m=f[1],v=[this.graphToViewport({x:c,y:g},{cameraState:e}),this.graphToViewport({x:h,y:g},{cameraState:e}),this.graphToViewport({x:c,y:m},{cameraState:e}),this.graphToViewport({x:h,y:m},{cameraState:e})],p=1/0,b=-1/0,y=1/0,T=-1/0;v.forEach(function(R){var P=R.x,G=R.y;p=Math.min(p,P),b=Math.max(b,P),y=Math.min(y,G),T=Math.max(T,G)});var x=b-p,_=T-y,A=this.getDimensions(),E=A.width,L=A.height,w=0,F=0;if(x>=E?b<E-o?w=b-(E-o):p>o&&(w=p-o):b>E+o?w=b-(E+o):p<-o&&(w=p+o),_>=L?T<L-o?F=T-(L-o):y>o&&(F=y-o):T>L+o?F=T-(L+o):y<-o&&(F=y+o),w||F){var D=this.viewportToFramedGraph({x:0,y:0},{cameraState:e}),I=this.viewportToFramedGraph({x:w,y:F},{cameraState:e});w=I.x-D.x,F=I.y-D.y,l.x+=w,l.y+=F}return l}},{key:"refreshMatrices",value:function(){var e=this.camera.getState(),r=this.getDimensions(),i=this.getGraphDimensions(),o=this.getStagePadding();this.matrix=Ve(e,r,i,o),this.invMatrix=Ve(e,r,i,o,!0),this.correctionRatio=ei(this.matrix,e,r),this.graphToViewportRatio=this.getGraphToViewportRatio()}},{key:"processData",value:function(){var e;this.emit("beforeProcess"),(e=this.internals.attachmentManager)===null||e===void 0||e.clear();var r=B({},this.pickingState.idsByKind),i=this.processNodes();(this.pendingProcess==="full"||i)&&this.processEdges(),Fc(this.pickingState,this.internals),Ic(r,this.pickingState.idsByKind)||this.internals.hoverResolver.invalidate(),this.pendingProcess="none",this.emit("afterProcess")}},{key:"render",value:function(){var e=this;if(this.contextLost)return this;this.emit("beforeRender");var r=function(){return e.internals.hoverResolver.frameRendered(),e.emit("afterRender"),e};if(this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null),this.resize(),(this.pendingProcess!=="none"||this.needToRefreshState)&&(this.labelRenderer.labelsDirty=!0),this.pendingProcess!=="none"&&this.processData(),this.needToRefreshState&&this.refreshState(),this.needToRefreshState=!1,this.stateManager.clearDirtyTracking(),this.pendingProcess!=="none"&&this.processData(),this.clear(),this.resetWebGLTexture(),!this.internals.graph.order)return r();var i=this.mouseCaptor,o=this.camera.isAnimating()||i.isMoving||i.draggedEvents||i.currentWheelDirection;if(this.refreshMatrices(),this.frameId++,this.internals.settings.renderEdgeLabels&&!this.internals.edgeLabelProgram){var s=this.createEdgeLabelPrograms(),l=s.labelProgram,u=s.labelBackgroundProgram;this.internals.edgeLabelProgram=l,this.internals.edgeLabelBackgroundProgram=u}var d=this.internals.settings.DEBUG_logRenderStats?this.getDebugPrograms():null;d&&d.forEach(function(q){return q.resetDebugStats()}),this.internals.settings.DEBUG_gpuTimerQueries&&this.beginGpuTimerQuery(),this.internals.nodeFrameTexture.ensureCapacity(this.internals.nodeDataTexture.getCapacity()),this.internals.edgeFrameTexture.ensureCapacity(this.internals.edgeDataTexture.getCapacity());var c=this.getRenderParams(),h=me.edge.writesPickingThisFrame(this.internals)?c:B(B({},c),{},{pickingFrameBuffer:null});this.labelRenderer.resetFrame();var f=this.webGLContext,g=Math.ceil(this.width*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio),m=Math.ceil(this.height*this.internals.pixelRatio/this.internals.settings.pickingDownSizingRatio);if(f.bindFramebuffer(f.FRAMEBUFFER,this.pickingFrameBuffer),f.viewport(0,0,g,m),f.clear(f.COLOR_BUFFER_BIT|f.DEPTH_BUFFER_BIT),f.bindFramebuffer(f.FRAMEBUFFER,null),f.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),f.clear(f.COLOR_BUFFER_BIT|f.DEPTH_BUFFER_BIT),this.internals.nodeDataTexture.upload(),this.internals.edgeDataTexture.upload(),this.nodeProgram.uploadLayerTexture(),this.edgeProgram.uploadAttributeTexture(),this.emit("afterTexturesUpload"),this.internals.nodeDataTexture.bind(zo),this.internals.edgeDataTexture.bind(Bo),this.edgeFramePass.run(c,this.internals.edgeFrameTexture,this.internals.edgeDataTexture.getHighWaterMark(),this.edgeProgram.getAttributeTexture(),this.nodeProgram.getAttributeTexture()),this.internals.edgeFrameTexture.bind(Go),this.internals.settings.renderLabels){this.labelRenderer.computeDisplayedNodeLabels();var v=this.labelRenderer.buildFramePassPoints(),p=v.data,b=v.count;this.nodeFramePass.run(p,b,this.internals.nodeFrameTexture,c,this.nodeProgram.getAttributeTexture()),this.internals.nodeFrameTexture.bind(Mo)}this.internals.settings.renderEdgeLabels&&this.labelRenderer.computeDisplayedEdgeLabels();var y=z(this.customLayerPrograms.values()),T;try{for(y.s();!(T=y.n()).done;){var x=T.value.program;x.preRender&&x.preRender(c)}}catch(q){y.e(q)}finally{y.f()}f.bindFramebuffer(f.FRAMEBUFFER,null),f.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio),f.blendFunc(f.ONE,f.ONE_MINUS_SRC_ALPHA);var _=!this.internals.settings.hideLabelsOnMove||!o,A=z(this.depthLayers),E;try{for(A.s();!(E=A.n()).done;){var L=E.value,w=z(this.customLayerPrograms.values()),F;try{for(w.s();!(F=w.n()).done;){var D=F.value;D.depth===L&&D.program.render(c)}}catch(q){w.e(q)}finally{w.f()}var I=this.depthRanges.edges[L];if(I&&(!this.internals.settings.hideEdgesOnMove||!o)){var R=z(I),P;try{for(R.s();!(P=R.n()).done;){var G=P.value,N=G.offset,k=G.count;k>0&&this.edgeProgram.render(h,N,k)}}catch(q){R.e(q)}finally{R.f()}}this.internals.settings.renderEdgeLabels&&_&&(this.labelRenderer.renderEdgeLabelBackgrounds(me.edgeLabel.writesPickingThisFrame(this.internals)?c:B(B({},c),{},{pickingFrameBuffer:null}),L),this.labelRenderer.renderEdgeLabels(c,L)),this.labelRenderer.cacheAttachments(L),this.labelRenderer.renderBackdrops(B(B({},c),{},{pickingFrameBuffer:null}),L);var C=this.depthRanges.nodes[L];if(C){var M=z(C),$;try{for(M.s();!($=M.n()).done;){var H=$.value,Y=H.offset,j=H.count;j>0&&this.nodeProgram.render(c,Y,j)}}catch(q){M.e(q)}finally{M.f()}}_&&(this.labelRenderer.renderAttachments(B(B({},c),{},{pickingFrameBuffer:null}),L),this.labelRenderer.renderLabelBackgrounds(me.nodeLabel.writesPickingThisFrame(this.internals)?c:B(B({},c),{},{pickingFrameBuffer:null}),L),this.internals.settings.renderLabels&&this.labelRenderer.renderWebGLLabels(c,L))}}catch(q){A.e(q)}finally{A.f()}return this.labelRenderer.labelsDirty=!1,this.internals.settings.DEBUG_displayPickingLayer&&(f.bindFramebuffer(f.READ_FRAMEBUFFER,this.pickingFrameBuffer),f.bindFramebuffer(f.DRAW_FRAMEBUFFER,null),f.blitFramebuffer(0,0,g,m,0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio,f.COLOR_BUFFER_BIT,f.NEAREST)),this.internals.settings.DEBUG_gpuTimerQueries&&this.endGpuTimerQuery(),this.pollGpuTimerQueries(),d&&this.logRenderStats(d),r()}},{key:"getDebugPrograms",value:function(){return[this.nodeProgram,this.edgeProgram,this.internals.labelProgram,this.internals.edgeLabelProgram,this.internals.edgeLabelBackgroundProgram,this.internals.backdropProgram,this.internals.labelBackgroundProgram,this.internals.attachmentProgram].filter(function(e){return e!==null})}},{key:"logRenderStats",value:function(e){var r={},i=z(e),o;try{for(i.s();!(o=i.n()).done;){var s=o.value;r[s.constructor.name]=B({},s.debugStats)}}catch(l){i.e(l)}finally{i.f()}console.log("[sigma] DEBUG_logRenderStats: frame #".concat(this.frameId)),console.table(r)}},{key:"getGpuTimerExtension",value:function(){return this.gpuTimerExt===void 0&&(this.gpuTimerExt=this.webGLContext.getExtension("EXT_disjoint_timer_query_webgl2"),this.gpuTimerExt||console.warn("Sigma: DEBUG_gpuTimerQueries is enabled, but this browser/driver doesn't support EXT_disjoint_timer_query_webgl2.")),this.gpuTimerExt}},{key:"beginGpuTimerQuery",value:function(){var e=this.getGpuTimerExtension();if(e){var r=this.webGLContext,i=r.createQuery();i&&(r.beginQuery(e.TIME_ELAPSED_EXT,i),this.activeGpuTimerQuery=i)}}},{key:"endGpuTimerQuery",value:function(){!this.gpuTimerExt||!this.activeGpuTimerQuery||(this.webGLContext.endQuery(this.gpuTimerExt.TIME_ELAPSED_EXT),this.pendingGpuTimerQueries.push({query:this.activeGpuTimerQuery,frameId:this.frameId}),this.activeGpuTimerQuery=null)}},{key:"pollGpuTimerQueries",value:function(){if(this.pendingGpuTimerQueries.length!==0){var e=this.webGLContext,r=this.gpuTimerExt;if(!r){this.pendingGpuTimerQueries.forEach(function(c){var h=c.query;return e.deleteQuery(h)}),this.pendingGpuTimerQueries=[];return}var i=e.getParameter(r.GPU_DISJOINT_EXT),o=[],s=z(this.pendingGpuTimerQueries),l;try{for(s.s();!(l=s.n()).done;){var u=l.value;if(!e.getQueryParameter(u.query,e.QUERY_RESULT_AVAILABLE)){o.push(u);continue}if(!i){var d=e.getQueryParameter(u.query,e.QUERY_RESULT);console.log("[sigma] DEBUG_gpuTimerQueries: frame #".concat(u.frameId," GPU time = ").concat((d/1e6).toFixed(2),"ms"))}e.deleteQuery(u.query)}}catch(c){s.e(c)}finally{s.f()}this.pendingGpuTimerQueries=o}}},{key:"postEvaluateNode",value:function(e,r,i){var o=e;o.x===void 0&&(o.x=r.x),o.y===void 0&&(o.y=r.y),e.highlighted=i.isHighlighted;for(var s=0,l=this.nodeVariableEntries.length;s<l;s++){var u,d,c=ae(this.nodeVariableEntries[s],2),h=c[0],f=c[1];o[h]=(u=(d=o[h])!==null&&d!==void 0?d:r[h])!==null&&u!==void 0?u:f.default}}},{key:"addNode",value:function(e){var r=this.internals.graph.getNodeAttributes(e),i=this.stateManager.getNodeState(e),o=this.internals.nodeDataCache[e]||{};if(Ln(this.stylesDeclaration.nodes,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateNode(o,r,i),this.nodeReducer){var s=this.nodeReducer(e,o,r,i,this.stateManager.graphState,this.internals.graph);o=B(B({},o),s)}if(typeof o.x!="number"||typeof o.y!="number")throw new Error('Sigma: could not find a valid position (x, y) for node "'.concat(e,'". ')+"Provide coordinates via node attributes, styles, or a nodeReducer.");this.internals.nodeShapeMap?(!o.shape||!(o.shape in this.internals.nodeShapeMap))&&(o.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(o.shape=this.nodeShapeSlug),this.internals.nodeDataCache[e]=o,this.nodeGraphCoords[e]={x:o.x,y:o.y},nt(this.internals.nodesWithForcedLabels,e,Xe(o)),nt(this.internals.nodesWithBackdrop,e,Ca(o)),this.itemBuckets.nodes.set(e,o.depth)}},{key:"updateNode",value:function(e){this.addNode(e);var r=this.internals.nodeDataCache[e];this.normalizationFunction.applyTo(r)}},{key:"removeNode",value:function(e){Co(this.pickingState,"node",e),this.itemBuckets.nodes.remove(e),delete this.internals.nodeDataCache[e],delete this.nodeGraphCoords[e],delete this.nodeProgramIndex[e],this.internals.nodeDataTexture.free(e),this.internals.dragManager.removeNode(e),this.stateManager.removeNode(e),this.internals.nodesWithForcedLabels.delete(e),this.internals.nodesWithBackdrop.delete(e)}},{key:"postEvaluateEdge",value:function(e,r){for(var i=e,o=0,s=this.edgeVariableEntries.length;o<s;o++){var l,u,d=ae(this.edgeVariableEntries[o],2),c=d[0],h=d[1];i[c]=(l=(u=i[c])!==null&&u!==void 0?u:r[c])!==null&&l!==void 0?l:h.default}}},{key:"applyEdgeSpread",value:function(e,r,i){var o;if(!(i.parallelCount<=1)){var s=this.internals.graph.source(e),l=this.internals.graph.target(e),u=s===l,d=u?r.selfLoopPath||r.path:r.parallelPath||r.path,c=d?this.edgePathsByName.get(d):void 0;if(c!=null&&c.spread){var h=(o=r.parallelSpread)!==null&&o!==void 0?o:.25,f=c.spread.compute(i.parallelIndex,i.parallelCount,h);!u&&this.internals.graph.isDirected(e)&&s>l&&(f=-f),r[c.spread.variable]=f}}}},{key:"addEdge",value:function(e){var r=this.internals.graph.getEdgeAttributes(e),i=this.stateManager.getEdgeState(e),o={};if(Cn(this.stylesDeclaration.edges,r,i,this.stateManager.graphState,this.internals.graph,o),this.postEvaluateEdge(o,r),this.edgeReducer){var s=this.edgeReducer(e,o,r,i,this.stateManager.graphState,this.internals.graph);o=B(B({},o),s)}this.applyEdgeSpread(e,o,i),this.internals.edgeDataCache[e]=o,nt(this.internals.edgesWithForcedLabels,e,Xe(o)),this.itemBuckets.edges.set(e,o.depth)}},{key:"updateEdge",value:function(e){this.addEdge(e)}},{key:"removeEdge",value:function(e){this.itemBuckets.edges.remove(e),delete this.internals.edgeDataCache[e],delete this.edgeProgramIndex[e],delete this.edgeTextureIndexCache[e],this.internals.edgeDataTexture.free(e),Co(this.pickingState,"edge",e),this.stateManager.removeEdge(e),this.internals.edgesWithForcedLabels.delete(e)}},{key:"clearNodeIndices",value:function(){var e;this.labelRenderer.resetLabelGrid(),this.autoRescaleFrozen||(this.nodeExtent={x:[0,1],y:[0,1]}),this.internals.nodeDataCache={},this.nodeGraphCoords={},this.nodeProgramIndex={},(e=this.internals.nodeDataTexture)===null||e===void 0||e.clear(),this.internals.nodesWithForcedLabels.clear(),this.internals.nodesWithBackdrop.clear(),this.prevNodeVisibilities={},this.itemBuckets.nodes.clearAll(),this.depthRanges.nodes={},this.nodeBaseDepth={}}},{key:"clearEdgeIndices",value:function(){var e;this.internals.edgeDataCache={},this.edgeProgramIndex={},this.edgeTextureIndexCache={},(e=this.internals.edgeDataTexture)===null||e===void 0||e.clear(),this.internals.edgesWithForcedLabels.clear(),this.itemBuckets.edges.clearAll(),this.depthRanges.edges={},this.edgeBaseDepth={},this.edgeGroups.clear()}},{key:"clearIndices",value:function(){this.clearEdgeIndices(),this.clearNodeIndices()}},{key:"clearNodeState",value:function(){this.labelRenderer.resetFrame(),this.internals.nodesWithBackdrop.clear(),this.internals.dragManager.clear(),this.autoRescaleFrozen=!1,this.stateManager.clearNodes(),Kn(this.pickingState,"node")}},{key:"clearEdgeState",value:function(){this.labelRenderer.clearEdgeLabels(),this.stateManager.clearEdges(),Kn(this.pickingState,"edge")}},{key:"clearState",value:function(){this.clearEdgeState(),this.clearNodeState(),this.stateManager.resetGraphState()}},{key:"addNodeToProgram",value:function(e,r){var i,o=this.internals.nodeDataCache[e];this.internals.nodeDataTexture.allocate(e),(i=this.internals.nodeDataTexture).updateNode.apply(i,[e,o.x,o.y,o.size,this.getNodeShapeId(o)].concat(V(kn(o)),[o.color]));var s=this.internals.nodeDataTexture.getIndex(e);this.nodeProgram.process(lt(this.pickingState,"node",e),r,o,s),this.nodeProgramIndex[e]=r}},{key:"addEdgeToProgram",value:function(e,r){var i,o,s=this.internals.edgeDataCache[e],l=this.internals.graph.source(e),u=this.internals.graph.target(e),d=this.internals.edgeDataTexture.allocate(e);this.edgeTextureIndexCache[e]=d;var c=l===u,h=!c&&((i=(o=this.stateManager.getEdgeState(e))===null||o===void 0?void 0:o.parallelCount)!==null&&i!==void 0?i:1)>1,f=this.edgeProgram.resolveEdgeIds(s,c,h),g=f.pathId,m=f.headId,v=f.tailId,p=f.headLengthRatio,b=f.tailLengthRatio;this.internals.edgeDataTexture.updateEdge(e,this.internals.nodeDataTexture.getIndex(l),this.internals.nodeDataTexture.getIndex(u),s.size,p,b,g,m,v),this.edgeProgram.process(lt(this.pickingState,"edge",e),r,this.internals.nodeDataCache[l],this.internals.nodeDataCache[u],s,d),this.edgeProgramIndex[e]=r}},{key:"getRenderParams",value:function(){return{frameId:this.frameId,matrix:this.matrix,invMatrix:this.invMatrix,width:this.width,height:this.height,pixelRatio:this.internals.pixelRatio,zoomRatio:this.camera.ratio,cameraAngle:this.camera.angle,sizeRatio:1/this.scaleSize(),correctionRatio:this.correctionRatio,downSizingRatio:this.internals.settings.pickingDownSizingRatio,minEdgeThickness:this.internals.settings.minEdgeThickness,antiAliasingFeather:this.internals.settings.antiAliasingFeather,nodePickingPadding:this.internals.settings.nodePickingPadding,edgePickingPadding:this.internals.settings.edgePickingPadding,labelPickingPadding:this.internals.settings.labelPickingPadding,nodeDataTextureUnit:zo,nodeDataTextureWidth:this.internals.nodeDataTexture.getTextureWidth(),nodeFrameTextureUnit:Mo,nodeFrameTextureWidth:this.internals.nodeFrameTexture.getTextureWidth(),edgeDataTextureUnit:Bo,edgeDataTextureWidth:this.internals.edgeDataTexture.getTextureWidth(),edgeFrameTextureUnit:Go,edgeFrameTextureWidth:this.internals.edgeFrameTexture.getTextureWidth(),pickingFrameBuffer:this.pickingFrameBuffer,labelPixelSnapping:this.internals.settings.labelPixelSnapping?1:0}}},{key:"getStagePadding",value:function(){var e=this.internals.settings,r=e.stagePadding,i=e.autoRescale;return i&&r||0}},{key:"getLayerElement",value:function(e){if(e==="mouse")return this.mouseLayer;var r=this.extraElements[e];if(!r)throw new Error('Sigma: layer "'.concat(e,'" does not exist'));return r}},{key:"initWebGLContext",value:function(){var e=this,r=this.createWebGLContext("stage");this.stageCanvas=this.extraElements.stage,this.webGLContext=r,this.stageCanvas.addEventListener("webglcontextlost",function(i){e.webGLContext&&(i.preventDefault(),e.contextLost=!0,e.renderFrame&&(cancelAnimationFrame(e.renderFrame),e.renderFrame=null),e.gpuTimerExt=void 0,e.activeGpuTimerQuery=null,e.pendingGpuTimerQueries=[],e.emit("webglContextLost"))}),this.stageCanvas.addEventListener("webglcontextrestored",function(){requestAnimationFrame(function(){e.webGLContext&&!e.webGLContext.isContextLost()&&e.restoreWebGLContext()})}),this.initPickingFramebuffer()}},{key:"initPickingFramebuffer",value:function(){var e=this.webGLContext,r=e.createFramebuffer();if(!r)throw new Error("Sigma: cannot create picking frame buffer");e.bindFramebuffer(e.FRAMEBUFFER,r);var i=e.createTexture();if(!i)throw new Error("Sigma: cannot create picking texture");e.bindTexture(e.TEXTURE_2D,i),e.texImage2D(e.TEXTURE_2D,0,e.RGBA,1,1,0,e.RGBA,e.UNSIGNED_BYTE,null),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MIN_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_MAG_FILTER,e.NEAREST),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_S,e.CLAMP_TO_EDGE),e.texParameteri(e.TEXTURE_2D,e.TEXTURE_WRAP_T,e.CLAMP_TO_EDGE),e.framebufferTexture2D(e.FRAMEBUFFER,e.COLOR_ATTACHMENT0,e.TEXTURE_2D,i,0);var o=e.createRenderbuffer();if(!o)throw new Error("Sigma: cannot create picking depth buffer");if(e.bindRenderbuffer(e.RENDERBUFFER,o),e.renderbufferStorage(e.RENDERBUFFER,e.DEPTH_COMPONENT16,1,1),e.framebufferRenderbuffer(e.FRAMEBUFFER,e.DEPTH_ATTACHMENT,e.RENDERBUFFER,o),e.checkFramebufferStatus(e.FRAMEBUFFER)!==e.FRAMEBUFFER_COMPLETE)throw new Error("Sigma: picking framebuffer is not complete");e.bindFramebuffer(e.FRAMEBUFFER,null),this.pickingFrameBuffer=r,this.pickingTexture=i,this.pickingDepthBuffer=o}},{key:"initPrograms",value:function(e,r){var i=this,o=this.webGLContext,s=fo(o,this.pickingFrameBuffer,i,e?.nodes,r.antialiasNodes),l=s.nodeProgram,u=s.labelProgram,d=s.backdropProgram,c=s.labelBackgroundProgram,h=s.attachmentProgram,f=s.framePass,g=s.shapeSlug,m=s.shapeNameToIndex,v=s.shapeGlobalIds,p=s.variables;this.nodeProgram=l,this.nodeFramePass=f,this.nodeVariableEntries=Object.entries(p),g&&(this.nodeShapeSlug=g);var b=go(o,this.pickingFrameBuffer,i,e?.edges,r.antialiasEdges,e?.nodes),y=b.edgeProgram,T=b.createLabelPrograms,x=b.framePass,_=b.variables,A=b.paths;return this.createEdgeLabelPrograms=T,this.edgeProgram=y,this.edgeFramePass=x,this.edgeVariableEntries=Object.entries(_),this.edgePathsByName=new Map(A.map(function(E){return[E.name,E]})),{labelProgram:u,edgeLabelProgram:null,edgeLabelBackgroundProgram:null,backdropProgram:d,labelBackgroundProgram:c,attachmentProgram:h,nodeShapeMap:m??null,nodeGlobalShapeIds:v??null}}},{key:"restoreWebGLContext",value:function(){var e,r,i,o,s,l=this.internals;this.initPickingFramebuffer(),(e=l.nodeDataTexture)===null||e===void 0||e.restore(),(r=l.nodeFrameTexture)===null||r===void 0||r.restore(),(i=l.edgeDataTexture)===null||i===void 0||i.restore(),(o=l.edgeFrameTexture)===null||o===void 0||o.restore(),(s=l.attachmentManager)===null||s===void 0||s.restore(),l.hoverResolver.reset(),Object.assign(l,this.initPrograms(l.primitives,l.settings));var u=z(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var c=d.value;c.program.kill(),c.program=c.factory(this.webGLContext)}}catch(h){u.e(h)}finally{u.f()}this.contextLost=!1,this.emit("webglContextRestored"),this.refresh()}},{key:"createLayer",value:function(e,r){var i=arguments.length>2&&arguments[2]!==void 0?arguments[2]:{};if(this.extraElements[e])throw new Error('Sigma: a layer named "'.concat(e,'" already exists'));var o=Et(r,{position:"absolute"},{class:"sigma-".concat(e)});return i.style&&Object.assign(o.style,i.style),this.extraElements[e]=o,"beforeLayer"in i&&i.beforeLayer?this.getLayerElement(i.beforeLayer).before(o):"afterLayer"in i&&i.afterLayer?this.getLayerElement(i.afterLayer).after(o):this.container.appendChild(o),o}},{key:"createCanvas",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.createLayer(e,"canvas",r)}},{key:"createWebGLContext",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=this.createCanvas(e,r);r.hidden&&i.remove();var o=i.getContext("webgl2",B({preserveDrawingBuffer:!1,antialias:!1,depth:!0},r));if(!o)throw new Error("Sigma: WebGL 2 is not supported by your browser. Please use a modern browser (Chrome 56+, Firefox 51+, Safari 15+, Edge 79+).");return o.blendFunc(o.ONE,o.ONE_MINUS_SRC_ALPHA),o}},{key:"killLayer",value:function(e){if(e==="stage"||e==="mouse")throw new Error('Sigma: cannot kill built-in layer "'.concat(e,'"'));var r=this.extraElements[e];if(!r)throw new Error("Sigma: cannot kill layer ".concat(e,", which does not exist"));return r.remove(),delete this.extraElements[e],this}},{key:"getWebGLContext",value:function(){if(!this.webGLContext)throw new Error("Sigma: WebGL context is not available");return this.webGLContext}},{key:"getNodeDataTexture",value:function(){if(!this.internals.nodeDataTexture)throw new Error("Sigma: node data texture is not available");return this.internals.nodeDataTexture}},{key:"getNormalizationFunction",value:function(){return this.normalizationFunction}},{key:"addCustomLayerProgram",value:function(e,r,i){var o;if(!this.depthLayers.includes(r))throw new Error('Sigma: cannot add custom layer program at depth "'.concat(r,'", ')+"it must be declared in primitives.depthLayers. Current layers: ".concat(this.depthLayers.join(", ")));(o=this.customLayerPrograms.get(e))===null||o===void 0||o.program.kill();var s=i(this.webGLContext);return this.customLayerPrograms.set(e,{depth:r,factory:i,program:s}),s.cacheData&&s.cacheData(),this.scheduleRender(),this}},{key:"removeCustomLayerProgram",value:function(e){var r=this.customLayerPrograms.get(e);return r&&(r.program.kill(),this.customLayerPrograms.delete(e),this.scheduleRender()),this}},{key:"getCamera",value:function(){return this.camera}},{key:"setCamera",value:function(e){return this.camera.cancelAnimation(),this.unbindCameraHandlers(),this.camera=e,this.bindCameraHandlers(),this.scheduleRender(),this}},{key:"getContainer",value:function(){return this.container}},{key:"getGraph",value:function(){return this.internals.graph}},{key:"setGraph",value:function(e){return e===this.internals.graph?this:(this.stateManager.pruneNodes(function(r){return e.hasNode(r)}),this.stateManager.pruneEdges(function(r){return e.hasEdge(r)}),this.unbindGraphHandlers(),this.checkEdgesEventsFrame!==null&&(cancelAnimationFrame(this.checkEdgesEventsFrame),this.checkEdgesEventsFrame=null),this.internals.graph=e,this.autoRescaleFrozen=!1,this.bindGraphHandlers(),this.refresh(),this)}},{key:"getMouseCaptor",value:function(){return this.mouseCaptor}},{key:"getTouchCaptor",value:function(){return this.touchCaptor}},{key:"getDimensions",value:function(){return{width:this.width,height:this.height}}},{key:"getGraphDimensions",value:function(){var e=this.customBBox||this.nodeExtent;return{width:e.x[1]-e.x[0]||1,height:e.y[1]-e.y[0]||1}}},{key:"getNodeDisplayData",value:function(e){var r=this.internals.nodeDataCache[e];return r?Object.assign({},r):void 0}},{key:"getEdgeDisplayData",value:function(e){var r=this.internals.edgeDataCache[e];return r?Object.assign({},r):void 0}},{key:"getNodeState",value:function(e){return this.stateManager.getNodeState(e)}},{key:"getEdgeState",value:function(e){return this.stateManager.getEdgeState(e)}},{key:"getGraphState",value:function(){return this.stateManager.getGraphState()}},{key:"setNodeState",value:function(e,r){return this.stateManager.setNodeState(e,r),this}},{key:"setEdgeState",value:function(e,r){return this.stateManager.setEdgeState(e,r),this}},{key:"setGraphState",value:function(e){return this.stateManager.setGraphState(e),this}},{key:"_setPanning",value:function(e){this.stateManager.setGraphState({isPanning:e})}},{key:"_setZooming",value:function(e){this.stateManager.setGraphState({isZooming:e})}},{key:"_hasNodeDrag",value:function(){var e=this.internals.dragManager;return!!(e.pendingNode||e.session)}},{key:"_showGestureHint",value:function(e){var r=this.internals.settings;r.gestureTarget==="shared"&&(this.gestureHint=this.gestureHint||new Yc(this.container),this.gestureHint.show(e,r))}},{key:"setNodesState",value:function(e,r){return this.stateManager.setNodesState(e,r),this}},{key:"setEdgesState",value:function(e,r){return this.stateManager.setEdgesState(e,r),this}},{key:"updateContainerCursor",value:function(){var e=this.stateManager.hovered,r=this.resolvedStageStyle.cursor||"";this.container.style.cursor=e&&Nc(this.internals,e)||r}},{key:"refreshStageStyle",value:function(){this.resolvedStageStyle=wa(this.stylesDeclaration.stage,this.stateManager.graphState),this.resolvedStageStyle.background!==void 0&&(this.container.style.backgroundColor=this.resolvedStageStyle.background),this.updateContainerCursor()}},{key:"getNodeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedNodeLabels)}},{key:"getEdgeDisplayedLabels",value:function(){return new Set(this.labelRenderer.displayedEdgeLabels)}},{key:"getSettings",value:function(){return B({},this.internals.settings)}},{key:"getSetting",value:function(e){return this.internals.settings[e]}},{key:"getStyles",value:function(){return B({},this.stylesDeclaration)}},{key:"getPrimitives",value:function(){return B({},this.internals.primitives)}},{key:"setSetting",value:function(e,r){return this.internals.settings[e]=r,In(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"updateSetting",value:function(e,r){return this.setSetting(e,r(this.internals.settings[e])),this}},{key:"setSettings",value:function(e){return this.internals.settings=B(B({},this.internals.settings),e),In(this.internals.settings),this.handleSettingsUpdate(),this.scheduleRefresh(),this}},{key:"resize",value:function(e){var r=this.width,i=this.height;if(this.width=this.container.offsetWidth,this.height=this.container.offsetHeight,this.internals.pixelRatio=qt(),this.width===0)if(this.internals.settings.allowInvalidContainer)this.width=1;else throw new Error("Sigma: Container has no width. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(this.height===0)if(this.internals.settings.allowInvalidContainer)this.height=1;else throw new Error("Sigma: Container has no height. You can set the allowInvalidContainer setting to true to stop seeing this error.");if(!e&&r===this.width&&i===this.height)return this;this.labelRenderer.labelsDirty=!0;for(var o=0,s=[this.mouseLayer].concat(V(Object.values(this.extraElements)));o<s.length;o++){var l=s[o];l.style.width=this.width+"px",l.style.height=this.height+"px"}return this.webGLContext&&(this.stageCanvas.setAttribute("width",this.width*this.internals.pixelRatio+"px"),this.stageCanvas.setAttribute("height",this.height*this.internals.pixelRatio+"px"),this.webGLContext.viewport(0,0,this.width*this.internals.pixelRatio,this.height*this.internals.pixelRatio)),this.emit("resize"),this}},{key:"clear",value:function(){return this.emit("beforeClear"),this.webGLContext.bindFramebuffer(WebGLRenderingContext.FRAMEBUFFER,null),this.webGLContext.clear(WebGLRenderingContext.COLOR_BUFFER_BIT),this.emit("afterClear"),this}},{key:"scheduleStateRefresh",value:function(){this.needToRefreshState=!0,this.scheduleRender()}},{key:"refreshState",value:function(){var e=this,r;this.stateManager.flushGraphStateFlags();var i=this.stateManager.graphStateChanged&&this.internals.nodeStyleAnalysis.dependency==="graph-state",o=this.stateManager.graphStateChanged&&this.edgeStyleAnalysis.dependency==="graph-state",s=!1;if(i)this.internals.graph.forEachNode(function(g){e.refreshNodeState(g)&&(s=!0)});else if(this.internals.nodeStyleAnalysis.dependency!=="static"){var l=z(this.stateManager.dirtyNodes),u;try{for(l.s();!(u=l.n()).done;){var d=u.value;this.refreshNodeState(d)&&(s=!0)}}catch(g){l.e(g)}finally{l.f()}}if(o)this.internals.graph.forEachEdge(function(g){e.refreshEdgeState(g)&&(s=!0)});else if(this.edgeStyleAnalysis.dependency!=="static"){var c=z(this.stateManager.dirtyEdges),h;try{for(c.s();!(h=c.n()).done;){var f=h.value;this.refreshEdgeState(f)&&(s=!0)}}catch(g){c.e(g)}finally{c.f()}}this.stateManager.graphStateChanged&&(r=this.stylesDeclaration)!==null&&r!==void 0&&r.stage&&this.refreshStageStyle(),this.stateManager.clearDirtyTracking(),s&&(this.pendingProcess="full")}},{key:"refreshNodeState",value:function(e){var r=this.internals.nodeDataCache[e];if(!r||this.nodeReducer){var i,o,s,l,u=(i=this.internals.nodeDataCache[e])===null||i===void 0?void 0:i.depth,d=(o=this.internals.nodeDataCache[e])===null||o===void 0?void 0:o.zIndex,c=(s=this.internals.nodeDataCache[e])===null||s===void 0?void 0:s.labelAttachment;this.updateNode(e);var h=this.internals.nodeDataCache[e];this.internals.attachmentManager&&h.labelAttachment!==c&&this.internals.attachmentManager.invalidateNode(e);var f;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&h.shape&&h.shape in this.internals.nodeShapeMap?f=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[h.shape]]:f=Jt(h.shape||"circle"),(l=this.internals.nodeDataTexture).updateNode.apply(l,[e,h.x,h.y,h.size,f].concat(V(kn(h)),[h.color])),u&&h.depth!==u&&this.updateNodeDepthRanges(e,u,h.depth);var g=this.nodeProgramIndex[e];return g!==void 0&&(this.addNodeToProgram(e,g),this.nodeProgram.invalidateBuffers()),d!==void 0&&h.zIndex!==d}var m=this.internals.graph.getNodeAttributes(e),v=this.stateManager.getNodeState(e),p=r.size,b=r.shape,y=r.depth,T=r.zIndex,x=r.labelAttachment,_=r.rotationAlignment,A=r.labelRotationAlignment,E=r.color;Ln(this.stylesDeclaration.nodes,m,v,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateNode(r,m,v);var L=this.nodeGraphCoords[e],w=r.x!==L.x||r.y!==L.y;if(w&&(L.x=r.x,L.y=r.y),this.normalizationFunction.applyTo(r),this.internals.nodeShapeMap?(!r.shape||!(r.shape in this.internals.nodeShapeMap))&&(r.shape=Object.keys(this.internals.nodeShapeMap)[0]):this.nodeShapeSlug&&(r.shape=this.nodeShapeSlug),this.internals.attachmentManager&&r.labelAttachment!==x&&this.internals.attachmentManager.invalidateNode(e),nt(this.internals.nodesWithForcedLabels,e,Xe(r)),nt(this.internals.nodesWithBackdrop,e,Ca(r)),w||r.size!==p||r.shape!==b||r.rotationAlignment!==_||r.labelRotationAlignment!==A||r.color!==E){var F,D;this.internals.nodeShapeMap&&this.internals.nodeGlobalShapeIds&&r.shape&&r.shape in this.internals.nodeShapeMap?D=this.internals.nodeGlobalShapeIds[this.internals.nodeShapeMap[r.shape]]:D=Jt(r.shape||"circle"),(F=this.internals.nodeDataTexture).updateNode.apply(F,[e,r.x,r.y,r.size,D].concat(V(kn(r)),[r.color]))}this.itemBuckets.nodes.set(e,r.depth),r.depth!==y&&this.updateNodeDepthRanges(e,y,r.depth);var I=this.nodeProgramIndex[e];return I!==void 0&&(this.addNodeToProgram(e,I),this.nodeProgram.invalidateBuffers()),r.zIndex!==T}},{key:"refreshEdgeState",value:function(e){var r=this.internals.edgeDataCache[e];if(!r||this.edgeReducer){var i=r?.depth,o=r?.zIndex;this.updateEdge(e);var s=this.internals.edgeDataCache[e];i&&s.depth!==i&&this.updateEdgeDepthRanges(e,i,s.depth);var l=this.edgeProgramIndex[e];return l!==void 0&&(this.addEdgeToProgram(e,l),this.edgeProgram.invalidateBuffers()),o!==void 0&&s.zIndex!==o}var u=this.internals.graph.getEdgeAttributes(e),d=this.stateManager.getEdgeState(e),c=r.depth,h=r.zIndex,f=r.size,g=r.path,m=r.selfLoopPath,v=r.parallelPath,p=r.head,b=r.tail;Cn(this.stylesDeclaration.edges,u,d,this.stateManager.graphState,this.internals.graph,r),this.postEvaluateEdge(r,u),this.applyEdgeSpread(e,r,d),nt(this.internals.edgesWithForcedLabels,e,Xe(r)),this.itemBuckets.edges.set(e,r.depth),r.depth!==c&&this.updateEdgeDepthRanges(e,c,r.depth);var y=this.edgeProgramIndex[e];if(y!==void 0){var T=r.size!==f||r.path!==g||r.selfLoopPath!==m||r.parallelPath!==v||r.head!==p||r.tail!==b;if(T)this.addEdgeToProgram(e,y),this.edgeProgram.invalidateBuffers();else{var x=this.internals.graph.source(e),_=this.internals.graph.target(e),A=this.internals.nodeDataCache[x],E=this.internals.nodeDataCache[_],L=this.edgeTextureIndexCache[e];this.edgeProgram.process(lt(this.pickingState,"edge",e),y,A,E,r,L),this.edgeProgram.invalidateBuffers()}}return r.zIndex!==h}},{key:"refresh",value:function(e){var r=this,i=e?.skipIndexation!==void 0?e?.skipIndexation:!1,o=e?.schedule!==void 0?e.schedule:!1,s=!e||!e.partialGraph;if(s)this.clearEdgeIndices(),this.clearNodeIndices(),this.internals.graph.forEachNode(function(A){return r.addNode(A)}),this.edgeGroups.rebuild(),this.internals.graph.forEachEdge(function(A){return r.addEdge(A)}),this.pendingProcess="full";else{for(var l,u,d=((l=e.partialGraph)===null||l===void 0?void 0:l.nodes)||[],c=0,h=d?.length||0;c<h;c++){var f,g,m=""+d[c],v=(f=this.internals.nodeDataCache[m])===null||f===void 0?void 0:f.labelAttachment;if(this.updateNode(m),this.internals.attachmentManager&&((g=this.internals.nodeDataCache[m])!==null&&g!==void 0&&g.labelAttachment||v)&&this.internals.attachmentManager.invalidateNode(m),i){var p=this.nodeProgramIndex[m];if(p===void 0)throw new Error('Sigma: node "'.concat(m,`" can't be repaint`));this.addNodeToProgram(m,p)}}i&&d.length>0&&(this.nodeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0);for(var b=(e==null||(u=e.partialGraph)===null||u===void 0?void 0:u.edges)||[],y=0,T=b.length;y<T;y++){var x=""+b[y];if(this.updateEdge(x),i){var _=this.edgeProgramIndex[x];if(_===void 0)throw new Error('Sigma: edge "'.concat(x,`" can't be repaint`));this.addEdgeToProgram(x,_)}}i&&b.length>0&&(this.edgeProgram.invalidateBuffers(),this.labelRenderer.labelsDirty=!0),!i&&this.pendingProcess!=="full"&&(this.pendingProcess=b.length>0?"full":"nodes")}return o?this.scheduleRender():this.render(),this}},{key:"scheduleRender",value:function(){var e=this;return this.renderFrame||(this.renderFrame=requestAnimationFrame(function(){e.render()})),this}},{key:"scheduleRefresh",value:function(e){return this.refresh(B(B({},e),{},{schedule:!0}))}},{key:"getViewportZoomedState",value:function(e,r){var i=this.camera.getState(),o=i.ratio,s=i.angle,l=i.x,u=i.y,d=this.internals.settings,c=d.minCameraRatio,h=d.maxCameraRatio;typeof h=="number"&&(r=Math.min(r,h)),typeof c=="number"&&(r=Math.max(r,c));var f=r/o,g={x:this.width/2,y:this.height/2},m=this.viewportToFramedGraph(e),v=this.viewportToFramedGraph(g);return{angle:s,x:(m.x-v.x)*(1-f)+l,y:(m.y-v.y)*(1-f)+u,ratio:r}}},{key:"viewRectangle",value:function(){var e=this.viewportToFramedGraph({x:0,y:0}),r=this.viewportToFramedGraph({x:this.width,y:0}),i=this.viewportToFramedGraph({x:0,y:this.height});return{x1:e.x,y1:e.y,x2:r.x,y2:r.y,height:r.y-i.y}}},{key:"framedGraphToViewport",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?Ve(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding()):this.matrix),s=bt(o,e);return{x:(1+s.x)*this.width/2,y:(1-s.y)*this.height/2}}},{key:"viewportToFramedGraph",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{},i=!!r.cameraState||!!r.viewportDimensions||!!r.graphDimensions||!!r.padding,o=r.matrix||(i?Ve(r.cameraState||this.camera.getState(),r.viewportDimensions||this.getDimensions(),r.graphDimensions||this.getGraphDimensions(),r.padding||this.getStagePadding(),!0):this.invMatrix),s=bt(o,{x:e.x/this.width*2-1,y:1-e.y/this.height*2});return isNaN(s.x)&&(s.x=0),isNaN(s.y)&&(s.y=0),s}},{key:"viewportToGraph",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.normalizationFunction.inverse(this.viewportToFramedGraph(e,r))}},{key:"graphToViewport",value:function(e){var r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:{};return this.framedGraphToViewport(this.normalizationFunction(e),r)}},{key:"graphToFramedGraph",value:function(e){return this.normalizationFunction(e)}},{key:"framedGraphToGraph",value:function(e){return this.normalizationFunction.inverse(e)}},{key:"getGraphToViewportRatio",value:function(){var e={x:0,y:0},r={x:1,y:1},i=Math.sqrt(Math.pow(e.x-r.x,2)+Math.pow(e.y-r.y,2)),o=this.graphToViewport(e),s=this.graphToViewport(r),l=Math.sqrt(Math.pow(o.x-s.x,2)+Math.pow(o.y-s.y,2));return l/i}},{key:"computeNodeExtent",value:function(){var e=this.nodeGraphCoords,r=Object.keys(e);if(!r.length)return{x:[0,1],y:[0,1]};for(var i=1/0,o=-1/0,s=1/0,l=-1/0,u=0,d=r.length;u<d;u++){var c=e[r[u]],h=c.x,f=c.y;h<i&&(i=h),h>o&&(o=h),f<s&&(s=f),f>l&&(l=f)}return{x:[i,o],y:[s,l]}}},{key:"getBBox",value:function(){return this.nodeExtent}},{key:"getCustomBBox",value:function(){return this.customBBox}},{key:"setCustomBBox",value:function(e){return this.customBBox=e,this.scheduleRender(),this}},{key:"kill",value:function(){var e,r,i,o,s;this.emit("kill"),this.removeAllListeners(),this.camera.cancelAnimation(),this.unbindCameraHandlers(),window.removeEventListener("resize",this.activeListeners.handleResize),this.mouseCaptor.kill(),this.touchCaptor.kill(),this.internals.hoverResolver.kill(),this.unbindGraphHandlers(),this.clearIndices(),this.clearState(),this.internals.nodeDataCache={},this.internals.edgeDataCache={},this.renderFrame&&(cancelAnimationFrame(this.renderFrame),this.renderFrame=null);for(var l=this.container;l.firstChild;)l.removeChild(l.firstChild);this.nodeProgram.kill(),this.nodeFramePass.kill(),this.edgeProgram.kill(),this.edgeFramePass.kill(),this.internals.labelProgram.kill(),(e=this.internals.edgeLabelProgram)===null||e===void 0||e.kill(),(r=this.internals.edgeLabelBackgroundProgram)===null||r===void 0||r.kill(),this.internals.backdropProgram.kill(),this.internals.labelBackgroundProgram.kill(),(i=this.internals.attachmentProgram)===null||i===void 0||i.kill(),(o=this.internals.attachmentManager)===null||o===void 0||o.kill(),this.internals.attachmentProgram=null,this.internals.attachmentManager=null;var u=z(this.customLayerPrograms.values()),d;try{for(u.s();!(d=u.n()).done;){var c=d.value.program;c.kill()}}catch(g){u.e(g)}finally{u.f()}if(this.customLayerPrograms.clear(),this.sdfAtlas&&(this.sdfAtlas=null),this.internals.nodeDataTexture&&(this.internals.nodeDataTexture.kill(),this.internals.nodeDataTexture=null),this.internals.nodeFrameTexture&&(this.internals.nodeFrameTexture.kill(),this.internals.nodeFrameTexture=null),this.internals.edgeDataTexture&&(this.internals.edgeDataTexture.kill(),this.internals.edgeDataTexture=null),this.internals.edgeFrameTexture&&(this.internals.edgeFrameTexture.kill(),this.internals.edgeFrameTexture=null),this.webGLContext){var h;(h=this.webGLContext.getExtension("WEBGL_lose_context"))===null||h===void 0||h.loseContext(),this.webGLContext=null}(s=this.gestureHint)===null||s===void 0||s.kill(),this.gestureHint=null,this.mouseLayer.remove();for(var f in this.extraElements)this.extraElements[f].remove();this.extraElements={}}},{key:"scaleSize",value:function(){var e=arguments.length>0&&arguments[0]!==void 0?arguments[0]:1,r=arguments.length>1&&arguments[1]!==void 0?arguments[1]:this.camera.ratio;return e/this.internals.settings.zoomToSizeRatioFunction(r)*(this.getSetting("itemSizesReference")==="positions"?r*this.graphToViewportRatio:1)}},{key:"getStageCanvas",value:function(){return this.stageCanvas}},{key:"getMouseLayer",value:function(){return this.mouseLayer}}])})(Xt),Qa=rh;var Gf=Je(ct());var Jn=7,$o=4,Ja=13,ih=1.6,oh=.3,sh=.5,Vo=.004,lh=3,uh="Segoe UI Variable Text, Segoe UI, system-ui, sans-serif",dh=`#version 300 es
void main(){gl_Position=vec4(float((gl_VertexID<<1)&2)*2.0-1.0,float(gl_VertexID&2)*2.0-1.0,0.0,1.0);}`,ch=`#version 300 es
precision mediump float;uniform vec4 u_tint;out vec4 o_color;void main(){o_color=u_tint;}`;function ea(a){let n=a.trim().toLowerCase();if(n.startsWith("#")){let s=n.slice(1),l=s.length<=4?[...s].map(d=>d+d).join(""):s,u=d=>parseInt(l.slice(d,d+2),16)/255;return[u(0),u(2),u(4),l.length>=8?u(6):1]}let t=(n.match(/[\d.]+%?/g)??[]).map(s=>s.endsWith("%")?parseFloat(s)/100:parseFloat(s)),[e=0,r=0,i=0,o=1]=t;return[e/255,r/255,i/255,o]}function jo(a){return Math.min($o,Math.max(0,Math.ceil(Math.log2(1/Math.max(a[3],2**-$o)))))}function Xo(a,n,t){return e=>{let r=e.createProgram();for(let[s,l]of[[e.VERTEX_SHADER,dh],[e.FRAGMENT_SHADER,ch]]){let u=e.createShader(s);e.shaderSource(u,l),e.compileShader(u),e.attachShader(r,u),e.deleteShader(u)}e.linkProgram(r);let i=e.getUniformLocation(r,"u_tint"),o=e.createVertexArray();return{render(s){if(t()){e.bindFramebuffer(e.FRAMEBUFFER,null),e.viewport(0,0,s.width*s.pixelRatio,s.height*s.pixelRatio),e.enable(e.BLEND),e.useProgram(r),e.bindVertexArray(o),e.blendFuncSeparate(e.ZERO,e.ONE,e.DST_ALPHA,e.ONE),e.uniform4f(i,0,0,0,1);for(let l=0;l<n;l++)e.drawArrays(e.TRIANGLES,0,3);e.blendFuncSeparate(e.DST_ALPHA,e.ZERO,e.DST_ALPHA,e.ZERO),e.uniform4f(i,a[0]*a[3],a[1]*a[3],a[2]*a[3],a[3]),e.drawArrays(e.TRIANGLES,0,3),e.bindVertexArray(null),e.blendFunc(e.ONE,e.ONE_MINUS_SRC_ALPHA)}},kill(){e.deleteProgram(r),e.deleteVertexArray(o)}}}}function er(a){return Math.max(Vo,Math.min(1,a))**sh}function qo(a){return{labelColor:a.label,labelFont:uh,labelSize:Ja}}function Ko(){return{labelRenderedSizeThreshold:Jn,labelDensity:.35,labelGridCellSize:110,minEdgeThickness:.8,minCameraRatio:Vo,maxCameraRatio:lh,zoomToSizeRatioFunction:er}}function hh(a){let n=[Tt.nodes,{...qo(a),label:{attribute:"tag"},cursor:"pointer",depth:"nodes"},{whenState:"named",then:{labelVisibility:"visible"}},{whenState:"mute",then:{labelVisibility:"hidden"}},{whenState:"off",then:{visibility:"hidden"}}],t=[Tt.edges,{color:{attribute:"color",defaultValue:a.edge},size:oh,depth:"edges"},{whenState:"off",then:{visibility:"hidden"}}];return{nodes:n,edges:t}}function fh(a){let n=[Tt.nodes,{...qo(a),label:{attribute:"tag"},backdropColor:a.back,backdropBorderColor:a.backLine,backdropBorderWidth:1,backdropCornerRadius:6,backdropShadowColor:a.shadow,depth:"nodes"},{whenData:"plate",then:{labelVisibility:"visible",backdropVisibility:"visible"}},{whenData:"pin",then:{depth:"pin"}},{whenState:"hush",then:{labelVisibility:"hidden",backdropVisibility:"hidden"}}],t=[Tt.edges,{color:a.edgeOn,size:ih,depth:"edges"}];return{nodes:n,edges:t}}function gh(a){return{primitives:{depthLayers:["edges","nodes","veil"]},customNodeState:{...Br},customEdgeState:{...Wr},styles:hh(a),settings:Ko()}}function ph(a){return{primitives:{depthLayers:["edges","nodes","pin"]},customNodeState:{...Ur},styles:fh(a),settings:Ko()}}function Yo(a,n,t){return new Qa(a,n,gh(t))}function tr(a,n,t){return new Qa(a,n,ph(t))}var Ze=a=>getComputedStyle(document.documentElement).getPropertyValue(a).trim(),vh=10,mh=14,bh=200,yh=2e3,xh=120,_h=160,Zo=140,Th=400,Qo=12,Sh=.4,Eh=24,Ah=350;async function Qf(a,n){let t={label:Ze("--g-label"),dim:Ze("--g-dim"),edge:Ze("--g-edge"),edgeOn:Ze("--g-edge-on"),back:Ze("--g-back"),backLine:Ze("--g-back-line"),shadow:Ze("--g-shadow")};Nt(a,Q("div",{class:"empty",style:"margin:24px"},se.graph.loading));let e=await rr("graph",...n?[n]:[]),{g:r,colors:i,names:o}=Ir(e,Ze("--g-other"));Fr(r,Zr(r),i,ea(t.edge),Sh),Pr();let s=Q("div",{class:"canvas"}),l=Q("div",{class:"canvas",style:"pointer-events:none"}),u=Q("span",{class:"info"}),d=Q("aside",{}),c=new Set,h=!1,f=r.nodes(),g=new Set([...f].sort((U,K)=>r.getNodeAttribute(K,"deg")-r.getNodeAttribute(U,"deg")).slice(0,mh)),m=new Set(g);r.forEachNode((U,K)=>{K.size>=Jn&&m.add(U)});let v=[...m];for(let U of v)r.setNodeAttribute(U,"tag",Wt(r.getNodeAttribute(U,"label")));let p=null,b=null,y=r.mapNodes((U,K)=>K.t).filter(U=>U>0),[T,x]=y.length?[Math.min(...y),Math.max(...y)]:[0,0],_=x,A=U=>!c.has(U.kind)&&(h||U.deg>0)&&(U.t===0||U.t<=_),E=!1,L=0,w=null,F=!1,D=0,I=0,R=!1,P=()=>{u.textContent=se.graph.nodes(r.order-Pe.gone.size,r.order,r.size-Pe.cut.size)},G="",N=U=>{let{width:K,height:de}=ne.getDimensions();return Mr(U,Ja,K,de)},k=()=>{let U=ne.getCamera().ratio,K=[...g].filter(Z=>!Pe.gone.has(Z)).map(Z=>{let ce=r.getNodeAttributes(Z),nr=ne.graphToViewport(ce);return{n:Z,x:nr.x,y:nr.y,r:ne.scaleSize(ce.size,U),text:ce.tag}}),de=new Set(N(K)),oe=[...de].join("\0");oe!==G&&(G=oe,ne.setNodesState([...g].filter(Z=>!de.has(Z)),{named:!1}),ne.setNodesState([...de],{named:!0}))},C=()=>{k(),$();let{width:U,height:K}=ne.getDimensions(),de=Gr([[0,0],[U,0],[0,K],[U,K]].map(([Z,ce])=>ne.viewportToGraph({x:Z,y:ce}))),oe=zr(r,de,Jn*er(ne.getCamera().ratio),Pe.gone,Th);if(oe.length){for(let Z of oe){let ce=r.getNodeAttributes(Z);ce.tag=Wt(ce.label),m.add(Z),v.push(Z)}ne.refresh({partialGraph:{nodes:oe},schedule:!0}),E&&ne.setNodesState(oe,{mute:!0})}},M=U=>{clearTimeout(D),U&&(D=window.setTimeout(()=>M(!1),_h)),!(U===F||!w||U&&Ne.order<=xh)&&(F=U,w.setNodesState(Ne.nodes(),{hush:U}))},$=()=>{if(!E)return;let U=ne.getCamera().ratio,K=Ge().map(oe=>{let Z=Ne.getNodeAttributes(oe),ce=ne.graphToViewport(Z);return{n:oe,x:ce.x,y:ce.y,r:ne.scaleSize(Z.size,U),text:Z.tag}}),de=new Set(N(K).slice(0,Eh));Ne.forEachNode((oe,Z)=>{let ce=Z.pin||de.has(oe);Z.plate!==ce&&Ne.setNodeAttribute(oe,"plate",ce)})},H=()=>{let U=na(p??b,p===null?null:b)>0;U!==E&&ne.setNodesState(v,{mute:U}),E=U,F=!1,U&&(w??=tr(Ne,l,t),w.setCustomBBox(ne.getBBox()),w.getCamera().setState(ne.getCamera().getState()),$()),ne.scheduleRender()},Y=Vr(()=>{let U=Pe.sift(A);U.length&&(P(),C(),E&&H(),U.some(K=>m.has(K))&&(clearTimeout(L),L=window.setTimeout(()=>ne.refresh({partialGraph:{nodes:[]},schedule:!0}),bh)))}),j=U=>Q("span",{class:"swatch",style:`background:${r.getNodeAttribute(U,"color")}`}),q=U=>Q("a",{href:"#",onclick:K=>{K.preventDefault(),xe(U)}},j(U),r.getNodeAttribute(U,"label")),be=U=>{let K=r.getNodeAttributes(U);K.kind==="note"?aa(`#/n/${U.slice(1)}`):K.kind==="project"&&aa(`#/p/${or(K.group)}`)},ve=Q("div",{class:"legend"},kr.map(U=>Q("label",{},Q("input",{type:"checkbox",checked:!0,onchange:K=>{K.target.checked?c.delete(U):c.add(U),Y()}}),se.graph.kinds[U])),Q("label",{},Q("input",{type:"checkbox",checked:h,onchange:U=>{h=U.target.checked,Y()}}),se.graph.orphans)),ye=()=>{let U=r.filterNodes((Z,ce)=>ce.kind==="note").sort((Z,ce)=>r.getNodeAttribute(ce,"deg")-r.getNodeAttribute(Z,"deg")).slice(0,vh),K=(R?It:It.slice(0,Qo)).map(Z=>Q("a",{href:"#",onclick:ce=>{ce.preventDefault(),xe(Z.hub)}},Q("span",{class:"swatch",style:`background:${Z.color}`}),Z.name,Q("span",{class:"faint"},` \xB7 ${Z.count.toLocaleString("en-US")}`))),de=It.length-Qo,oe=de>0?Q("a",{href:"#",class:"more",onclick:Z=>{Z.preventDefault(),R=!R,ye()}},R?se.graph.fewer:se.graph.more(de)):null;Nt(d,Q("h2",{style:"margin-top:0"},se.graph.show),ve,K.length?Q("h2",{},se.graph.groups):null,...K,oe,Q("h2",{},se.graph.hubs),...U.map(q))},Oe=U=>{let K=r.getNodeAttributes(U),de=r.neighbors(U).sort((oe,Z)=>r.getNodeAttribute(Z,"deg")-r.getNodeAttribute(oe,"deg"));Nt(d,Q("div",{class:"detail"},Q("button",{class:"back",onclick:()=>xe(null)},`\u2190 ${se.graph.back}`),Q("h3",{},K.label),Q("div",{class:"meta"},[se.graph.kinds[K.kind],K.t?se.ago(ir(K.t)):"",i.has(K.cluster)?o.get(K.cluster):""].filter(Boolean).join(" \xB7 ")),Q("div",{class:"actions"},K.kind==="session"?null:Q("button",{class:"primary",onclick:()=>be(U)},se.graph.open),Q("button",{onclick:()=>He(U)},se.graph.center)),Q("h2",{},se.graph.connections,Q("span",{class:"n"},de.length)),de.length?de.map(q):Q("div",{class:"faint"},se.graph.alone)))},He=U=>{let K=ne.getNodeDisplayData(U);if(!K)return;let de=r.filterNeighbors(U,oe=>!Pe.gone.has(oe)).map(oe=>{let Z=ne.getNodeDisplayData(oe);return Z?Math.hypot(Z.x-K.x,Z.y-K.y):0});ne.getCamera().animate({x:K.x,y:K.y,ratio:Hr(de)},{duration:Ah})},xe=U=>{p=U,H(),U?(Oe(U),He(U)):ye()},Qe=Q("input",{type:"search",class:"find",placeholder:se.graph.find,onkeydown:U=>{if(U.key!=="Enter")return;let K=U.target.value.trim().toLowerCase();if(!K)return xe(null);let de=r.filterNodes((oe,Z)=>Z.label.toLowerCase().includes(K));de.sort((oe,Z)=>+(r.getNodeAttribute(Z,"kind")==="project")-+(r.getNodeAttribute(oe,"kind")==="project")||r.getNodeAttribute(Z,"deg")-r.getNodeAttribute(oe,"deg")),de.length&&xe(de[0])}}),ln=Q("span",{},se.graph.all),ta=Q("input",{type:"range",min:0,max:1e3,value:1e3,"aria-label":se.graph.until,oninput:U=>{let K=Number(U.target.value)/1e3;_=T+(x-T)*K,ln.textContent=K>=1?se.graph.all:`${se.graph.until} ${new Date(_*1e3).toLocaleDateString("en-US",{year:"numeric",month:"short",day:"numeric"})}`,Y()}});Nt(a,Q("div",{class:"graph"},Q("div",{class:"stage"},s,l,Q("div",{class:"tools"},Qe,u),Q("div",{class:"hint-box"},se.graph.focus),x>T?Q("div",{class:"time"},ta,ln):null),d));let It=Nr(r,i,o);ye();let ne=Yo(r,s,t),Pe=$r(r,ne),{top:Ne,spot:na,ranked:Ge}=Or(r,Pe.gone);ne.addCustomLayerProgram("veil","veil",Xo(ea(t.dim),jo(ea(t.edge)),()=>E)),Pe.sift(A),k(),P(),I=window.setTimeout(C,Zo),ne.getCamera().on("updated",U=>{clearTimeout(I),I=window.setTimeout(C,Zo),E&&(w?.getCamera().setState(U),M(!0))});let Jo=jr(()=>w??=tr(Ne,l,t),yh);return ne.on("enterNode",({node:U})=>{b=U,H()}),ne.on("leaveNode",()=>{b=null,H()}),ne.on("clickNode",({node:U})=>xe(U===p?null:U)),ne.on("clickStage",()=>xe(null)),ne.on("doubleClickNode",({node:U})=>be(U)),()=>{clearTimeout(L),clearTimeout(D),clearTimeout(I),Jo(),w?.kill(),ne.kill()}}export{Qf as graph};
