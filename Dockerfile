# Uses Rust latest version as our base image
FROM rust:latest

# Goes to the app directory (Like in Terminal)
WORKDIR /home/Ferro-Maurus/.

# Copy the app into the container
COPY . .

# Install dependencies
RUN cargo install --path .

# Set port enviornment variable
ENV PORT=9000
# Expose the port so our computer can access it
EXPOSE 9000

#run the app
CMD ["cargo", "run"]