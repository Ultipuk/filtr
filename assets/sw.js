const CACHE_NAME = 'filtr-app-v0.1.2';
const PRECACHE_FILES = [
  './',
  './index.html',
];

/* Start the service worker and cache stable shell files */
self.addEventListener('install', function (event) {
  event.waitUntil(
    caches.open(CACHE_NAME).then(function (cache) {
      return cache.addAll(PRECACHE_FILES);
    })
  );
  self.skipWaiting();
});

/* Remove old caches from previous versions */
self.addEventListener('activate', function (event) {
  event.waitUntil(
    caches.keys().then(function (keys) {
      return Promise.all(
        keys
          .filter(function (key) { return key !== CACHE_NAME; })
          .map(function (key) { return caches.delete(key); })
      );
    })
  );
  self.clients.claim();
});

/* Avoid stale JS/WASM (prevents SRI mismatch after deploy) */
self.addEventListener('fetch', function (event) {
  const req = event.request;
  const url = new URL(req.url);
  const isCodeAsset = url.pathname.endsWith('.js') || url.pathname.endsWith('.wasm');

  if (isCodeAsset) {
    event.respondWith(
      fetch(req).catch(function () {
        return caches.match(req);
      })
    );
    return;
  }

  event.respondWith(
    caches.match(req).then(function (cached) {
      if (cached) {
        return cached;
      }
      return fetch(req).then(function (networkResponse) {
        if (req.method === 'GET' && req.url.startsWith(self.location.origin)) {
          const responseClone = networkResponse.clone();
          caches.open(CACHE_NAME).then(function (cache) {
            cache.put(req, responseClone);
          });
        }
        return networkResponse;
      });
    })
  );
});
