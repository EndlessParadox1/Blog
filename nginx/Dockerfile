FROM nginx:latest

COPY ./nginx.conf /etc/nginx/nginx.conf
COPY ./static/ /usr/share/nginx/html/

EXPOSE 443

CMD ["nginx", "-g", "daemon on;"]
