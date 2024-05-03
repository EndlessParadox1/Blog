FROM nginx:latest

COPY ./assets/ /usr/share/nginx/html/

RUN rm -rf /etc/nginx/conf.d

CMD nginx -g "daemon off;"
