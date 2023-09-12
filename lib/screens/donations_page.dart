import 'package:flutter/material.dart';

class DonationPage extends StatefulWidget {
  final PageController controller;
  final bool isDonor;
  const DonationPage(
      {super.key, required this.controller, required this.isDonor});

  @override
  State<DonationPage> createState() => _DonationPageState();
}

class _DonationPageState extends State<DonationPage> {
  @override
  Widget build(BuildContext context) {
    final size = MediaQuery.of(context).size;
    return Center(
        child: Column(crossAxisAlignment: CrossAxisAlignment.center, children: [
      SizedBox(
        height: size.height * 0.075,
      ),
      Row(
        mainAxisAlignment: MainAxisAlignment.spaceAround,
        children: [
          GestureDetector(
              onTap: () {
                FocusScope.of(context).unfocus();
                widget.controller.previousPage(
                    duration: const Duration(milliseconds: 500),
                    curve: Curves.easeOutBack);
              },
              child: Icon(Icons.arrow_back_ios_rounded,
                  size: size.width * 0.075, color: Colors.black)),
          SizedBox(
            width: size.width * 0.075,
          ),
          Text(
            'Donations',
            style: TextStyle(
                fontSize: size.width * 0.125, height: 0, color: Colors.black),
          ),
          SizedBox(
            width: size.width * 0.075,
          ),
          SizedBox(
            width: size.width * 0.075,
          )
        ],
      ),
      SizedBox(
        height: size.height * 0.05,
      ),
    ]));
  }
}
