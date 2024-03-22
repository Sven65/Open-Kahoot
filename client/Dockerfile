FROM --platform="linux/amd64" node:20 as builder

WORKDIR /usr/src/app

COPY tsconfig.json package*.json ./
COPY vite.config.ts ./
COPY src ./src
COPY index.html ./
COPY public ./public

RUN npm install --legacy-peer-deps
RUN npm run build

# Create nginx image

FROM nginx:alpine

COPY --from=builder /usr/src/app/dist/ /usr/share/nginx/html
COPY --from=builder /usr/src/app/public/ /usr/share/nginx/html/public
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]