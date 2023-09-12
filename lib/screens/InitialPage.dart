import 'package:flutter/material.dart';
import 'package:lottie/lottie.dart';

class Initial extends StatefulWidget {
  final PageController controller;
  final Function setDonor;
  const Initial({super.key, required this.controller, required this.setDonor});

  @override
  State<Initial> createState() => _InitialState();
}

class _InitialState extends State<Initial> {
  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    return Center(
        child: Column(crossAxisAlignment: CrossAxisAlignment.center, children: [
      SizedBox(
        height: size.height * 0.1,
      ),
      Text(
        "Let's Donate",
        style: TextStyle(fontSize: size.width * 0.12, color: Colors.black),
      ),
      Lottie.asset("assets/anims/running.json"),
      SizedBox(
        height: size.height * 0.05,
      ),
      Text(
        "Are You A...",
        style: TextStyle(fontSize: size.width * 0.05),
      ),
      SizedBox(
        height: size.height * 0.075,
      ),
      Row(
        mainAxisAlignment: MainAxisAlignment.spaceAround,
        children: [
          GestureDetector(
            onTap: (){
              widget.setDonor(true);
              widget.controller.nextPage(duration: const Duration(milliseconds: 500), curve: Curves.easeOutBack);

            },
            child: Container(
              width: size.width * 0.4,
              height: size.height * 0.075,
              decoration: BoxDecoration(
                  color: Colors.green,
                  borderRadius: BorderRadius.circular(size.width * 0.25)),
              child: Center(
                child: Text("Donor",
                style: TextStyle(
                  fontSize: size.width*0.05,
                  color: Colors.white
                ),),
              ),
            ),
          ),

          GestureDetector(
            onTap: (){
              widget.setDonor(false);
              widget.controller.nextPage(duration: const Duration(milliseconds: 500), curve: Curves.easeOutBack);
            },
            child: Container(
              width: size.width * 0.4,
              height: size.height * 0.075,
              decoration: BoxDecoration(
                  color: Colors.blue,
                  borderRadius: BorderRadius.circular(size.width * 0.25)),
              child: Center(
                child: Text("Runner",
                  style: TextStyle(
                      fontSize: size.width*0.05,
                    color: Colors.white
                  ),),

              ),
            ),
          ),
        ],
      )
    ]));
  }
}
